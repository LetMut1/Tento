use crate::domain_layer::entity::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::entity::application_user::_component::created_at::CreatedAt;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::id::Id;
use crate::domain_layer::entity::entity::application_user::_component::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::_component::password_hash::PasswordHash;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::select::Select;
use std::borrow::Cow;

pub struct Factory;

impl Factory {
    pub fn new_from_application_user_pre_confirmed<'outer_a>(
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed, nickname: Nickname, password_hash: PasswordHash
    ) -> ApplicationUser<'_> {
        return ApplicationUser::new(
            None,
            Cow::Borrowed(application_user_pre_confirmed.get_application_user_email()),
            nickname,
            password_hash,
            CreatedAt::new()
        );
    }

    pub fn new_from_select(select: Select) -> ApplicationUser<'static> {
        let (
            id,
            email,
            nickname,
            password_hash,
            created_at
        ) = select.into_inner();

        return ApplicationUser::new(
            Some(Id::new(id)),
            Cow::Owned(Email::new(email)),
            Nickname::new(nickname),
            PasswordHash::new(password_hash),
            CreatedAt::new_from_date_time(created_at)
        );
    }
}