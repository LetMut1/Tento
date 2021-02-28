#[derive(Debug)]
pub enum ApplicationUserErrorKind {
    AlreadyExist(Option<String>),
    NotConfirmed(Option<String>),
    WrongPassword(Option<String>)
}