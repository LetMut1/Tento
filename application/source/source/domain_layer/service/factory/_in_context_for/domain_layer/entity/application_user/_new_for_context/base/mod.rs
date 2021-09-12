use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::select::Select;
use std::borrow::Cow;

pub struct Base;

impl Base {
    pub fn new_from_application_user_pre_confirmed<'outer_a>(
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed,
        nickname: String,
        password_hash: String
    ) -> ApplicationUser<'_> {
        return ApplicationUser::new(
            None,
            Cow::Borrowed(application_user_pre_confirmed.get_application_user_email()),
            nickname,
            password_hash,
            chrono::Utc::now().to_rfc2822() // TODO 
        );
    }

    pub fn new_from_select(
        select: Select
    ) -> ApplicationUser<'static> {
        let (
            id,
            email,
            nickname,
            password_hash,
            created_at
        ) = select.into_inner();

        return ApplicationUser::new(
            Some(id),
            Cow::Owned(email),
            nickname,
            password_hash,
            created_at.to_rfc2822() // TODO
        );
    }
}