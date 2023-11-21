use actix_web::{
    HttpServer, 
    App, 
    web, 
    middleware
};

use rst02_users_crud_auth::{
    estalish_connection, 
    AppState, 
    users::routes::scoped_users
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db_url= std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set [{}]", err);
        std::process::exit(1);
    });

    let pool = estalish_connection(&db_url).await;

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::NormalizePath::trim()
            )
            .app_data(web::Data::new(app_state.clone()))
            .configure(scoped_users)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
