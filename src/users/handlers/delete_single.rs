use actix_web::{
    web,
    HttpResponse,
    Responder
};
use serde_json::json;
use crate::AppState;

pub async fn delete_user(
    path: web::Path<String>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let username = path.into_inner();

    let pool = &app_state.pool;

    let query = sqlx::query("DELETE FROM users WHERE username = ?")
        .bind(&username)
        .execute(pool)
        .await;

    match query {
        Ok(res) => {
            if res.rows_affected() >= 1 {
                HttpResponse::Ok()
                    .json(json!({
                        "status": "success",
                        "msg": format!("user {} successfully deleted", &username),
                    }))
            } else {
                HttpResponse::BadRequest() 
                    .json(json!({
                        "status": "failed",
                        "msg": format!("user {} cannot deleted", &username)
                    }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "msg": "failed delete user",
                    "details": error.to_string()
                }))
        }
    }
}