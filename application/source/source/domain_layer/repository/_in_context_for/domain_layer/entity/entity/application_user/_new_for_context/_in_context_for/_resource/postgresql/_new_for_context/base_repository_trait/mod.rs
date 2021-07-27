use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::id::Id;
use crate::domain_layer::entity::entity::application_user::_core::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use diesel::PgConnection as Connection;

pub trait BaseRepositoryTrait {
    fn create<'outer_a>(
        connection: &'outer_a Connection, application_user: &'outer_a ApplicationUser<'_>
    ) -> Result<(), BaseError>;

    fn update<'outer_a>(
        connection: &'outer_a Connection, application_user: &'outer_a ApplicationUser<'_>, update_resolver: UpdateResolver
    ) -> Result<(), BaseError>;

    fn is_exist_by_nickanme<'outer_a>(connection: &'outer_a Connection, nickname: &'outer_a Nickname) -> Result<bool, BaseError>;

    fn is_exist_by_email<'outer_a>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<bool, BaseError>;

    fn get_by_email<'outer_a>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<Option<ApplicationUser<'static>>, BaseError>;

    fn get_by_id<'outer_a>(connection: &'outer_a Connection, id: &'outer_a Id) -> Result<Option<ApplicationUser<'static>>, BaseError>;
}