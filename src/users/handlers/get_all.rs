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

pub async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let pool = &app_state.pool;

    let query = sqlx::query_as!(
        User,
        "SELECT * FROM users",
    )
        .fetch_all(pool)
        .await;

    match query {
        Ok(users) => {
            HttpResponse::Ok().json(
                json!({
                    "status": "success",
                    "length": users.len(),
                    "data": users
                })
            )
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(
                json!({
                    "status": "failed",
                    "msg": "failed get users list",
                    "details": err.to_string()
                })
            )
        }
    }
}