use actix_web::web;

use crate::users::handlers::{
    get_all_users,
    get_user,
    update_user,
    delete_user,
    register_user,
    login_user
};


pub fn scoped_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_all_users))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .service(
                web::resource("/{username}")
                    .get(get_user)
                    .put(update_user)
                    .delete(delete_user)
            )
    );
}