use crate::domain_layer::functionality::service::action_round_register__context_creator::ActionRoundRegister_ContextCreator;
use crate::domain_layer::functionality::service::action_round_register__context_creator::ActionRoundRegister_CreateContext;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::action_round_register__creator::ActionRoundRegister_Creator;
use crate::infrastructure_layer::functionality::service::converter::Convert;
use crate::infrastructure_layer::functionality::service::converter::Converter;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::http::header;
use extern_crate::hyper::Body;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionRoundLogger;

impl ActionRoundLogger {
    pub async fn log<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log_(database_2_postgresql_connection_pool, request, response, None).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn log_invalid_argument<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        invalid_argment: InvalidArgument
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log_(
            database_2_postgresql_connection_pool,
            request, response,
            Some(ActionRoundRegister_ContextCreator::create(&invalid_argment)),
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn log_error_auditor<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: &'a ErrorAuditor
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log_(
            database_2_postgresql_connection_pool,
            request,
            response,
            Some(ActionRoundRegister_ContextCreator::create(error_auditor))
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn log_<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        action_round_register_context: Option<String>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        // let message = match error_auditor {
        //     Some(error_auditor_) => {
        //         match request.headers().get(header::USER_AGENT) {
        //             Some(header_value) => {
        //                 let header_value_ = match header_value.to_str() {
        //                     Ok(header_value__) => header_value__,
        //                     Err(error) => {
        //                         return Err(
        //                             ErrorAuditor::new(
        //                                 BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
        //                                 BacktracePart::new(line!(), file!(), None)
        //                             )
        //                         );
        //                     }
        //                 };

        //                 format!(
        //                     "{} {} {} {} - {}",
        //                     request.uri().path(),
        //                     request.method().as_str(),
        //                     response.status().as_u16(),
        //                     header_value_,
        //                     error_auditor_
        //                 )
        //             }
        //             None => {
        //                 format!(
        //                     "{} {} {} - {}",
        //                     request.uri().path(),
        //                     request.method().as_str(),
        //                     response.status().as_u16(),
        //                     error_auditor_
        //                 )
        //             }
        //         }
        //     }
        //     None => {
        //         format!(
        //             "{} {} {}",
        //             request.uri().path(),
        //             request.method().as_str(),
        //             response.status().as_u16(),
        //         )
        //     }
        // };  DELETE

        let action_round_register_status_code = match Converter::<u16, i16>::convert(response.status().as_u16()) {
            Ok(action_round_register_status_code_) => action_round_register_status_code_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(mut error) = ActionRoundRegister_Creator::create(
            &*database_2_postgresql_pooled_connection, request.uri().path(), action_round_register_status_code, action_round_register_context
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}