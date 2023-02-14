use crate::domain_layer::data::entity::system_registry::Level;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::system_registry__creator::SystemRegistry_Creator;
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
    pub async fn log_info<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: Option<ErrorAuditor>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log(Level::Info, database_2_postgresql_connection_pool, request, response, error_auditor).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn log_error<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: Option<ErrorAuditor>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log(Level::Error, database_2_postgresql_connection_pool, request, response, error_auditor).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn log_fatal_error<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: Option<ErrorAuditor>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if let Err(mut error) = Self::log(Level::FatalError, database_2_postgresql_connection_pool, request, response, error_auditor).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn log<'a, T>(
        level: Level,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: Option<ErrorAuditor>
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let message = match error_auditor {
            Some(error_auditor_) => {
                match request.headers().get(header::USER_AGENT) {
                    Some(header_value) => {
                        todo!();













                    }
                    None => {
                        format!(
                            "{} {} {} - {}",
                            request.uri().path(),
                            request.method().as_str(),
                            response.status().as_u16(),
                            &error_auditor_
                        )
                    }
                }
            }
            None => {
                format!(
                    "{} {} {}",
                    request.uri().path(),
                    request.method().as_str(),
                    response.status().as_u16(),
                )
            }
        };

        if let Err(mut error) = SystemRegistry_Creator::create(database_2_postgresql_connection_pool, message, level).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}