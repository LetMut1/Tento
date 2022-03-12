
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::base::Base as Response;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const CHANNEL_ID_REGISTRY_LENGTH_LIMIT: usize = 30;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        request_data: RequestData
    ) -> Result<Response, BaseError> {
        let channel_id_registry = request_data.into_inner();
        let channel_id_registry_ = serde_json::from_str::<Vec<i64>>(channel_id_registry.as_str())?;
        if channel_id_registry_.len() == 0 || channel_id_registry_.len() > Self::CHANNEL_ID_REGISTRY_LENGTH_LIMIT {
            return Err(BaseError::InvalidArgumentError);
        }

        let channel_registry = ChannelDataProviderPostgresql::per_request_4(
            &mut *postgresql_connection_pool.get().await?, &channel_id_registry_
        ).await?;

        return Ok(Response::new(channel_registry));
    }
}