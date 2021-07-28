use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::created_at::CreatedAt;
use super::_component::id::Id;

pub struct PreConfirmedApplicationUser {
    id: Option<Id>,
    application_user_email: Email,
    created_at: CreatedAt
}

impl PreConfirmedApplicationUser {
    pub fn new(
        id: Option<Id>,
        application_user_email: Email,
        created_at: CreatedAt
    ) -> Self {
        return Self {id, application_user_email, created_at};
    }

    pub fn get_id<'this>(&'this self) -> Result<&'this Id, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_email<'this>(&'this self) -> &'this Email {
        return &self.application_user_email;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}