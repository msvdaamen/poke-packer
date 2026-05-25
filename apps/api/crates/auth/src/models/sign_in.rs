pub struct SignInDto {
    pub email: String,
    pub password: String,
}

pub struct SignInResult {
    pub access_token: String,
    pub refresh_token: String,
}

pub enum SignInError {
    UserNotFound,
    IncorrectPassword,
    InternalServerError,
}
