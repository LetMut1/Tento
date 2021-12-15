use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedDataProviderRedisTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl ApplicationUserPreConfirmedDataProviderRedisTrait for Base {
    type Error = BaseError;

    fn is_exist_by_application_user_email<'a>(
        connection: &'a mut Connection,
        application_user_email: &'a str
    ) -> Result<bool, Self::Error> {
        let result: bool = connection.exists::<&'_ str, bool>(
            StorageKeyResolver::get_repository_application_user_pre_confirmed_first(application_user_email)
        )?;

        return Ok(result);
    }
}