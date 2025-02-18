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
    pub id: i64,
    _id: PhantomData<Id>,
    pub email: String,
    _email: PhantomData<Email>,
    pub nickname: String,
    _nickname: PhantomData<Nickname>,
    pub password_hash: String,
    _password_hash: PhantomData<PasswordHash>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl User {
    pub fn new(id: i64, email: String, nickname: String, password_hash: String, created_at: i64) -> Self {
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
