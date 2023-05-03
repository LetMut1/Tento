use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::register_by_last_step::ActionProcessor;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::register_by_last_step::Incoming;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::register_by_last_step::Outcoming;
use crate::application_layer::functionality::core_action_processor::CoreActionProcessor;
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
use extern_crate::hyper::Body;
use extern_crate::hyper::Request;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::wrapped_action_creator::WrappedActionCreator;

pub struct RegisterByLastStep;

impl RegisterByLastStep {
    pub async fn run<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        request: Request<Body>,
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
                )
            }
            ActionProcessorResult::Outcoming { outcoming } => {
                return Ok(UnifiedReport::data(outcoming));
            }
            ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent } => {
                match user_workflow_precedent {
                    UserWorkflowPrecedent::ApplicationUser_NicknameAlreadyExist => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER__NICKNAME_ALREADY_EXIST
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUser_EmailAlreadyExist => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER__EMAIL_ALREADY_EXIST
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUserRegistrationToken_NotFound => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUserRegistrationToken_AlreadyExpired => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUserRegistrationToken_IsNotApproved => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED
                            )
                        );
                    }
                    UserWorkflowPrecedent::ApplicationUserRegistrationToken_WrongValue => {
                        return Ok(
                            UnifiedReport::communication_code(
                                CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
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
impl RegisterByLastStep {
    pub async fn run_<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        request: Request<Body>,
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
        return WrappedActionCreator::create_for_json::<'_, _, _, _, Incoming, Outcoming>(
            environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            Self::run
        ).await;
    }
}