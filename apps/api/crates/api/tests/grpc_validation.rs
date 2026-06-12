use api::grpc::user::GetUserWithPasswordByEmailRequest;
use prost_validate::Validator;

#[test]
fn get_user_with_password_by_email_accepts_valid_email() {
    let request = GetUserWithPasswordByEmailRequest {
        email: "ash@example.com".to_string(),
    };

    assert!(request.validate().is_ok());
}

#[test]
fn get_user_with_password_by_email_rejects_invalid_email() {
    let request = GetUserWithPasswordByEmailRequest {
        email: "not-an-email".to_string(),
    };

    assert!(request.validate().is_err());
}
