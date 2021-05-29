pub struct UpdateResolver {
    change_email: bool,
    change_nickname: bool,
    change_password_hash: bool,
    change_created_at: bool
}

impl UpdateResolver {
    pub fn new(change_email: bool, change_nickname: bool, change_password_hash: bool, change_created_at: bool) -> Self {
        return Self {
            change_email,
            change_nickname,
            change_password_hash,
            change_created_at
        };
    }

    pub fn is_change_email<'this>(&'this self) -> bool {
        return self.change_email;
    }

    pub fn is_change_nickname<'this>(&'this self) -> bool {
        return self.change_nickname;
    }

    pub fn is_change_password_hash<'this>(&'this self) -> bool {
        return self.change_password_hash;
    }

    pub fn is_change_created_at<'this>(&'this self) -> bool {
        return self.change_created_at;
    }
}