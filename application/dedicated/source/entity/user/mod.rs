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
    id: PhantomData<(i64, Id)>,
    email: PhantomData<(String, Email)>,
    nickname: PhantomData<(String, Nickname)>,
    password_hash: PhantomData<(String, PasswordHash)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}
pub type User_Email = Email;
pub type User_Id = Id;
pub type User_Nickname = Nickname;
pub type User_Password = Password;
