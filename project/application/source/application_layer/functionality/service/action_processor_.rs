use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::functionality::service::action_round_result_writer::ActionRoundResultWriter;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use std::future::Future;
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
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize as SerdeSerialize;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, SF, T, AP, F1, API, APO, APRR, F2>(
        environment_configuration: &'a EnvironmentConfiguration,
        mut request: Request<Body>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor: AP,
        action_processor_result_resolver: APRR
    ) -> Response<Body>
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
        ) -> F1,
        F1: Future<Output = Result<ArgumentResult<ActionProcessorResult<APO>>, ErrorAuditor>>,
        API: for<'de> Deserialize<'de>,
        APO: SerdeSerialize,
        APRR: FnOnce(ArgumentResult<ActionProcessorResult<APO>>) -> F2,
        F2: Future<Output = Result<Response<Body>, ErrorAuditor>>
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

        let action_processor_incoming = match Serializer::<SF>::deserialize::<'_, API>(bytes.chunk()) {
            Ok(action_processor_incoming_) => action_processor_incoming_,
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

        let action_processor_result = match action_processor(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            action_processor_incoming
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

        let response = match action_processor_result_resolver(action_processor_result).await {
            Ok(response_) => response_,
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

        return response;
    }
}