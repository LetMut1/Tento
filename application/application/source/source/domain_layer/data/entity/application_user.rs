use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;

pub use self::CreatedAt as ApplicationUser_CreatedAt;
pub use self::Email as ApplicationUser_Email;
pub use self::Id as ApplicationUser_Id;
pub use self::Nickname as ApplicationUser_Nickname;
pub use self::Password as ApplicationUser_Password;
pub use self::PasswordHash as ApplicationUser_PasswordHash;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Id(i64);

impl Id {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Clone, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }

    pub fn into_inner(self) -> String {
        return self.0;
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Clone, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Nickname(String);

impl Nickname {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Password(String);

impl Password {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[derive(Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

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
