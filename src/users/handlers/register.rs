use actix_web::{
    web,
    HttpResponse,
    Responder
};
use serde_json::json;
use validator::Validate;
use crate::{
    AppState,
    users::model::{
        In,
        UserRegister
    },
    validate_error_response,
    check_username,
    check_email,
    hash_password
};

pub async fn register_user(
    payload: web::Json<In<UserRegister>>, 
    app_state: web::Data<AppState>
) -> impl Responder {
    let pool = &app_state.pool;

    let payload = payload.into_inner().user;

    if let Err(validation_errros) = payload.validate() {
        return validate_error_response(&validation_errros);
    }

    if let Some(response) = check_username(&payload.username, &pool).await {
        return response;
    }

    if let Some(response) = check_email(&payload.email, &payload.username, pool).await {
        return response;
    }

    let password_hashed = hash_password(&payload.password.as_bytes()).unwrap();

    let query = sqlx::query("INSERT INTO users (username, email, password) VALUES (?, ?, ?);")
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(password_hashed)
        .execute(pool)
        .await;

    match query {
        Ok(res) => {
            if res.rows_affected() >= 1 {
                HttpResponse::Created()
                    .json(json!({
                        "status": "success",
                        "msg": format!("user {} successfully created", &payload.username),
                    }))
            } else {
                HttpResponse::BadRequest() 
                    .json(json!({
                        "status": "failed",
                        "msg": format!("user {} cannot created", &payload.username)
                    }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "msg": "failed create user",
                    "details": error.to_string()
                }))
        }
    }
}