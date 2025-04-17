use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_publication1_view::ChannelPublication1View,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelPublication1View>> {
    pub fn create<'a>(client_database_3: &'a Client, insert: Insert) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.channel_publication1_view (\
                        user__id,\
                        channel_publication1__id,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
                .add(
                    &insert.user__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_view__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_3
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
pub struct Insert {
    pub user__id: i64,
    pub channel_publication1__id: i64,
    pub channel_publication1_view__created_at: i64,
}
