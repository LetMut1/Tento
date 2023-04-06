use std::borrow::Cow;
use std::marker::PhantomData;

pub use self::Id as ApplicationUser_Id;
pub use self::Email as ApplicationUser_Email;
pub use self::Nickname as ApplicationUser_Nickname;
pub use self::Password as ApplicationUser_Password;
pub use self::PasswordHash as ApplicationUser_PasswordHash;
pub use self::CreatedAt as ApplicationUser_CreatedAt;

pub struct ApplicationUser<'a> {
    id: i64,
    _id: PhantomData<Id>,

    email: Cow<'a, str>,
    _email: PhantomData<Email>,

    nickname: Cow<'a, str>,
    _nickname: PhantomData<Nickname>,

    _password: PhantomData<Password>,

    password_hash: String,
    _password_hash: PhantomData<PasswordHash>,

    created_at: String,
    _created_at: PhantomData<CreatedAt>
}

impl<'a> ApplicationUser<'a> {
    pub fn new(
        id: i64,
        email: Cow<'a, str>,
        nickname: Cow<'a, str>,
        password_hash: String,
        created_at: String
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            email,
            _email: PhantomData,
            nickname,
            _nickname: PhantomData,
            _password: PhantomData,
            password_hash,
            _password_hash: PhantomData,
            created_at,
            _created_at: PhantomData
        };
    }

    pub fn get_id<'b>(&'b self) -> i64 {
        return self.id;
    }

    pub fn get_email<'b>(&'b self) -> &'b str {
        return self.email.as_ref();
    }

    pub fn get_nickname<'b>(&'b self) -> &'b str {
        return self.nickname.as_ref();
    }

    pub fn get_password_hash<'b>(&'b self) -> &'b str {
        return self.password_hash.as_str();
    }

    pub fn set_password_hash<'b>(&'b mut self, password_hash: String) -> &'b mut Self {
        self.password_hash = password_hash;

        return self;
    }
}

pub struct ApplicationUser_1 {
    id: i64,
    _id: PhantomData<Id>,

    email: String,
    _email: PhantomData<Email>,

    password_hash: String,
    _password_hash: PhantomData<PasswordHash>,
}

impl ApplicationUser_1 {
    pub fn new(
        id: i64,
        email: String,
        password_hash: String
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            email,
            _email: PhantomData,
            password_hash,
            _password_hash: PhantomData
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_email<'a>(&'a self) -> &'a str {
        return self.email.as_str();
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a str {
        return self.password_hash.as_str();
    }
}

pub struct ApplicationUser_2 {
    id: i64,
    _id: PhantomData<Id>,

    password_hash: String,
    _password_hash: PhantomData<PasswordHash>,
}

impl ApplicationUser_2 {
    pub fn new(
        id: i64,
        password_hash: String
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            password_hash,
            _password_hash: PhantomData
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a str {
        return self.password_hash.as_str();
    }
}

pub struct ApplicationUser_3 {
    id: i64,
    _id: PhantomData<Id>
}

impl ApplicationUser_3 {
    pub fn new(
        id: i64
    ) -> Self {
        return Self {
            id,
            _id: PhantomData
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }
}

pub struct Id;

pub struct Email;

pub struct Nickname;

pub struct Password;

pub struct PasswordHash;

pub struct CreatedAt;