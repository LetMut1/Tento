use crate::domain_layer::entity::entity::application_user_pre_confirmed::_component::created_at::CreatedAt;
use crate::domain_layer::entity::entity::application_user_pre_confirmed::_component::id::Id;
use crate::domain_layer::entity::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::select::Select;

pub struct Base;

impl Base {
    pub fn new_from_email(email: Email) -> ApplicationUserPreConfirmed {
        return ApplicationUserPreConfirmed::new(
            None,
            email,
            CreatedAt::new()
        );
    }

    pub fn new_from_select(select: Select) -> ApplicationUserPreConfirmed {
        let (
            id,
            application_user_email,
            created_at
        ) = select.into_inner();

        return ApplicationUserPreConfirmed::new(
            Some(Id::new(id)),
            Email::new(application_user_email),
            CreatedAt::new_from_date_time(created_at)
        );
    }
}