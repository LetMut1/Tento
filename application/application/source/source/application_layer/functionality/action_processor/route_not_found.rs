use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::functionality::service::writer::Writer;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::Response;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use http::request::Parts;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use crate::infrastructure_layer::data::control_type::RouteNotFound;

impl ActionProcessor<RouteNotFound> {
    pub async fn process<'a, T>(
        parts: &'a Parts,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let response = Creator::<Response>::create_not_found();

        // if let Err(mut error) = Writer::<ActionRoundRegister>::write_with_context(
        //     database_2_postgresql_connection_pool,
        //     &request,
        //     &response,
        //     &InvalidArgument::HttpRoute,
        // )
        // .await
        // {
        //     error.add_backtrace_part(
        //         BacktracePart::new(
        //             line!(),
        //             file!(),
        //             None,
        //         ),
        //     );

        //     unreachable!(
        //         "{}. TODO: Write in concurrent way. It is also necessary that the write
        //         process does not wait for another write process, and writes immediately.",
        //         &error
        //     );
        // }

        return response;
    }
}
