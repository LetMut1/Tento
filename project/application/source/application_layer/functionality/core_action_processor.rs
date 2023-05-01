use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::functionality::service::writer::Writer;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::Response;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize as SerdeSerialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;

pub struct CoreActionProcessor;

impl CoreActionProcessor {
    pub async fn process<'a, SF, T, AP, F, API, APO, APRR>(
        environment_configuration: &'a EnvironmentConfiguration,
        mut request: Request<Body>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor: AP,
        action_processor_result_resolver: APRR
    ) -> Response
    where
        Serializer<SF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(
            &'a EnvironmentConfiguration,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>,
            API
        ) -> F,
        F: Future<Output = Result<ArgumentResult<ActionProcessorResult<APO>>, ErrorAuditor>>,
        API: for<'de> Deserialize<'de>,
        APO: SerdeSerialize,
        APRR: FnOnce(ActionProcessorResult<APO>) -> Result<UnifiedReport<APO>, ErrorAuditor>
    {
        if !RequestHeaderChecker::is_valid(&request) {
            let response = Creator::<Response>::create_bad_request();

            if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
                database_2_postgresql_connection_pool, &request, &response, &InvalidArgument::HttpHeaders
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                unreachable!(
                    "{}. TODO: Write in concurrent way. It is also necessary that the write                     // TODO CHANGE all occurences.
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

                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error__) = Writer::<ActionRoundRegister>::write_with_context(
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

        let action_processor_incoming = match Serializer::<SF>::deserialize::<'_, API>(bytes.chunk()) {
            Ok(action_processor_incoming_) => action_processor_incoming_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
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

        let action_processor_result = match action_processor(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            action_processor_incoming
        ).await {
            Ok(action_processor_result_) => action_processor_result_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
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
                let response = Creator::<Response>::create_bad_request();

                if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
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

        let response = match action_processor_result_resolver(action_processor_result_) {
            Ok(unified_report) => {
                let data = match Serializer::<SF>::serialize(&unified_report) {
                    Ok(data_) => data_,
                    Err(error) => {
                        let response = Creator::<Response>::create_internal_server_error();

                        if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
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

                let response = Creator::<Response>::create_ok(data);

                if let Err(mut error) = Writer::<ActionRoundRegister>::write(
                    database_2_postgresql_connection_pool, &request, &response
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    unreachable!(
                        "{}. TODO: Write in concurrent way. It is also necessary that the write
                        process does not wait for another write process, and writes immediately.",
                        &error
                    );
                }

                response
            },
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
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

                response
            }
        };

        return response;
    }
}