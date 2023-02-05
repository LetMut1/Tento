use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessToken_WorkflowException;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel_PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ApplicationUserAccessToken_Extractor;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::infrastructure_layer::functionality::service::order_convention_resolver::OrderConventionResolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    const LIMIT: i8 = 30;

    pub async fn process<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        database_1_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let mut limit = incoming.limit;

        let extractor_result = match ApplicationUserAccessToken_Extractor::extract(
            environment_configuration_resolver, incoming.application_user_access_token_deserialized_form.as_str()
        ).await {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        match extractor_result {
            ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {}
            ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                return Ok(ActionProcessorResult::application_user_access_token__workflow_exception(ApplicationUserAccessToken_WorkflowException::AlreadyExpired));
            }
            ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                return Ok(ActionProcessorResult::application_user_access_token__workflow_exception(ApplicationUserAccessToken_WorkflowException::InApplicationUserAccessTokenBlackList));
            }
            ExtractorResult::ApplicationUserAccessTokenWrongDeserializedForm => {
                return Ok(ActionProcessorResult::application_user_access_token__workflow_exception(ApplicationUserAccessToken_WorkflowException::WrongDeserializedForm));
            }
        }

        if let Some(ref channel_created_at_) = incoming.channel_created_at {
            if !DateTimeResolver::is_valid_timestamp(channel_created_at_.as_str()) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::InvalidArgumentError,
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }

        if !OrderConventionResolver::can_convert(incoming.order) {
            return Err(
                ErrorAuditor::new(
                    BaseError::InvalidArgumentError,
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        if limit <= 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_registry = match Channel_PostgresqlRepository::per_request_2(
            &*database_1_postgresql_pooled_connection, &incoming.channel_created_at, incoming.order, limit as i16
        ).await {
            Ok(channel_registry_) => channel_registry_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(ActionProcessorResult::outcoming(Outcoming { channel_registry }));
    }
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_deserialized_form: String,
    channel_created_at:  Option<String>,
    order: i8,
    limit: i8
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    channel_registry: Option<Vec<Channel>>
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel {
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_personalization_image_path: String,
    pub channel_subscribers_quantity: i64,
    pub channel_public_marks_quantity: i64,
    pub channel_hidden_marks_quantity: i64,
    pub channel_reactions_quantity: i64,
    pub channel_viewing_quantity: i64,
    pub channel_created_at: String
}