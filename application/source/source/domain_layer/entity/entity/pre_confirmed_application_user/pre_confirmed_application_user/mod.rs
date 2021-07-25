use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::select::Select;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_core::created_at::CreatedAt;
use super::_core::id::Id;

pub struct PreConfirmedApplicationUser {
    id: Option<Id>,
    application_user_email: Email,
    created_at: CreatedAt
}

impl PreConfirmedApplicationUser {
    pub fn new(email: Email) -> Self {
        return Self {
            id: None,
            application_user_email: email,
            created_at: CreatedAt::new()
        };
    }

    pub fn new_from_select(select: Select) -> Self {
        let (
            id,
            application_user_email,
            created_at
        ) = select.into_inner();

        return Self {
            id: Some(Id::new(id)),
            application_user_email: Email::new(application_user_email),
            created_at: CreatedAt::new_from_date_time(created_at)
        };
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