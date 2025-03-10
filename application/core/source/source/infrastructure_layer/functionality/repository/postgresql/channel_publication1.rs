use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_publication1::ChannelPublication1,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::{
        types::Type,
        Row,
    },
};
impl Repository<Postgresql<ChannelPublication1>> {
    // channel_publication1__id: i64
    pub fn create<'a, 'b>(database_3_client: &'a Client, insert: Insert<'a, 'b>) -> impl Future<Output = Result<i64, AggregateError>> + Send + use<'a, 'b> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_publication1 AS cp1 (\
                        id,\
                        channel__id,\
                        images_pathes,\
                        text_,\
                        marks_quantity,\
                        viewing_quantity,\
                        created_at\
                    ) VALUES (\
                        nextval('public.channel_publication1_1'),\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6\
                    )\
                RETURNING \
                    u.id AS i;";
            let mut parameter_storage = ParameterStorage::new(6);
            parameter_storage
                .add(
                    &insert.channel__id,
                    Type::TEXT,
                )
                .add(
                    &insert.channel_publication1__images_pathes,
                    Type::TEXT,
                )
                .add(
                    &insert.channel_publication1__text,
                    Type::TEXT,
                )
                .add(
                    &insert.channel_publication1__marks_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1__viewing_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel_publication1__created_at,
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
            return Result::Ok(
                crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0))
            );
        };
    }
    pub fn delete<'a>(database_3_client: &'a Client, by: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.channel_publication1 cp1 \
                WHERE \
                    cp1.id = $1 \
                RETURNING \
                    true AS v;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel_publication1__id,
                Type::TEXT,
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
    pub fn find_1<'a>(database_3_client: &'a Client, by: By2, limit: i16) -> impl Future<Output = Result<Vec<Row>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    cp1.id AS i,\
                    cp1.images_pathes AS ip,\
                    cp1.text_ AS t,\
                    cp1.marks_quantity AS mq,\
                    cp1.viewing_quantity AS vq,\
                    cp1.created_at AS ca \
                FROM \
                    public.channel_publication1 cp1 \
                WHERE \
                    cp1.channel__id = $1 \
                    AND cp1.created_at < $2 \
                ORDER BY \
                    cp1.created_at DESC \
                LIMIT $3";
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
            .add(
                &by.channel__id,
                Type::INT8,
            )
            .add(
                &by.channel_publication1__created_at,
                Type::INT8,
            )
            .add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_3_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            return crate::result_into_runtime!(
                database_3_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
        };
    }
    // channel__id: i64
    pub fn find_2<'a>(database_3_client: &'a Client, by: By1) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    cp1.channel__id AS ci \
                FROM \
                    public.channel_publication1 cp1 \
                WHERE \
                    cp1.id = $1";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                ),
            );
        };
    }
}
pub struct Insert<'a, 'b> {
    pub channel__id: i64,
    pub channel_publication1__images_pathes: &'a [&'b str],
    pub channel_publication1__text: Option<&'a str>,
    pub channel_publication1__marks_quantity: i64,
    pub channel_publication1__viewing_quantity: i64,
    pub channel_publication1__created_at: i64,
}
pub struct By1 {
    pub channel_publication1__id: i64,
}
pub struct By2 {
    pub channel__id: i64,
    pub channel_publication1__created_at: i64,
}