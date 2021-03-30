use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::pre_confirmed_application_user::_new_for_context::existing::Existing;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::core::email::Email;

pub struct PreConfirmedApplicationUser {
    id: UuidV4,
    email: Email    // TODO add created_at (чтобы по крону удалять )
}

impl<'this> PreConfirmedApplicationUser {
    pub fn new(email: Email) -> Self {
        return Self {
            id: UuidV4::new(),
            email
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.id),
            email: Email::new(existing.email)
        };
    }

    pub fn get_id(&'this self) -> &'this UuidV4 {
        return &self.id;
    }

    pub fn get_email(&'this self) -> &'this Email {
        return &self.email;
    }
}