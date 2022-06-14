use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_exception::JsonAccessWebTokenWorkflowException;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_new_for_context::base::Base as ActionHandlerIncomingDataGetManyByCreatedAt;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::_new_for_context::base::Base as ActionHandlerIncomingDataGetManyByIdRegistry;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_new_for_context::base::Base as ActionHandlerIncomingDataGetManyByName;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_new_for_context::base::Base as ActionHandlerIncomingDataGetManyBySubscribersQuantity;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as ActionHandlerGetManyByCreatedAt;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as ActionHandlerGetManyByIdRegistry;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as ActionHandlerGetManyByName;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as ActionHandlerGetManyBySubscribersQuantity;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::service::unified_report_creator::UnifiedReportCreator;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::Request;
use hyper::Response;
use std::clone::Clone;
use std::convert::From;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn get_many_by_name<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
        
        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataGetManyByName>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerGetManyByName::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));
                
                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("TODO");
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_base_error() {
                            BaseError::InvalidArgumentError => {
                                return ActionResponseCreator::create_bad_request();
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

    pub async fn get_many_by_created_at<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataGetManyByCreatedAt>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerGetManyByCreatedAt::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));
                
                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("TODO");
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_base_error() {
                            BaseError::InvalidArgumentError => {
                                return ActionResponseCreator::create_bad_request();
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
    
    pub async fn get_many_by_subscribers_quantity<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataGetManyBySubscribersQuantity>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerGetManyBySubscribersQuantity::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));
                
                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("TODO");
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_base_error() {
                            BaseError::InvalidArgumentError => {
                                return ActionResponseCreator::create_bad_request();
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

    pub async fn get_many_by_id_registry<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataGetManyByIdRegistry>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerGetManyByIdRegistry::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));
                
                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
                                                ) {
                                                    Ok(data) => {
                                                        return ActionResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("TODO");
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_base_error() {
                            BaseError::InvalidArgumentError => {
                                return ActionResponseCreator::create_bad_request();
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
}