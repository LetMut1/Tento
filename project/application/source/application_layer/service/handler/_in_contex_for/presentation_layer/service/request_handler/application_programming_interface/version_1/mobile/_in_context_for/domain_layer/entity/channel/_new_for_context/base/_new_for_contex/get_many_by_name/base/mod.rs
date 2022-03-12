use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::channel::_new_for_context::base_trait::BaseTrait as ChannelValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::channel::_new_for_context::base::Base as ChannelValidator;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let (
            mut channel_name,
            mut requery_channel_name,
            mut limit
        )= request_data.into_inner();
        
        if limit <= 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        channel_name = String::from_utf8(base64::decode_config(channel_name, base64::URL_SAFE)?)?;
        if !ChannelValidator::is_valid_name(channel_name.as_str()) {
            return Err(BaseError::InvalidArgumentError);
        }
        match requery_channel_name {
            Some(mut requery_channel_name_) => {
                requery_channel_name_ = String::from_utf8(base64::decode_config(requery_channel_name_, base64::URL_SAFE)?)?;
                if !ChannelValidator::is_valid_name(requery_channel_name_.as_str()) {
                    return Err(BaseError::InvalidArgumentError);
                }
                
                requery_channel_name = Some(requery_channel_name_);
            },
            None => {
                requery_channel_name = None;
            }
        }

        let channel_registry = ChannelDataProviderPostgresql::per_request_1(
            &mut *postgresql_connection_pool.get().await?, channel_name.as_str(), &requery_channel_name, &(limit as i16)
        ).await?;

        return Ok(ResponseData::new(channel_registry));
    }
}