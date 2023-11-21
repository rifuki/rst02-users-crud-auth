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
        UserUpdate
    },
    validate_error_response,
    check_username,
    check_email,
    hash_password
};

pub async fn update_user(
    path: web::Path<String>,
    payload: web::Json<In<UserUpdate>>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let username = path.into_inner();
    let payload = payload.into_inner().user;

    let pool = &app_state.pool;

    if let Err(validation_errors) = payload.validate() {
        return validate_error_response(&validation_errors);
    }

    if &username != &payload.username {
        if let Some(response) = check_username(&payload.username, pool).await {
            return response;
        }
    }
    
    if let Some(response) = check_email(&payload.email, &username, pool).await {
        return response;
    }
    
    let password_hash = hash_password(&payload.password.as_bytes()).unwrap();

    let query = sqlx::query("UPDATE users SET username = ?, email = ?, password = ? WHERE username = ?")
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&password_hash)
        .bind(&username)
        .execute(pool)
        .await;

    match query {
        Ok(res) => {
            if res.rows_affected() >= 1 {
                HttpResponse::Ok()
                    .json(json!({
                        "status": "success",
                        "msg": format!("user {} successfully updated", &username),
                    }))
            } else {
                HttpResponse::BadRequest() 
                    .json(json!({
                        "status": "failed",
                        "msg": format!("user {} cannot updated", &username)
                    }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "msg": "failed update user",
                    "details": error.to_string()
                }))
        }
    }
}