use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::select::Select;
use crate::entity::entity::application_user::_core::email::Email;
use super::_core::id::Id;
use super::_core::created_at::CreatedAt;

pub struct PreConfirmedApplicationUser {
    id: Id,
    application_user_email: Email,
    created_at: CreatedAt
}

impl PreConfirmedApplicationUser {
    pub fn new(email: Email) -> Self {
        return Self {
            id: Id::new(),
            application_user_email: email,
            created_at: CreatedAt::new()
        };
    }

    pub fn new_from_model(select: Select) -> Self {
        let (
            id,
            application_user_email,
            created_at
        ) = select.into_inner();

        return Self {
            id: Id::new_from_uuid(id),
            application_user_email: Email::new(application_user_email),
            created_at: CreatedAt::new_from_date_time(created_at)
        };
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.id;
    }

    pub fn get_email<'this>(&'this self) -> &'this Email {
        return &self.application_user_email;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}