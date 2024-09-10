pub mod derivative;
pub use self::{
    Email as ApplicationUser_Email,
    Id as ApplicationUser_Id,
    Nickname as ApplicationUser_Nickname,
    Password as ApplicationUser_Password,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
pub struct ApplicationUser<'a> {
    pub id: i64,
    _id: PhantomData<Id>,

    pub email: String,
    _email: PhantomData<Email>,

    pub nickname: Cow<'a, str>,
    _nickname: PhantomData<Nickname>,

    pub password_hash: String,
    _password_hash: PhantomData<PasswordHash>,

    pub created_at: String,
    _created_at: PhantomData<CreatedAt>,

    _password: PhantomData<Password>,
}
impl<'a> ApplicationUser<'a> {
    pub fn new(id: i64, email: String, nickname: Cow<'a, str>, password_hash: String, created_at: String) -> Self {
        return Self {
            id,
            _id: PhantomData,
            email,
            _email: PhantomData,
            nickname,
            _nickname: PhantomData,
            password_hash,
            _password_hash: PhantomData,
            created_at,
            _created_at: PhantomData,
            _password: PhantomData,
        };
    }
}
pub struct Id;
pub struct Email;
impl Email {
    pub const MAXIMUM_LENGTH: usize = 320;
    pub const REGULAR_EXPRESSION: &'static str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
}
pub struct Nickname;
impl Nickname {
    pub const MAXIMUM_LENGTH: usize = 55;
}
pub struct Password;
impl Password {
    pub const MAXIMUM_LENGTH: usize = 65;
    pub const MINIMUM_LENGTH: usize = 7; // TODO Нужна ли максимальная длина? // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
}
pub struct PasswordHash;
pub struct CreatedAt;
