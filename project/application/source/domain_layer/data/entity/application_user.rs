use std::borrow::Cow;

pub struct ApplicationUser<'a> {
    id: i64,
    email: Cow<'a, str>,
    nickname: Cow<'a, str>,
    password_hash: String,
    created_at: String
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
            email,
            nickname,
            password_hash,
            created_at
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