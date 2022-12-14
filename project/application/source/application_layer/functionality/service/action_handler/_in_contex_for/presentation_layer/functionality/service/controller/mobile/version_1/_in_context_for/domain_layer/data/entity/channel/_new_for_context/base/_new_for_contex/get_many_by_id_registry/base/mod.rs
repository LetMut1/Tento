use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

#[derive(Deserialize)]
pub struct ActionHandlerIncomingData {
    application_user_access_token_web_form: String,
    channel_id_registry: Vec<i64>,
}

impl ActionHandlerIncomingData {
    pub fn into_inner(
        self
    ) -> (String, Vec<i64>) {
        return (
            self.application_user_access_token_web_form,
            self.channel_id_registry,
        );
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct Channel {
    channel_id: i64,
    channel_name: String,
    channel_personalization_image_path: String,
    channel_subscribers_quantity: i64,
    channel_public_marks_quantity: i64,
    channel_hidden_marks_quantity: i64,
    channel_reactions_quantity: i64,
    channel_viewing_quantity: i64,
    channel_created_at: String
}

impl Channel {
    pub fn new(
        channel_id: i64,
        channel_name: String,
        channel_personalization_image_path: String,
        channel_subscribers_quantity: i64,
        channel_public_marks_quantity: i64,
        channel_hidden_marks_quantity: i64,
        channel_reactions_quantity: i64,
        channel_viewing_quantity: i64,
        channel_created_at: String
    ) -> Self {
        return Self {
            channel_id,
            channel_name,
            channel_personalization_image_path,
            channel_subscribers_quantity,
            channel_public_marks_quantity,
            channel_hidden_marks_quantity,
            channel_reactions_quantity,
            channel_viewing_quantity,
            channel_created_at
        };
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct ActionHandlerOutcomingData {
    channel_registry: Option<Vec<Channel>>
}

impl ActionHandlerOutcomingData {
    pub fn new(
        channel_registry: Option<Vec<Channel>>
    ) -> Self {
        return Self {
            channel_registry
        };
    }
}

pub struct Base;

impl Base {
    const CHANNEL_ID_REGISTRY_LENGTH_LIMIT: usize = 30;

    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_access_token_web_form,
            channel_id_registry
        ) = action_handler_incoming_data.into_inner();

        match Extractor::extract(environment_configuration_resolver, application_user_access_token_web_form.as_str()).await {
            Ok(result) => {
                match result {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {
                        if channel_id_registry.is_empty() || channel_id_registry.len() > Self::CHANNEL_ID_REGISTRY_LENGTH_LIMIT {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::InvalidArgumentError,
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }

                        match core_postgresql_connection_pool.get().await {
                            Ok(core_postgresql_pooled_connection) => {
                                match ChannelDataProviderPostgresql::per_request_4(
                                    &*core_postgresql_pooled_connection, &channel_id_registry
                                ).await {
                                    Ok(channel_registry) => {
                                        return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(channel_registry)));
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                        return Err(error);
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::AlreadyExpired));
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList));
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}