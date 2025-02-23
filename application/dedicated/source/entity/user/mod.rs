mod field;
use {
    self::field::{
        CreatedAt,
        Email,
        Id,
        Nickname,
        Password,
        PasswordHash,
    },
    std::marker::PhantomData,
};
pub struct User {
    id: i64,
    _id: PhantomData<Id>,
    email: String,
    _email: PhantomData<Email>,
    nickname: String,
    _nickname: PhantomData<Nickname>,
    password_hash: String,
    _password_hash: PhantomData<PasswordHash>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
pub type User_Email = Email;
pub type User_Id = Id;
pub type User_Nickname = Nickname;
pub type User_Password = Password;
