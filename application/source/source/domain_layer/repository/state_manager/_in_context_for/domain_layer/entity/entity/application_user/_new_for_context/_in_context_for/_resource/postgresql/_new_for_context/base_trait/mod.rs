use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use diesel::PgConnection as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;
    
    fn create<'outer_a>(
        connection: &'outer_a Connection,
        application_user: &'outer_a ApplicationUser<'_>
    ) -> Result<(), Self::Error>;

    fn update<'outer_a>(
        connection: &'outer_a Connection,
        application_user: &'outer_a ApplicationUser<'_>,
        update_resolver: UpdateResolver
    ) -> Result<(), Self::Error>;
}