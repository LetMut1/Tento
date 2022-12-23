use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as ActionHandlerGetManyByCreatedAt;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Incoming as IncomingGetManyByCreatedAt;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as ActionHandlerGetManyByIdRegistry;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Incoming as IncomingGetManyByIdRegistry;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as ActionHandlerGetManyByName;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Incoming as IncomingGetManyByName;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as ActionHandlerGetManyBySubscribersQuantity;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Incoming as IncomingGetManyBySubscribersQuantity;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::functionality::service::unified_report_creator::UnifiedReportCreator;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::HttpBody;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::rmp_serde;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::convert::From;
use std::marker::Send;
use std::marker::Sync;

pub struct Base;

impl Base {



    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)











    pub async fn get_many_by_name<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingGetManyByName>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerGetManyByName::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
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
                                    EntityWorkflowException::ApplicationUserAccessTokenWorkflowException { application_user_access_token_workflow_exception } => {
                                        match application_user_access_token_workflow_exception {
                                            ApplicationUserAccessTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingGetManyByCreatedAt>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerGetManyByCreatedAt::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
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
                                    EntityWorkflowException::ApplicationUserAccessTokenWorkflowException { application_user_access_token_workflow_exception } => {
                                        match application_user_access_token_workflow_exception {
                                            ApplicationUserAccessTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingGetManyBySubscribersQuantity>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerGetManyBySubscribersQuantity::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
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
                                    EntityWorkflowException::ApplicationUserAccessTokenWorkflowException { application_user_access_token_workflow_exception } => {
                                        match application_user_access_token_workflow_exception {
                                            ApplicationUserAccessTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingGetManyByIdRegistry>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerGetManyByIdRegistry::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
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
                                    EntityWorkflowException::ApplicationUserAccessTokenWorkflowException { application_user_access_token_workflow_exception } => {
                                        match application_user_access_token_workflow_exception {
                                            ApplicationUserAccessTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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