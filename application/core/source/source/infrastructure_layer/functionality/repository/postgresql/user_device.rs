use {
    crate::{
        domain_layer::data::entity::user_device::UserDevice,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    super::{
        Postgresql,
        ParameterStorage,
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<UserDevice>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<UserDevice, AggregateError>> + Send + use<'a> {
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
                    user_device2 \
                DO NOTHING;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &insert_1.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert_1.user__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameter_type_registry(),
                )
                .await
            );
            crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameter_registry(),
                )
                .await
            );
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
