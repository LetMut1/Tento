use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::ActionProcessor;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::Incoming;
use crate::application_layer::functionality::service::action_round_result_writer::ActionRoundResultWriter;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::rmp_serde;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::Outcoming;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::wrapped_encoding_protocol_action_creator::WrappedEncodingProtocolActionCreator;

pub async fn register_by_last_step<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
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

    let incoming = match rmp_serde::from_read_ref::<'_, [u8], Incoming>(bytes.chunk()) {
        Ok(incoming_) => incoming_,
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

    let action_processor_result = match ActionProcessor::process(
        environment_configuration_resolver, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, incoming
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

    match action_processor_result {
        ActionProcessorResult::Outcoming { outcoming } => {
            let data = match rmp_serde::to_vec(&UnifiedReport::data(outcoming)) {
                Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUser_InvalidPassword => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__INVALID_PASSWORD
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUser_InvalidNickname => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__INVALID_NICKNAME
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUser_InvalidEmail => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__INVALID_EMAIL
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUser_NicknameAlreadyExist => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__NICKNAME_ALREADY_EXIST
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUser_EmailAlreadyExist => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER__EMAIL_ALREADY_EXIST
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUserRegistrationToken_InvalidValue => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__INVALID_VALUE
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUserRegistrationToken_NotFound => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUserRegistrationToken_AlreadyExpired => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUserRegistrationToken_IsNotApproved => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED
                        )
                    ) {
                        Ok(data_) => data_,
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
                UserWorkflowPrecedent::ApplicationUserRegistrationToken_WrongValue => {
                    let data = match rmp_serde::to_vec(
                        &UnifiedReport::<Void>::communication_code(
                            CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
                        )
                    ) {
                        Ok(data_) => data_,
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
        ActionProcessorResult::InvalidArgument { invalid_argument } => {
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
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub async fn register_by_last_step_<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
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
        environment_configuration_resolver,
        request,
        database_1_postgresql_connection_pool,
        database_2_postgresql_connection_pool,
        redis_connection_pool,
        register_by_last_step
    ).await;
}