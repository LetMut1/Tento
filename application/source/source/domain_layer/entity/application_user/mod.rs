use crate::domain_layer::error::logic_error::LogicError;
use std::borrow::Cow;

pub struct ApplicationUser<'outer_a> {
    id: Option<i64>,
    email: Cow<'outer_a, str>,
    nickname: String,
    password_hash: String,
    created_at: String
}

impl<'outer_a> ApplicationUser<'outer_a> {
    pub fn new(
        id: Option<i64>,
        email: Cow<'outer_a, str>,
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

    pub fn set_password_hash<'this>(
        &'this mut self,
        password_hash: String
    ) -> &'this mut Self {
        self.password_hash = password_hash;

        return self;
    }

    pub fn get_id<'this>(
        &'this self
    ) -> Result<&'this i64, LogicError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(LogicError::new("Id does not exist yet."))
            }
        }
    }

    pub fn get_email<'this>(
        &'this self
    ) -> &'this str {
        return self.email.as_ref();
    }

    pub fn get_nickname<'this>(
        &'this self
    ) -> &'this str {
        return &self.nickname;
    }

    pub fn get_password_hash<'this>(
        &'this self
    ) -> &'this str {
        return self.password_hash.as_str();
    }

    pub fn get_created_at<'this>(
        &'this self
    ) -> &'this str {
        return self.created_at.as_str();
    }
}