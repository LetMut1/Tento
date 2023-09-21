use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::functionality::service::writer::Writer;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::Response;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bytes::Buf;
use hyper::body::to_bytes;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;
use std::clone::Clone;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;

pub struct CommonActionProcessor;

impl CommonActionProcessor {
    pub async fn process<'a, SF, T, AP, F, API, APO, APP>(
        mut request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor: AP,
    ) -> Response
    where
        Serializer<SF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(&'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>, API) -> F,
        F: Future<Output = Result<InvalidArgumentResult<UnifiedReport<APO, APP>>, ErrorAuditor>>,
        API: for<'de> Deserialize<'de>,
        APO: SerdeSerialize,
        APP: SerdeSerialize,
    {
        if !Validator::<Request>::is_valid(&request) {
            let response = Creator::<Response>::create_bad_request();

            if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
                database_2_postgresql_connection_pool,
                &request,
                &response,
                &InvalidArgument::HttpHeader,
            )
            .await
            {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error__) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error_,
                )
                .await
                {
                    error__.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    unreachable!(
                        "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                        process does not wait for another write process, and writes immediately.",
                        &error_, &error__
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
                    &error,
                )
                .await
                {
                    error_.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    unreachable!(
                        "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                        process does not wait for another write process, and writes immediately.",
                        &error, &error_
                    );
                }

                return response;
            }
        };

        let unified_report = match action_processor(
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            action_processor_incoming,
        )
        .await
        {
            Ok(unified_report_) => unified_report_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                if let Err(mut error_) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &error,
                )
                .await
                {
                    error_.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    unreachable!(
                        "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                        process does not wait for another write process, and writes immediately.",
                        &error, &error_
                    );
                }

                return response;
            }
        };

        let unified_report_ = match unified_report {
            InvalidArgumentResult::Ok {
                subject: unified_report__,
            } => unified_report__,
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                let response = Creator::<Response>::create_bad_request();

                if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
                    database_2_postgresql_connection_pool,
                    &request,
                    &response,
                    &invalid_argument,
                )
                .await
                {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

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
                    &error,
                )
                .await
                {
                    error_.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    unreachable!(
                        "{} ({}). TODO: Write in concurrent way. It is also necessary that the write
                        process does not wait for another write process, and writes immediately.",
                        &error, &error_
                    );
                }

                return response;
            }
        };

        let response = Creator::<Response>::create_ok(data);

        if let Err(mut error) = Writer::<ActionRoundRegister>::write(
            database_2_postgresql_connection_pool,
            &request,
            &response,
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            unreachable!(
                "{}. TODO: Write in concurrent way. It is also necessary that the write
                process does not wait for another write process, and writes immediately.",
                &error
            );
        }

        return response;
    }
}
