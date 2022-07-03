pub struct Base {
    update_email: bool,
    update_nickname: bool,
    update_password_hash: bool
}

impl Base {
    pub fn new(
        update_email: bool,
        update_nickname: bool,
        update_password_hash: bool
    ) -> Self {
        return Self {
            update_email,
            update_nickname,
            update_password_hash
        };
    }

    pub fn is_update_email<'a>(
        &'a self
    ) -> bool {
        return self.update_email;
    }

    pub fn is_update_nickname<'a>(
        &'a self
    ) -> bool {
        return self.update_nickname;
    }

    pub fn is_update_password_hash<'a>(
        &'a self
    ) -> bool {
        return self.update_password_hash;
    }
}