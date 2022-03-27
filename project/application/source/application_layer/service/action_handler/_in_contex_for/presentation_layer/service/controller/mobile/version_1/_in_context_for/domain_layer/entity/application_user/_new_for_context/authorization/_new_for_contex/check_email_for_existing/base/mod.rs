use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let application_user_email = request_data.into_inner();

        if ApplicationUserValidator::is_valid_email(application_user_email.as_str())? {
            let result = ApplicationUserDataProviderPostgresql::is_exist_by_email(
                &mut *postgresql_connection_pool.get().await?, application_user_email.as_str()
            ).await?;

            return Ok(ResponseData::new(result));
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidEmail}});
    }
}