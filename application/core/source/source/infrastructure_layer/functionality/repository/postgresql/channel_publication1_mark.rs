use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_publication1_mark::ChannelPublication1Mark,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelPublication1Mark>> {
    pub fn create<'a>(database_3_client: &'a Client, insert: Insert) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_publication1_mark AS cp1m (\
                        user__id,\
                        channel_publication1__id,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    )\
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
                    &insert.channel_publication1_mark__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_3_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_3_client
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
    pub fn delete<'a>(database_3_client: &'a Client, by: By) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.channel_publication1_mark cp1m \
                WHERE \
                    cp1m.user__id = $1 \
                    AND cp1m.channel_publication1__id = $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_3_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_3_client
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
pub struct Insert {
    pub user__id: i64,
    pub channel_publication1__id: i64,
    pub channel_publication1_mark__created_at: i64,
}
pub struct By {
    pub user__id: i64,
    pub channel_publication1__id: i64,
}
