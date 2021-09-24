use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;
    type UpdateResolverApplicationUser: UpdateResolverApplicationUserTrait;
    
    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user: &'outer_a ApplicationUser
    ) -> Result<(), Self::Error>;

    fn update<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user: &'outer_a ApplicationUser,
        update_resolver: Self::UpdateResolverApplicationUser
    ) -> Result<(), Self::Error>;
}