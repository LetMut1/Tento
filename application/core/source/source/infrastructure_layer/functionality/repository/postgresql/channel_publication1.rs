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
    tokio_postgres::types::Type,
    dedicated::action_processor_incoming_outcoming::action_processor::channel_publication1::get_many::Data,
};
impl Repository<Postgresql<ChannelPublication1>> {
    pub fn create<'a>(database_3_client: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_publication1 AS cp1 (\
                        id,\
                        channel__id,\
                        images_pathes,\
                        text_,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5\
                    );";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &insert.channel_publication1__id,
                    Type::INT8,
                )
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
            return Result::Ok(());
        };
    }
    pub fn delete<'a>(database_3_client: &'a Client, by: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.channel_publication1 cp1 \
                WHERE \
                    cp1.id = $1;";
            let mut parameter_storage = ParameterStorage::new();
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
            crate::result_return_runtime!(
                database_3_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn find<'a>(database_1_client: &'a Client, by: By2, limit: i16) -> impl Future<Output = Result<Vec<Data>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    cp1.id AS i,\
                    cp1.images_pathes AS ip,\
                    cp1.text_ AS t,\
                    cp1.created_at AS ca \
                FROM \
                    public.channel_publication1 cp1 \
                WHERE \
                    cp1.channel__id = $1 \
                    AND cp1.created_at < $2 \
                ORDER BY \
                    cp1.created_at DESC \
                LIMIT $3";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__id,
                Type::INT8,
            );
            parameter_storage.add(
                &by.channel_publication1__created_at,
                Type::INT8,
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
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
            let mut data_registry: Vec<Data> = vec![];
            if rows.is_empty() {
                return Result::Ok(data_registry);
            }
            '_a: for row in rows.iter() {
                data_registry.push(
                    Data {
                        channel_publication1__id: crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        channel_publication1__images_pathes: crate::result_return_logic!(rows[0].try_get::<'_, usize, Vec<String>>(1)),
                        channel_publication1__text: crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(2)),
                        channel_publication1__created_at: crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                    }
                );
            }
            return Result::Ok(data_registry);
        };
    }
}
pub struct Insert<'a> {
    pub channel_publication1__id: i64,
    pub channel__id: i64,
    pub channel_publication1__images_pathes: &'a [String],
    pub channel_publication1__text: Option<&'a str>,
    pub channel_publication1__created_at: i64,
}
pub struct By1 {
    pub channel_publication1__id: i64,
}
pub struct By2 {
    pub channel__id: i64,
    pub channel_publication1__created_at: i64,
}