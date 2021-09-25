use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;
    type UpdateResolverApplicationUser: UpdateResolverApplicationUserTrait;
    
    fn create<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser
    ) -> Result<(), Self::Error>;

    fn update<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser,
        update_resolver: Self::UpdateResolverApplicationUser
    ) -> Result<(), Self::Error>;
}