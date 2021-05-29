use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::select::Select;
use crate::entity::entity::application_user::core::email::Email;
use super::core::id::Id;

pub struct PreConfirmedApplicationUser {
    id: Id,
    application_user_email: Email    // TODO add created_at (чтобы по крону удалять )
}

impl PreConfirmedApplicationUser {
    pub fn new(email: Email) -> Self {
        return Self {
            id: Id::new(),
            application_user_email: email
        };
    }

    pub fn new_from_model(select: Select) -> Self {
        return Self {
            id: Id::new_from_uuid(select.id),
            application_user_email: Email::new(select.application_user_email)
        };
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.id;
    }

    pub fn get_email<'this>(&'this self) -> &'this Email {
        return &self.application_user_email;
    }
}