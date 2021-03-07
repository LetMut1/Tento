#[derive(Debug)]
pub enum ApplicationUserErrorKind {
    AlreadyExist,
    InvalidEmail,
    NotConfirmed,
    WrongPassword
}