use crate::domain_layer::error::logic_error::LogicError;

pub struct ApplicationUserPreConfirmed {
    id: Option<i64>,
    application_user_email: String,
    created_at: String
}

impl ApplicationUserPreConfirmed {
    pub fn new(
        id: Option<i64>,
        application_user_email: String,
        created_at: String
    ) -> Self {
        return Self {
            id,
            application_user_email,
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

    pub fn get_application_user_email<'a>(
        &'a self
    ) -> &'a str {
        return self.application_user_email.as_str();
    }

    pub fn get_application_user_email_as_owner(
        self
    ) -> String {
        return self.application_user_email;
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}