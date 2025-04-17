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
    pub fn create<'a>(client_database_1: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.user_device (\
                        id,\
                        user__id\
                    ) VALUES (\
                        $1,\
                        $2\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
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
                client_database_1
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_1
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
}
pub struct Insert<'a> {
    pub user_device__id: &'a str,
    pub user__id: i64,
}
