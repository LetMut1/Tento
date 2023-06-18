use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::WorkflowPrecedent;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::send_email_for_authorize::ActionProcessor;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::send_email_for_authorize::Incoming;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::send_email_for_authorize::Outcoming;
use crate::application_layer::functionality::core_action_processor::CoreActionProcessor;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use crate::presentation_layer::data::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::wrapped_action_processor::WrappedActionProcessor;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

pub struct SendEmailForAuthorize;

impl SendEmailForAuthorize {
    pub async fn run<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return CoreActionProcessor::process::<'_, MessagePack, _, _, _, Incoming, Outcoming, _>(
            environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            ActionProcessor::process,
            Self::resolve
        ).await;
    }

    fn resolve(
        action_processor_result: ActionProcessorResult<Outcoming>
    ) -> Result<UnifiedReport<Outcoming>, ErrorAuditor> {
        match action_processor_result {
            ActionProcessorResult::Void => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::create_unreachable_state(),
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
            ActionProcessorResult::Outcoming { outcoming } => {
                return Ok(UnifiedReport::data(outcoming));
            }
            ActionProcessorResult::WorkflowPrecedent { workflow_precedent } => {
                match workflow_precedent {
                    WorkflowPrecedent::ApplicationUser_NotFound => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER__NOT_FOUND
                            )
                        );
                    }
                    WorkflowPrecedent::ApplicationUserAuthorizationToken_NotFound => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
                            )
                        );
                    }
                    WorkflowPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
                            )
                        );
                    }
                    WorkflowPrecedent::ApplicationUserAuthorizationToken_TimeToResendHasNotCome => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_AUTHORIZATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
                            )
                        );
                    }
                    _ => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::create_unreachable_state(),
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
        }
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl SendEmailForAuthorize {
    pub async fn run_<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return WrappedActionProcessor::process::<'_, Json, MessagePack, _, _, _, Incoming, Outcoming>(
            environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            Self::run
        ).await;
    }
}