use super::creator::Creator;
use super::writer::Writer;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Method;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Route;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_StatusCode;
use crate::domain_layer::functionality::service::action_round_register__context___creator::ContextFrom;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert11;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::converter::Convert;
use crate::infrastructure_layer::functionality::service::converter::Converter;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;

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
        let action_round_register_status_code = match <Converter as Convert<u16, i16>>::convert(response.status().as_u16()) {
            Ok(action_round_register_status_code_) => ActionRoundRegister_StatusCode(action_round_register_status_code_),
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

        let action_round_register_route = ActionRoundRegister_Route(Cow::Borrowed(request.uri().path()));

        let action_round_register_method = ActionRoundRegister_Method(Cow::Borrowed(request.method().as_str()));

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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
            Insert11 {
                action_round_register_route,
                action_round_register_method,
                action_round_register_status_code,
                action_round_register_context,
            },
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
