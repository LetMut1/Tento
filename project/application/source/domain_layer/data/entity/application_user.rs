use crate::domain_layer::functionality::service::getter::Getter;
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

#[derive(
    Clone,
    Copy,
    Serialize,
    Deserialize
)]
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
#[derive(
    Clone,
    Deserialize
)]
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
#[derive(
    Clone,
    Deserialize
)]
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
    id: Id,
    email: Cow<'a, Email>,
    nickname: Cow<'a, Nickname>,
    _password: PhantomData<Password>,
    password_hash: PasswordHash,
    created_at: CreatedAt,
}

impl<'a> ApplicationUser<'a> {
    pub fn new(
        id: Id,
        email: Cow<'a, Email>,
        nickname: Cow<'a, Nickname>,
        password_hash: PasswordHash,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            id,
            email,
            nickname,
            _password: PhantomData,
            password_hash,
            created_at,
        };
    }

    pub fn get_id<'b>(&'b self) -> Id {
        return self.id;
    }

    pub fn get_email<'b>(&'b self) -> &'b Email {
        return self.email.as_ref();
    }

    pub fn get_nickname<'b>(&'b self) -> &'b Nickname {
        return self.nickname.as_ref();
    }

    pub fn get_password_hash<'b>(&'b self) -> &'b PasswordHash {
        return &self.password_hash;
    }

    pub fn get_created_at<'b>(&'b self) -> &'b CreatedAt {
        return &self.created_at;
    }

    pub fn set_password_hash<'b>(&'b mut self, password_hash: PasswordHash) -> &'b mut Self {
        self.password_hash = password_hash;

        return self;
    }
}

impl<'a> Getter<'a, Id> for ApplicationUser<'_> {
    fn get(&'a self) -> Id {
        return self.get_id();
    }
}

impl<'a> Getter<'a, &'a Email> for ApplicationUser<'_> {
    fn get(&'a self) -> &'a Email {
        return self.get_email();
    }
}

impl<'a> Getter<'a, &'a Nickname> for ApplicationUser<'_> {
    fn get(&'a self) -> &'a Nickname {
        return self.get_nickname();
    }
}

impl<'a> Getter<'a, &'a PasswordHash> for ApplicationUser<'_> {
    fn get(&'a self) -> &'a PasswordHash {
        return self.get_password_hash();
    }
}

impl<'a> Getter<'a, &'a CreatedAt> for ApplicationUser<'_> {
    fn get(&'a self) -> &'a CreatedAt {
        return self.get_created_at();
    }
}

pub struct ApplicationUser_1 {
    id: Id,
    email: Email,
    password_hash: PasswordHash,
}

impl ApplicationUser_1 {
    pub fn new(id: Id, email: Email, password_hash: PasswordHash) -> Self {
        return Self {
            id,
            email,
            password_hash,
        };
    }

    pub fn get_id<'a>(&'a self) -> Id {
        return self.id;
    }

    pub fn get_email<'a>(&'a self) -> &'a Email {
        return &self.email;
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a PasswordHash {
        return &self.password_hash;
    }
}

pub struct ApplicationUser_2 {
    id: Id,
    password_hash: PasswordHash,
}

impl ApplicationUser_2 {
    pub fn new(id: Id, password_hash: PasswordHash) -> Self {
        return Self {
            id,
            password_hash,
        };
    }

    pub fn get_id<'a>(&'a self) -> Id {
        return self.id;
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a PasswordHash {
        return &self.password_hash;
    }
}

pub struct ApplicationUser_3 {
    id: Id,
}

impl ApplicationUser_3 {
    pub fn new(id: Id) -> Self {
        return Self {
            id,
        };
    }

    pub fn get_id<'a>(&'a self) -> Id {
        return self.id;
    }
}

pub struct ApplicationUser_4 {
    password_hash: PasswordHash,
}

impl ApplicationUser_4 {
    pub fn new(password_hash: PasswordHash) -> Self {
        return Self {
            password_hash,
        };
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a PasswordHash {
        return &self.password_hash;
    }

    pub fn set_password_hash<'b>(&'b mut self, password_hash: PasswordHash) -> &'b mut Self {
        self.password_hash = password_hash;

        return self;
    }
}

impl<'a> Getter<'a, &'a PasswordHash> for ApplicationUser_4 {
    fn get(&'a self) -> &'a PasswordHash {
        return self.get_password_hash();
    }
}

pub struct ApplicationUser_5 {
    email: Email,
}

impl ApplicationUser_5 {
    pub fn new(email: Email) -> Self {
        return Self {
            email,
        };
    }

    pub fn get_email<'a>(&'a self) -> &'a Email {
        return &self.email;
    }
}

impl<'a> Getter<'a, &'a Email> for ApplicationUser_5 {
    fn get(&'a self) -> &'a Email {
        return self.get_email();
    }
}
