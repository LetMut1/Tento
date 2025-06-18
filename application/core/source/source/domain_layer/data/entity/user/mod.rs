mod field;
use self::field::{
    CreatedAt,
    Email,
    Id,
    Nickname,
    ObfuscatedId,
    Password,
    PasswordHash,
};
pub struct User {
    id: Id,
    obfuscated_id: ObfuscatedId,
    email: Email,
    nickname: Nickname,
    password_hash: PasswordHash,
    created_at: CreatedAt,
}
pub type User_Id = Id;
pub type User_ObfuscatedId = ObfuscatedId;
pub type User_Email = Email;
pub type User_Nickname = Nickname;
pub type User_Password = Password;
