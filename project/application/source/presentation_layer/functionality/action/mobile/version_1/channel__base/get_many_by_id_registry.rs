use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_id_registry::ActionProcessor;
use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_id_registry::Incoming;
use crate::application_layer::functionality::service::action_round_result_writer::ActionRoundResultWriter;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::message_pack_encoder::MessagePackEncoder;
use crate::presentation_layer::data::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;


    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)
    // TODO ПРоставить в Хендлерах валидацию входящих параметров !!!!!!!!!!!!!!!!!!!!! ( Я проверил каждый экшн Авторизации, а сюда еще не дошли руки)


pub async fn get_many_by_id_registry<'a, T>(
    environment_configuration: &'a EnvironmentConfiguration,
    mut request: Request<Body>,
    database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    _redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    if !RequestHeaderChecker::is_valid(&request) {
        let response = ActionResponseCreator::create_bad_request();

        if let Err(mut error) = ActionRoundResultWriter::write_with_context(
            database_2_postgresql_connection_pool, &request, &response, &InvalidArgument::HttpHeaders
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            unreachable!(
                "{}. TODO: Write in concurrent way. It is also necessary that the write
                process does not wait for another write process, and writes immediately.",
                &error,
            );
        }

        return response;
    }

    let bytes = match to_bytes(request.body_mut()).await {
        Ok(bytes_) => bytes_,
        Err(error) => {
            let error_ = ErrorAuditor::new(
                BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                BacktracePart::new(line!(), file!(), None)
            );

            let response = ActionResponseCreator::create_internal_server_error();

            if let Err(mut error__) = ActionRoundResultWriter::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &error_
            ).await {
                error__.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error_,
                    &error__
                );
            }

            return response;
        }
    };

    let incoming = match MessagePackEncoder::decode::<'_, Incoming>(bytes.chunk()) {
        Ok(incoming_) => incoming_,
        Err(error) => {
            let response = ActionResponseCreator::create_internal_server_error();

            if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &error
            ).await {
                error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error,
                    &error_
                );
            }

            return response;
        }
    };

    let action_processor_result = match ActionProcessor::process(
        environment_configuration, database_1_postgresql_connection_pool, incoming
    ).await {
        Ok(action_processor_result_) => action_processor_result_,
        Err(error) => {
            let response = ActionResponseCreator::create_internal_server_error();

            if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &error
            ).await {
                error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error,
                    &error_
                );
            }

            return response;
        }
    };

    let action_processor_result_ = match action_processor_result {
        ArgumentResult::Ok { subject: action_processor_result__ } => action_processor_result__,
        ArgumentResult::InvalidArgument { invalid_argument } => {
            let response = ActionResponseCreator::create_bad_request();

            if let Err(mut error) = ActionRoundResultWriter::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &invalid_argument
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{}. TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error
                );
            }

            return response;
        }
    };

    match action_processor_result_ {
        ActionProcessorResult::Void => {
            let error = ErrorAuditor::new(
                BaseError::LogicError { message: "Unreachable state." },
                BacktracePart::new(line!(), file!(), None)
            );

            let response = ActionResponseCreator::create_not_extended();

            if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &error
            ).await {
                error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error,
                    &error_
                );
            }

            return response;
        }
        ActionProcessorResult::Outcoming { outcoming } => {
            let data = match MessagePackEncoder::encode(&UnifiedReport::data(outcoming)) {
                Ok(data_) => data_,
                Err(error) => {
                    let response = ActionResponseCreator::create_internal_server_error();

                    if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                        database_2_postgresql_connection_pool, &request, &response, &error
                    ).await {
                        error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        unreachable!(
                            "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                            process does not wait for another write process, and writes immediately.",
                            &error,
                            &error_
                        );
                    }

                    return response;
                }
            };

            let response = ActionResponseCreator::create_ok(data);

            if let Err(mut error) = ActionRoundResultWriter::write(database_2_postgresql_connection_pool, &request, &response).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{}. TODO: Write in concurrent way. It is also necessary that the write
                    process does not wait for another write process, and writes immediately.",
                    &error
                );
            }

            return response;
        }
        ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent } => {
            match user_workflow_precedent {
                UserWorkflowPrecedent::ApplicationUserAccessToken_AlreadyExpired => {
                    let data = match MessagePackEncoder::encode(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
                        )
                    ) {
                        Ok(data_) => data_,
                        Err(error) => {
                            let response = ActionResponseCreator::create_internal_server_error();

                            if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                                database_2_postgresql_connection_pool, &request, &response, &error
                            ).await {
                                error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                unreachable!(
                                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                                    process does not wait for another write process, and writes immediately.",
                                    &error,
                                    &error_
                                );
                            }

                            return response;
                        }
                    };

                    let response = ActionResponseCreator::create_ok(data);

                    if let Err(mut error) = ActionRoundResultWriter::write(database_2_postgresql_connection_pool, &request, &response).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        unreachable!(
                            "{}. TODO: Write in concurrent way. It is also necessary that the write
                            process does not wait for another write process, and writes immediately.",
                            &error
                        );
                    }

                    return response;
                }
                UserWorkflowPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                    let data = match MessagePackEncoder::encode(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
                        )
                    ) {
                        Ok(data_) => data_,
                        Err(error) => {
                            let response = ActionResponseCreator::create_internal_server_error();

                            if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                                database_2_postgresql_connection_pool, &request, &response, &error
                            ).await {
                                error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                unreachable!(
                                    "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                                    process does not wait for another write process, and writes immediately.",
                                    &error,
                                    &error_
                                );
                            }

                            return response;
                        }
                    };

                    let response = ActionResponseCreator::create_ok(data);

                    if let Err(mut error) = ActionRoundResultWriter::write(database_2_postgresql_connection_pool, &request, &response).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        unreachable!(
                            "{}. TODO: Write in concurrent way. It is also necessary that the write
                            process does not wait for another write process, and writes immediately.",
                            &error
                        );
                    }

                    return response;
                }
                _ => {
                    let error = ErrorAuditor::new(
                        BaseError::LogicError { message: "Unreachable state." },
                        BacktracePart::new(line!(), file!(), None)
                    );

                    let response = ActionResponseCreator::create_not_extended();

                    if let Err(mut error_) = ActionRoundResultWriter::write_with_context(
                        database_2_postgresql_connection_pool, &request, &response, &error
                    ).await {
                        error_.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        unreachable!(
                            "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                            process does not wait for another write process, and writes immediately.",
                            &error,
                            &error_
                        );
                    }

                    return response;
                }
            }
        }
    }
}