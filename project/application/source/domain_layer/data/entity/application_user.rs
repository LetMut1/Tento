use std::borrow::Cow;
use std::marker::PhantomData;

pub struct ApplicationUser<'a> {
    id: i64,
    _id: PhantomData<Id>,

    email: Cow<'a, str>,
    _email: PhantomData<Email>,

    nickname: Cow<'a, str>,
    _nickname: PhantomData<Nickname>,

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

pub struct Id;

pub struct Email;

pub struct Nickname;

pub struct PasswordHash;

pub struct CreatedAt;