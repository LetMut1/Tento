use crate::domain_layer::error::logic_error::LogicError;

pub struct ApplicationUserChannelAdministrator {
    id: Option<i64>,
    email: String,
    nickname: String,
    password_hash: String,
    created_at: String
}

impl ApplicationUserChannelAdministrator {
    pub fn new(
        id: Option<i64>,
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

    pub fn get_id<'a>(
        &'a self
    ) -> Result<&'a i64, LogicError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(LogicError::new("Id does not exist yet."))
            }
        }
    }

    pub fn get_email<'a>(
        &'a self
    ) -> &'a str {
        return self.email.as_str();
    }

    pub fn get_nickname<'a>(
        &'a self
    ) -> &'a str {
        return self.nickname.as_str();
    }

    pub fn get_password_hash<'a>(
        &'a self
    ) -> &'a str {
        return self.password_hash.as_str();
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }

    pub fn set_password_hash<'a>(
        &'a mut self,
        password_hash: String
    ) -> &'a mut Self {
        self.password_hash = password_hash;

        return self;
    }
}