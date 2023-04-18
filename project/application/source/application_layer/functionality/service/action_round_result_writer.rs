use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::functionality::service::action_round_register__context_creator::CreateContext;
use crate::domain_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::action_round_register__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::converter::Convert;
use crate::infrastructure_layer::functionality::service::converter::Converter;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Body;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionRoundResultWriter;

impl ActionRoundResultWriter {
    pub async fn write<'a, T>(
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
        if let Err(mut error) = Self::write_(database_2_postgresql_connection_pool, request, response, None).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn write_with_context<'a, T, E>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        subject: &'a E
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        Creator<ActionRoundRegister_Context>: CreateContext<E>
    {
        if let Err(mut error) = Self::write_(
            database_2_postgresql_connection_pool,
            request,
            response,
            Some(<Creator<ActionRoundRegister_Context> as CreateContext<E>>::create(subject))
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn write_<'a, T>(
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

        let insert = Insert {
            action_round_register_route: request.uri().path(),
            action_round_register_method: request.method().as_str(),
            action_round_register_status_code,
            action_round_register_context
        };

        if let Err(mut error) = PostgresqlRepository::<ActionRoundRegister>::create(
            &*database_2_postgresql_pooled_connection,
            insert
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}