use super::creator::Creator;
use super::writer::Writer;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Method;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Route;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_StatusCode;
use crate::domain_layer::functionality::service::action_round_register__context_creator::ContextFrom;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::action_round_register__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::converter::Convert;
use crate::infrastructure_layer::functionality::service::converter::Converter;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

impl Writer<ActionRoundRegister<'_>> {
    pub async fn write<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request,
        response: &'a Response,
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if let Err(mut error) = Self::write_(
            database_2_postgresql_connection_pool,
            request,
            response,
            None,
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

            return Err(error);
        }

        return Ok(());
    }

    pub async fn write_with_context<'a, T, E>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request,
        response: &'a Response,
        subject: &'a E,
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        Creator<ActionRoundRegister_Context>: ContextFrom<E>,
    {
        let action_round_register_context = <Creator<ActionRoundRegister_Context> as ContextFrom<E>>::create(subject);

        if let Err(mut error) = Self::write_(
            database_2_postgresql_connection_pool,
            request,
            response,
            Some(action_round_register_context),
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

            return Err(error);
        }

        return Ok(());
    }

    async fn write_<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        request: &'a Request,
        response: &'a Response,
        action_round_register_context: Option<ActionRoundRegister_Context>,
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let action_round_register_status_code =
            match <Converter as Convert<u16, i16>>::convert(response.status().as_u16()) {
                Ok(action_round_register_status_code_) => {
                    ActionRoundRegister_StatusCode::new(action_round_register_status_code_)
                }
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

        let action_round_register_route = ActionRoundRegister_Route::new(Cow::Borrowed(request.uri().path()));

        let action_round_register_method = ActionRoundRegister_Method::new(Cow::Borrowed(request.method().as_str()));

        let insert = Insert {
            action_round_register_route,
            action_round_register_method,
            action_round_register_status_code,
            action_round_register_context,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(mut error) = PostgresqlRepository::<ActionRoundRegister>::create(
            &*database_2_postgresql_pooled_connection,
            insert,
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

            return Err(error);
        }

        return Ok(());
    }
}
