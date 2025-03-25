use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_publication1_commentary::ChannelPublication1Commentary,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelPublication1Commentary>> {
    // channel_publication1_commentary__id: i64,
    pub fn create<'a>(database_4_client: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_publication1_commentary AS cp1c (\
                        id,\
                        author,\
                        channel_publication1__id,\
                        text_,\
                        marks_quantity,\
                        created_at,\
                        is_predeleted,\
                        can_be_deleted_from\
                    ) VALUES (\
                        nextval('public.channel_publication1_commentary_1'),\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6,\
                        $7\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    cp1c.id AS i;";
            let mut parameter_storage = ParameterStorage::new(7);
            parameter_storage
                .add(
                    &insert.channel_publication1_commentary__author,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_commentary__text,
                    Type::TEXT,
                )
                .add(
                    &insert.channel_publication1_commentary__marks_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_commentary__created_at,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_commentary__is_predeleted,
                    Type::BOOL,
                )
                .add(
                    &insert.channel_publication1_commentary__can_be_deleted_from,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_4_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_4_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(Option::None);
            }
            return Result::Ok(Option::Some(crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0))));
        };
    }
    pub fn update<'a>(database_4_client: &'a Client, update: Update, by: By) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.channel_publication1_commentary AS cp1c \
                SET (\
                    is_predeleted,\
                    can_be_deleted_from\
                ) = ROW(\
                    $1,\
                    $2\
                ) \
                WHERE \
                    cp1c.id = $3 \
                    AND cp1c.author = $4 \
                    AND cp1c.is_predeleted = $5 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(5);
            parameter_storage
                .add(
                    &update.channel_publication1_commentary__is_predeleted,
                    Type::BOOL,
                )
                .add(
                    &update.channel_publication1_commentary__can_be_deleted_from,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_commentary__id,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_commentary__author,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_commentary__is_predeleted,
                    Type::BOOL,
                );
            let statement = crate::result_return_logic!(
                database_4_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_4_client
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
    pub channel_publication1_commentary__author: i64,
    pub channel_publication1__id: i64,
    pub channel_publication1_commentary__text: &'a str,
    pub channel_publication1_commentary__marks_quantity: i64,
    pub channel_publication1_commentary__created_at: i64,
    pub channel_publication1_commentary__is_predeleted: bool,
    pub channel_publication1_commentary__can_be_deleted_from: i64,
}
pub struct Update {
    pub channel_publication1_commentary__is_predeleted: bool,
    pub channel_publication1_commentary__can_be_deleted_from: i64,
}
pub struct By {
    pub channel_publication1_commentary__author: i64,
    pub channel_publication1_commentary__id: i64,
    pub channel_publication1_commentary__is_predeleted: bool,
}
