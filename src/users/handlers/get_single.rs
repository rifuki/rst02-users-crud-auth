use actix_web::{
    web,
    HttpResponse,
    Responder
};
use serde_json::json;
use crate::{
    AppState,
    users::model::User
};

pub async fn get_user(
    path: web::Path<String>, 
    app_state: web::Data<AppState>
) -> impl Responder {
    let username = path.into_inner();

    let pool = &app_state.pool;

    let query = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = ?",
        &username
    )
        .fetch_optional(pool)
        .await;

    match query {
        Ok(res) => match res {
            Some(user) => {
                HttpResponse::Ok()
                    .json(json!({
                        "status": "success",
                        "data": user
                    }))
            }
            None => {
                HttpResponse::NotFound()
                    .json(json!({
                        "status": "failed",
                        "msg": format!("user {} not found", &username)
                    }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "msg": format!("failed to get user {}", &username),
                    "details": error.to_string()
                }))
        }
    }
}