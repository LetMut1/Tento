pub struct Insert {
    email: String,
    nickname: String,
    password_hash: String
}

impl Insert {
    pub fn new(
        email: String,
        nickname: String,
        password_hash: String
    ) -> Self {
        return Self {
            email,
            nickname,
            password_hash
        }
    }

    pub fn into_inner(
        self
    ) -> (String, String, String) {
        return (
            self.email,
            self.nickname,
            self.password_hash
        );
    }
}