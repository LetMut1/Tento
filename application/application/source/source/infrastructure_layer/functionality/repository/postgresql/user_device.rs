use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::user_device::UserDevice,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use dedicated_crate::void::Void;
use std::future::Future;
use tokio_postgres::{
    types::Type,
    Client as Connection,
};
impl PostgresqlRepository<UserDevice> {
    pub fn create_1<'a>(database_1_connection: &'a Connection, insert_1: Insert1) -> impl Future<Output = Result<UserDevice, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                INSERT INTO public.user_device AS ud ( \
                    id, \
                    user__id \
                ) VALUES ( \
                    $1, \
                    $2 \
                ) \
                ON CONFLICT ON CONSTRAINT user_device2 DO NOTHING;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.user_device__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user__id,
                    Type::INT8,
                );
            let statement = database_1_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_1_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(
                UserDevice::new(
                    insert_1.user_device__id,
                    insert_1.user__id,
                ),
            );
        };
    }
}
pub struct Insert1 {
    pub user_device__id: String,
    pub user__id: i64,
}
