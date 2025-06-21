use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_publication1_marked_view::ChannelPublication1MarkedView,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelPublication1MarkedView>> {
    pub fn create_1<'a>(client_database_3: &'a Client, insert: Insert) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.channel_publication1_marked_view (\
                        user__id,\
                        channel_publication1__id,\
                        marked_at,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(4);
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
                    &insert.channel_publication1_marked_view__marked_at,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_marked_view__created_at,
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
    pub fn create_2<'a>(client_database_3: &'a Client, insert: Insert, by: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.channel_publication1_marked_view AS cp1mv (\
                        user__id,\
                        channel_publication1__id,\
                        marked_at,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4\
                    ) \
                ON CONFLICT ON CONSTRAINT channel_publication1_marked_view_2 DO UPDATE \
                SET \
                    marked_at = EXCLUDED.marked_at \
                WHERE \
                    cp1mv.marked_at = $5 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(5);
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
                    &insert.channel_publication1_marked_view__marked_at,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_marked_view__created_at,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_marked_view__marked_at,
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
    pub fn update<'a>(client_database_3: &'a Client, updated: Update, by: By2) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.channel_publication1_marked_view AS cp1mv \
                SET \
                    marked_at = $1 \
                WHERE \
                    cp1mv.user__id = $2 \
                    AND cp1mv.channel_publication1__id = $3 \
                    AND cp1mv.marked_at != $4 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(4);
            parameter_storage
                .add(
                    &updated.channel_publication1_marked_view__marked_at,
                    Type::INT8,
                )
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1__id,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_marked_view__marked_at,
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
    pub channel_publication1_marked_view__marked_at: i64,
    pub channel_publication1_marked_view__created_at: i64,
}
pub struct By1 {
    pub channel_publication1_marked_view__marked_at: i64,
}
pub struct By2 {
    pub user__id: i64,
    pub channel_publication1__id: i64,
    pub channel_publication1_marked_view__marked_at: i64,
}
pub struct Update {
    pub channel_publication1_marked_view__marked_at: i64,
}