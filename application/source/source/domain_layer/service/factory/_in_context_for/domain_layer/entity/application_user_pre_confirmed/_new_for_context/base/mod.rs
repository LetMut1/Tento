use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::select::Select;

pub struct Base;

impl Base {
    pub fn new_from_email(
        email: String
    ) -> ApplicationUserPreConfirmed {
        return ApplicationUserPreConfirmed::new(
            None,
            email,
            crate::chrono::Utc::now().to_rfc2822()   // TODO 
        );
    }

    pub fn new_from_select(
        select: Select
    ) -> ApplicationUserPreConfirmed {
        let (
            id,
            application_user_email,
            created_at
        ) = select.into_inner();

        return ApplicationUserPreConfirmed::new(
            Some(id),
            application_user_email,
            created_at.to_rfc2822() // TODO 
        );
    }
}