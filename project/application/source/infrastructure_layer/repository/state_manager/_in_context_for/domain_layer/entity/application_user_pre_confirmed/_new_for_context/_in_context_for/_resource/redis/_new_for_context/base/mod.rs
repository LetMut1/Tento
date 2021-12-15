use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedStateManagerRedisTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl ApplicationUserPreConfirmedStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'a>(
        connection: &'a mut Connection, 
        application_user_pre_confirmed: &'a ApplicationUserPreConfirmed,
        quantity_of_minutes_for_expiration: Option<u16>
    ) -> Result<(), Self::Error> {
        match quantity_of_minutes_for_expiration {
            Some(quantity_of_minutes_for_expiration_) => {
                if quantity_of_minutes_for_expiration_ == 0 {
                    return Err(BaseError::LogicError {unreachable: false, message: "Quantity of minutes for expiration must be greater than 0."});
                }

                connection.set_ex::<&'_ str, u8, ()>(
                    StorageKeyResolver::get_repository_application_user_pre_confirmed_first(
                        application_user_pre_confirmed.get_application_user_email()
                    ),
                    1,
                    (quantity_of_minutes_for_expiration_ as usize) * (60 as usize)         // TODO посмотреть, можно ли делать как  (quantity_of_minutes_for_expiration * 60) as usize
                )?;
            },
            None => {
                connection.set::<&'_ str, u8, ()>(
                    StorageKeyResolver::get_repository_application_user_pre_confirmed_first(
                        application_user_pre_confirmed.get_application_user_email()
                    ),
                    1
                )?;
            }
        }

        return Ok(());
    }

    fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_email: &'a str
    ) -> Result<(), Self::Error> {
        connection.del::<&'_ str, ()>(
            StorageKeyResolver::get_repository_application_user_pre_confirmed_first(application_user_email)
        )?;
        
        return Ok(());
    }
}