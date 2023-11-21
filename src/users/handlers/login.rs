use actix_web::{
    web,
    HttpResponse,
    Responder
};
use serde_json::json;
use sqlx::Row;
use validator::Validate;
use crate::{
    AppState,
    users::model::{
        In,
        UserLogin
    },
    validate_error_response,
    verify_password
};

pub async fn login_user(payload: web::Json<In<UserLogin>>, app_state: web::Data<AppState>) -> impl Responder {
    let payload = payload.into_inner().user;
    let pool = &app_state.pool;

    if let Err(validation_errors) = payload.validate() {
        return validate_error_response(&validation_errors);
    }

    let query = sqlx::query("SELECT password FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_one(pool)
        .await;

    match query {
        Ok(row) => {
            let stored_hash_password: String = row.get("password");

            let is_verified = verify_password(&payload.password.as_bytes(), stored_hash_password);

            match is_verified {
                Ok(res) => {
                    if res {
                        HttpResponse::Ok()
                            .json(
                                json!({
                                    "status": "success",
                                    "msg": "Authentication Successfully",
                                })
                            )
                    } else {
                        HttpResponse::Unauthorized()
                            .json(
                                json!({
                                    "status": "failed",
                                    "msg": "Authentication Failed",
                                })
                            )
                    }
                },
                Err(err_response) => {
                    return err_response;
                }
            } 

        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "msg": format!("user {} does not exist", &payload.username),
                })
            ) 
        }
    }
}