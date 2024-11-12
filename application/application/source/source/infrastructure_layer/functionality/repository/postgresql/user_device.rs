use super::Postgresql;
use crate::infrastructure_layer::functionality::repository::Repository;
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
    },
};
use super::PostgresqlPreparedStatementParameterStorage;
use dedicated_crate::void::Void;
use std::future::Future;
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl Repository<Postgresql<UserDevice>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<UserDevice, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            let query = "\
                INSERT INTO \
                    public.user_device AS ud (\
                        id,\
                        user__id\
                    ) VALUES (\
                        $1,\
                        $2\
                    ) \
                ON CONFLICT ON CONSTRAINT \
                    user_device2 \
                DO NOTHING;";
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &insert_1.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert_1.user__id,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
