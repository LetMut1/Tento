use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::user_device::UserDevice,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<UserDevice>> {
    pub fn create<'a>(database_1_client: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
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
                    user_device_2 \
                DO NOTHING \
                RETURNING \
                    true AS v;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &insert.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert.user__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(());
        };
    }
}
pub struct Insert<'a> {
    pub user_device__id: &'a str,
    pub user__id: i64,
}
