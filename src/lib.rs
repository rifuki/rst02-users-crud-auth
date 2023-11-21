pub mod users;

use actix_web::HttpResponse;
use argon2::{
    PasswordHasher,
    PasswordVerifier
};
use sqlx::mysql::{
    MySqlPool,
    MySqlPoolOptions
};
use validator::ValidationErrors;
use serde_json::{
    json,
    Map as JsonMap,
    Value as JsonValue
};
use crate::users::model::User;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool
}

pub async fn estalish_connection(db_url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(15)
        .connect(db_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("can't create mysqlpool connections [{}]", err);
            std::process::exit(1);
        })
}

pub fn validate_error_response(
    validation_errors: 
    &ValidationErrors
) -> HttpResponse {
    let mut cleaned_errors = JsonMap::new();

    for (field, field_errors) in validation_errors.field_errors().iter() {
        let mut cleaned_field_errors = Vec::new();

        for error in field_errors.iter() {
            let cleaned_error = json!({
                "code": error.code,
                "msg": error.message
            });

            cleaned_field_errors.push(cleaned_error);
        }

        cleaned_errors.insert(
            field.to_string(),
            JsonValue::Array(cleaned_field_errors)
        );
    }

    let error_response = json!({
        "status": "failed",
        "msg": "validation failed",
        "details": cleaned_errors
    });

    HttpResponse::BadRequest()
        .json(error_response)
}


pub async fn check_username(username: &str, pool: &MySqlPool) -> Option<HttpResponse> {
    let query = sqlx::query(
        "SELECT username from users WHERE username = ?"
    )
        .bind(username)
        .fetch_all(pool)
        .await;

    match query {
        Ok(res) => {
            if res.len() >= 1 {
                Some(
                    HttpResponse::Conflict()
                        .json(json!({
                            "msg": format!("username {} is already taken", username)
                        }))
                )
            } else {
                None
            }
        },
        Err(error) => {
            Some(
                HttpResponse::InternalServerError()
                    .json(json!({
                        "status": "failed",
                        "msg": "failed checking username",
                        "details": error.to_string()
                    }))
                )
        }
    }
}

pub async fn check_email(email: &str, username: &str, pool: &MySqlPool) -> Option<HttpResponse> {
    let query = sqlx::query_as::<_, User>(
        "SELECT * from users WHERE email = ?"
    )
        .bind(email)
        .fetch_all(pool)
        .await;

    match query {
        Ok(res) => {
            if res.len() >= 1 && res[0].username != username  {
                Some(
                    HttpResponse::Conflict()
                        .json(json!({
                            "msg": format!("email {} is already binded to another user", email)
                        }))
                )
            } else {
                None
            }
        },
        Err(error) => {
            Some(
                HttpResponse::InternalServerError()
                    .json(json!({
                        "status": "failed",
                        "msg": "failed checking email",
                        "details": error.to_string()
                    }))
                )
        }
    }
}

pub fn hash_password(password: &[u8]) -> Result<String, HttpResponse> {
    let salt = argon2::password_hash::SaltString::generate(
        argon2::password_hash::rand_core::OsRng
    );
    let argon2 = argon2::Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)
        .map_err(|err| HttpResponse::InternalServerError()
            .json(
                json!({
                    "status": "failed",
                    "msg": "error hashing password",
                    "details": err.to_string()
                })
            )
        )?;

    Ok(password_hash.to_string())
}

pub fn verify_password(payload_password: &[u8], stored_hash_password: String) -> Result<bool, HttpResponse> {
    let parsed_stored_hash_password = argon2::password_hash::PasswordHash::new(&stored_hash_password)
        .map_err(|err| {
            HttpResponse::InternalServerError()
            .json(
                json!({
                    "status": "failed",
                    "msg": "failed to parase stored hash password",
                    "details": err.to_string()
                })
            )
        });

    let argon2 = argon2::Argon2::default();

    let password_verify = argon2
        .verify_password(
            payload_password, 
            &parsed_stored_hash_password.unwrap()
        )
        .is_ok();

    Ok(password_verify)
}