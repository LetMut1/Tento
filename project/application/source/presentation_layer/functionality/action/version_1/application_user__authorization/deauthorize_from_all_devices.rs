use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::deauthorize_from_all_devices::ActionProcessor;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::deauthorize_from_all_devices::Incoming;
use crate::application_layer::functionality::core_action_processor::CoreActionProcessor;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::void::Void;
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
use crate::presentation_layer::functionality::service::wrapped_action_creator::WrappedActionCreator;

pub struct DeauthorizeFromAllDevices;

impl DeauthorizeFromAllDevices {
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
        return CoreActionProcessor::process::<'_, MessagePack, _, _, _, Incoming, Void, _>(
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
        action_processor_result: ActionProcessorResult<Void>
    ) -> Result<UnifiedReport<Void>, ErrorAuditor> {
        match action_processor_result {
            ActionProcessorResult::Void => {
                return Ok(UnifiedReport::empty());
            }
            ActionProcessorResult::Outcoming { outcoming: _ } => {
               return Err(
                    ErrorAuditor::new(
                        BaseError::create_unreachable_state(),
                        BacktracePart::new(line!(), file!(), None)
                    )
               );
            }
            ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent } => {
                match user_workflow_precedent {
                    UserWorkflowPrecedent::ApplicationUserAccessToken_AlreadyExpired => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        return Ok(
                            UnifiedReport::<Void>::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
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
impl DeauthorizeFromAllDevices {
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
        return WrappedActionCreator::create_for_json::<'_, _, _, _, Incoming, Void>(
            environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            Self::run
        ).await;
    }
}