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
    pub fn create<'a>(client_database_4: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.channel_publication1_commentary AS cp1c (\
                        id,\
                        author,\
                        channel_publication1__id,\
                        text_,\
                        marks_quantity,\
                        created_at\
                    ) VALUES (\
                        nextval('public.channel_publication1_commentary_1'),\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    cp1c.id AS i;";
            let channel_publication1_commentary__marks_quantity = insert.channel_publication1_commentary__marks_quantity as i64;
            let mut parameter_storage = ParameterStorage::new(5);
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
                    &channel_publication1_commentary__marks_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1_commentary__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_4
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_4
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
    pub fn update_1<'a>(client_database_3: &'a Client, by: By2) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.channel_publication1_commentary AS cp1c \
                SET \
                    marks_quantity = marks_quantity + 1 \
                WHERE \
                    cp1c.id = $1 \
                    AND cp1c.marks_quantity < $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.channel_publication1_commentary__id,
                    Type::INT8,
                )
                .add(
                    &(u32::MAX as i64),
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
    pub fn update_2<'a>(client_database_3: &'a Client, by: By2) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.channel_publication1_commentary AS cp1c \
                SET \
                    marks_quantity = marks_quantity - 1 \
                WHERE \
                    cp1c.id = $1 \
                    AND cp1c.marks_quantity > $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.channel_publication1_commentary__id,
                    Type::INT8,
                )
                .add(
                    &(u32::MIN as i64),
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
    pub fn delete<'a>(client_database_4: &'a Client, by: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                DELETE FROM ONLY \
                    public.channel_publication1_commentary cp1c \
                WHERE \
                    cp1c.id = $1 \
                    AND cp1c.author = $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.channel_publication1_commentary__id,
                    Type::INT8,
                )
                .add(
                    &by.channel_publication1_commentary__author,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_4
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_4
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
    pub channel_publication1_commentary__marks_quantity: u32,
    pub channel_publication1_commentary__created_at: i64,
}
pub struct By1 {
    pub channel_publication1_commentary__id: i64,
    pub channel_publication1_commentary__author: i64,
}
pub struct By2 {
    pub channel_publication1_commentary__id: i64,
}
