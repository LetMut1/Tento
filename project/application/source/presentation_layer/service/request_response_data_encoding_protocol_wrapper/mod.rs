use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::body::to_bytes;
use hyper::Request;
use hyper::Response;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::ops::FnOnce;
use super::action_response_creator::ActionResponseCreator;
use super::request_header_checker::RequestHeaderChecker;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing_::base::Base as ActionHandlerCheckNicknameForExisting_;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing_::base::_new_for_context::base::Base as ActionHandlerIncomingDataCheckNicknameForExisting_;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as ActionHandlerIncomingDataCheckNicknameForExisting;

pub struct RequestResponseDataEncodingProtocolWrapper;

impl RequestResponseDataEncodingProtocolWrapper {
    pub async fn wrap_to_json<'a, 'b, T, FO, F>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        wrapped_action: FO
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        FO: FnOnce(
            &'b EnvironmentConfigurationResolver,
            Request<Body>,
            Pool<PostgresqlConnectionManager<T>>,
            Pool<RedisConnectionManager>
        ) -> F,
        F : Future<Output = Response<Body>>
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }

        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, ActionHandlerIncomingDataCheckNicknameForExisting>(bytes.chunk()) {
                    Ok(action_handler_incoming_data) => {
                        match ActionHandlerCheckNicknameForExisting_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            ActionHandlerIncomingDataCheckNicknameForExisting_::new(request_parts, action_handler_incoming_data)
                        ).await {
                            Ok(action_handler_result) => {
                                match action_handler_result {
                                    ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                        let (
                                            response_parts,
                                            convertible_data
                                        ) = action_handler_outcoming_data.into_inner();
        
                                        match convertible_data {
                                            Some(unified_report) => {
                                                match serde_json::to_vec(&unified_report) {
                                                    Ok(data) => {
                                                        return Response::from_parts(response_parts, Body::from(data));
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            None => {
                                                return Response::from_parts(response_parts, Body::empty());
                                            }
                                        }
                                    }
                                    ActionHandlerResult::EntityWorkflowException { entity_workflow_exception: _ } => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_base_error() {
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("TODO");
                                    }
                                    BaseError::LogicError { logic_error: _ } |
                                    BaseError::RunTimeError { run_time_error: _ } => {
                                        // log::error!("{}", error);
                
                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ActionResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        }
    }
}