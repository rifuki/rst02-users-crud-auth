mod get_all;
mod get_single;
mod update_single;
mod delete_single;
mod register;
mod login;

pub use get_all::get_all_users;
pub use get_single::get_user;
pub use update_single::update_user;
pub use delete_single::delete_user;
pub use register::register_user;
pub use login::login_user;