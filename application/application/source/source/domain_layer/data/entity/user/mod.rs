pub mod derivative;
mod field;
use std::{
    borrow::Cow,
    marker::PhantomData,
};
use self::field::{
    Id,
    Password,
    Email,
    Nickname,
    PasswordHash,
    CreatedAt,
};
pub struct User<'a> {
    pub id: i64,
    _id: PhantomData<Id>,
    pub email: String,
    _email: PhantomData<Email>,
    pub nickname: Cow<'a, str>,
    _nickname: PhantomData<Nickname>,
    pub password_hash: String,
    _password_hash: PhantomData<PasswordHash>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl<'a> User<'a> {
    pub fn new(id: i64, email: String, nickname: Cow<'a, str>, password_hash: String, created_at: i64) -> Self {
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
        };
    }
}
pub type User_Email = Email;
pub type User_Id = Id;
pub type User_Nickname = Nickname;
pub type User_Password = Password;