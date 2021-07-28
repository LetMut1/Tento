use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::_component::created_at::CreatedAt;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::_component::id::Id;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::select::Select;

pub struct Factory;

impl Factory {
    pub fn new_from_email(email: Email) -> PreConfirmedApplicationUser {
        return PreConfirmedApplicationUser::new(
            None,
            email,
            CreatedAt::new()
        );
    }

    pub fn new_from_select(select: Select) -> PreConfirmedApplicationUser {
        let (
            id,
            application_user_email,
            created_at
        ) = select.into_inner();

        return PreConfirmedApplicationUser::new(
            Some(Id::new(id)),
            Email::new(application_user_email),
            CreatedAt::new_from_date_time(created_at)
        );
    }
}