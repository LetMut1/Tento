pub struct ApplicationUser {
    id: i64,
    email: String,
    nickname: String,
    password_hash: String,
    created_at: String
}

impl ApplicationUser {
    pub fn new(
        id: i64,
        email: String,
        nickname: String,
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

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_email<'a>(&'a self) -> &'a str {
        return self.email.as_str();
    }

    pub fn get_nickname<'a>(&'a self) -> &'a str {
        return &self.nickname;
    }

    pub fn get_password_hash<'a>(&'a self) -> &'a str {
        return self.password_hash.as_str();
    }

    pub fn set_password_hash<'a>(&'a mut self, password_hash: String) -> &'a mut Self {
        self.password_hash = password_hash;

        return self;
    }
}