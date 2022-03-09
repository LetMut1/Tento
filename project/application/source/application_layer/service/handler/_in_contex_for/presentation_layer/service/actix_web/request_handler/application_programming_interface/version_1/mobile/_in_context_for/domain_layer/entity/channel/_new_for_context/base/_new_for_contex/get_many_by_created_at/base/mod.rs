use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::base::Base as Response;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        request: Request
    ) -> Result<Response, BaseError> {
        let (
            mut channel_created_at,
            order,
            mut limit
        ) = request.into_inner();
        
        if limit <= 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        if !OrderConventionResolver::can_convert(&order) {
            return Err(BaseError::InvalidArgumentError);
        }

        if let Some(mut channel_created_at_) = channel_created_at {
            channel_created_at_ = String::from_utf8(base64::decode_config(channel_created_at_, base64::URL_SAFE)?)?;
            if !DateTimeResolver::is_valid_timestamp(channel_created_at_.as_str()) {
                return Err(BaseError::InvalidArgumentError);
            }

            channel_created_at = Some(channel_created_at_);
        }

        let channel_registry = ChannelDataProviderPostgresql::per_request_2(
            &mut *postgresql_connection_pool.get().await?, &channel_created_at, &order, &(limit as i16)
        ).await?;

        return Ok(Response::new(channel_registry));
    }
}