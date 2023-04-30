use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::ActionProcessor;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::Incoming;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::Outcoming;
use crate::application_layer::functionality::service::core_action_processor::CoreActionProcessor;
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
use extern_crate::hyper::Response;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::wrapped_encoding_protocol_action_creator::WrappedEncodingProtocolActionCreator;

pub async fn authorize_by_first_step<'a, T>(
    environment_configuration: &'a EnvironmentConfiguration,
    request: Request<Body>,
    database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
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
        resolve
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
        ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent } => {
            match user_workflow_precedent {
                UserWorkflowPrecedent::ApplicationUser_NotFound |
                UserWorkflowPrecedent::ApplicationUser_WrongPassword => {
                    return Ok(
                        UnifiedReport::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
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

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub async fn authorize_by_first_step_<'a, T>(
    environment_configuration: &'a EnvironmentConfiguration,
    request: Request<Body>,
    database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    return WrappedEncodingProtocolActionCreator::create_for_json::<'_, _, _, _, Incoming, Outcoming>(
        environment_configuration,
        request,
        database_1_postgresql_connection_pool,
        database_2_postgresql_connection_pool,
        redis_connection_pool,
        authorize_by_first_step
    ).await;
}