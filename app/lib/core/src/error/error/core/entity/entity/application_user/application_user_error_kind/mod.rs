#[derive(Debug)]
pub enum ApplicationUserErrorKind {
    AlreadyExist(Option<String>)
}