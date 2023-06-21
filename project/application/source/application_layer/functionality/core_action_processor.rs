use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::functionality::service::writer::Writer;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::Response;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::body::to_bytes;
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
    pub async fn process<'a, SF, T, AP, F, API, APO, APP>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        mut request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor: AP
    ) -> Response
    where
        Serializer<SF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(
            &'a PushableEnvironmentConfiguration,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>,
            API
        ) -> F,
        F: Future<Output = Result<InvalidArgumentResult<UnifiedReport<APO, APP>>, ErrorAuditor>>,
        API: for<'de> Deserialize<'de>,
        APO: SerdeSerialize,
        APP: SerdeSerialize
    {
        if !Validator::<Request>::is_valid(&request) {
            let response = Creator::<Response>::create_bad_request();

            if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
                database_2_postgresql_connection_pool,
                &request,
                &response,
                &InvalidArgument::HttpHeaders
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
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error_
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
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error
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

        let unified_report = match action_processor(
            pushable_environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            action_processor_incoming
        ).await {
            Ok(unified_report_) => unified_report_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error
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

        let unified_report_ = match unified_report {
            InvalidArgumentResult::Ok { subject: unified_report__ } => unified_report__,
            InvalidArgumentResult::InvalidArgument { invalid_argument } => {
                let response = Creator::<Response>::create_bad_request();

                if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &invalid_argument
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

        let data = match Serializer::<SF>::serialize(&unified_report_) {
            Ok(data_) => data_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error
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
            database_2_postgresql_connection_pool,
            &request,
            &response
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