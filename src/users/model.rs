use sqlx::prelude::FromRow;

lazy_static::lazy_static! {
    static ref RE_USERNAME: regex::Regex = regex::Regex::new(r"^[0-9a-zA-Z]{5,}$").unwrap_or_else(|err| {
        eprintln!("failed creating regex pattern matching [{}]", err);
        std::process::exit(1);
    });
}

#[derive(serde::Serialize, FromRow)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(serde::Deserialize)]
pub struct In<T> {
    pub user: T
}

#[derive(serde::Deserialize, validator::Validate)]
pub struct UserUpdate {
    #[validate(
        length(
            min = 3,
            max = 25,
            message = "fails validation - must be 3-25 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(
        email(
            message = "fails validation - is not a valid email address"
        )
    )]
    pub email: String,
    #[validate(
        length(
            min = 8,
            max = 75,
            message = "fails validation - must be 8-75 characters long"
        ) 
    )]
    pub password: String,
}

#[derive(serde::Deserialize, validator::Validate)]
pub struct UserRegister {
    #[validate(
        length(
            min = 3,
            max = 25,
            message = "fails validation - must be 3-25 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(
        email(
            message = "fails validation - is not a valid email address"
        )
    )]
    pub email: String,
    #[validate(
        length(
            min = 8,
            max = 75,
            message = "fails validation - must be 8-75 characters long"
        )
    )]
    pub password: String
}

#[derive(serde::Deserialize, validator::Validate)]
pub struct UserLogin {
    #[validate(
        length(
            min = 3,
            max = 25,
            message = "fails validation - must be 3-25 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(
        length(
            min = 8,
            max = 75,
            message = "fails validation - must be 8-75 characters long"
        )
    )]
    pub password: String
}