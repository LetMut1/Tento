pub struct UpdateResolver {
    change_email: bool,
    change_nickname: bool,
    change_password_hash: bool,
    change_created_at: bool
}

impl<'this> UpdateResolver {
    pub fn new(change_email: bool, change_nickname: bool, change_password_hash: bool, change_created_at: bool) -> Self {
        return Self {
            change_email,
            change_nickname,
            change_password_hash,
            change_created_at
        };
    }

    pub fn is_change_email(&'this self) -> bool {
        return self.change_email;
    }

    pub fn is_change_nickname(&'this self) -> bool {
        return self.change_nickname;
    }

    pub fn is_change_password_hash(&'this self) -> bool {
        return self.change_password_hash;
    }

    pub fn is_change_created_at(&'this self) -> bool {
        return self.change_created_at;
    }
}