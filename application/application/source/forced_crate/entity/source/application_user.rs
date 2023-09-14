use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;

pub use self::CreatedAt as ApplicationUser_CreatedAt;
pub use self::Email as ApplicationUser_Email;
pub use self::Id as ApplicationUser_Id;
pub use self::Nickname as ApplicationUser_Nickname;
pub use self::Password as ApplicationUser_Password;
pub use self::PasswordHash as ApplicationUser_PasswordHash;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub i64);

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct Email(pub String);

impl Email {
    pub const REGULAR_EXPRESSION: &'static str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
    pub const MAXIMUM_LENGTH: usize = 320;
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct Nickname(pub String);

impl Nickname {
    pub const MAXIMUM_LENGTH: usize = 55;
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Password(pub String);

impl Password {
    pub const MINIMUM_LENGTH: usize = 7;
    pub const MAXIMUM_LENGTH: usize = 65; // TODO Нужна ли максимальная длина? // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
}

#[derive(Clone)]
pub struct PasswordHash(pub String);

pub struct CreatedAt(pub String);

pub struct ApplicationUser<'a> {
    pub id: Id,
    pub email: Email,
    pub nickname: Cow<'a, Nickname>,
    pub _password: PhantomData<Password>,
    pub password_hash: PasswordHash,
    pub created_at: CreatedAt,
}

pub struct ApplicationUser1 {
    pub id: Id,
    pub email: Email,
    pub password_hash: PasswordHash,
}

pub struct ApplicationUser2 {
    pub id: Id,
    pub nickname: Nickname,
    pub password_hash: PasswordHash,
}

pub struct ApplicationUser3 {
    pub id: Id,
}

pub struct ApplicationUser4 {
    pub email: Email,
    pub nickname: Nickname,
    pub password_hash: PasswordHash,
}

pub struct ApplicationUser5 {
    pub email: Email,
}
