use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel::Channel,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::{
        borrow::Cow,
        future::Future,
    },
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<Channel<'_>>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert: Insert1) -> impl Future<Output = Result<Channel<'static>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel AS c (\
                        id,\
                        owner,\
                        name,\
                        linked_name,\
                        description,\
                        access_modifier,\
                        visability_modifier,\
                        orientation,\
                        cover_image_path,\
                        background_image_path,\
                        subscribers_quantity,\
                        marks_quantity,\
                        viewing_quantity,\
                        created_at\
                    ) VALUES (\
                        nextval('public.channel1'), \
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6,\
                        $7,\
                        $8,\
                        $9,\
                        $10,\
                        $11,\
                        $12,\
                        $13\
                    ) \
                RETURNING \
                    c.id AS i;";
            let channel__description = insert.channel__description.as_ref();
            let channel__orientation = insert.channel__orientation.as_slice();
            let channel__cover_image_path = insert.channel__cover_image_path.as_ref();
            let channel__background_image_path = insert.channel__background_image_path.as_ref();
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &insert.channel__owner,
                    Type::INT8,
                )
                .add(
                    &insert.channel__name,
                    Type::TEXT,
                )
                .add(
                    &insert.channel__linked_name,
                    Type::TEXT,
                )
                .add(
                    &channel__description,
                    Type::TEXT,
                )
                .add(
                    &insert.channel__access_modifier,
                    Type::INT2,
                )
                .add(
                    &insert.channel__visability_modifier,
                    Type::INT2,
                )
                .add(
                    &channel__orientation,
                    Type::INT2_ARRAY,
                )
                .add(
                    &channel__cover_image_path,
                    Type::TEXT,
                )
                .add(
                    &channel__background_image_path,
                    Type::TEXT,
                )
                .add(
                    &insert.channel__subscribers_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel__marks_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel__viewing_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel__created_at,
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
            return Result::Ok(
                Channel::new(
                    crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                    insert.channel__owner,
                    Cow::Owned(insert.channel__name),
                    insert.channel__linked_name,
                    insert.channel__description,
                    insert.channel__access_modifier,
                    insert.channel__visability_modifier,
                    insert.channel__orientation,
                    insert.channel__cover_image_path,
                    insert.channel__background_image_path,
                    insert.channel__subscribers_quantity,
                    insert.channel__marks_quantity,
                    insert.channel__viewing_quantity,
                    insert.channel__created_at,
                ),
            );
        };
    }
    pub fn update_1<'a>(database_1_client: &'a Client, by: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.channel AS c \
                SET \
                    subscribers_quantity = subscribers_quantity + 1 \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__id,
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
            crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn find_1<'a>(database_1_client: &'a Client, by: By1) -> impl Future<Output = Result<Option<Channel<'static>>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.owner AS ow,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.description AS d,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.orientation AS or,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    c.subscribers_quantity,\
                    c.marks_quantity AS mq,\
                    c.viewing_quantity AS vq,\
                    c.created_at AS ca \
                FROM \
                    public.channel c \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__id,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    Channel::new(
                        by.channel__id,
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        Cow::Owned(
                            crate::result_return_logic!(
                                rows[0].try_get::<'_, usize, String>(1)
                            ),
                        ),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(3)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(4)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(5)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Vec<i16>>(6)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(7)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(8)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(9)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(10)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(11)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(12)),
                    ),
                ),
            );
        };
    }
    pub fn find_2<'a, 'b>(database_1_client: &'a Client, by: By2<'b>) -> impl Future<Output = Result<Option<Channel<'b>>, AggregateError>> + Send + use<'a, 'b> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i,\
                    c.owner AS ow,\
                    c.linked_name AS ln,\
                    c.description AS d,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.orientation AS or,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    c.subscribers_quantity,\
                    c.marks_quantity AS mq,\
                    c.viewing_quantity AS vq,\
                    c.created_at AS ca \
                FROM \
                    public.channel c \
                WHERE \
                    c.name = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__name,
                Type::TEXT,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    Channel::new(
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(1)),
                        Cow::Borrowed(by.channel__name),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(3)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(4)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(5)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Vec<i16>>(6)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(7)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(8)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(9)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(10)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(11)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(12)),
                    ),
                ),
            );
        };
    }
    pub fn find_3<'a>(database_1_client: &'a Client, by: By1) -> impl Future<Output = Result<Option<Channel<'static>>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.owner AS ow,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.description AS d,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.orientation AS or,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    c.subscribers_quantity,\
                    c.marks_quantity AS mq,\
                    c.viewing_quantity AS vq,\
                    c.created_at AS ca \
                FROM \
                    public.channel c \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__id,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    Channel::new(
                        by.channel__id,
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        Cow::Owned(
                            crate::result_return_logic!(
                                rows[0].try_get::<'_, usize, String>(1)
                            ),
                        ),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(3)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(4)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(5)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Vec<i16>>(6)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(7)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(8)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(9)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(10)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(11)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(12)),
                    ),
                ),
            );
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.name = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__name,
                Type::TEXT,
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
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
    pub fn is_exist_2<'a>(database_1_client: &'a Client, by: By3<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.linked_name = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.channel__linked_name,
                Type::TEXT,
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
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
}
pub struct Insert1 {
    pub channel__owner: i64,
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__description: Option<String>,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
    pub channel__orientation: Vec<i16>,
    pub channel__cover_image_path: Option<String>,
    pub channel__background_image_path: Option<String>,
    pub channel__subscribers_quantity: i64,
    pub channel__marks_quantity: i64,
    pub channel__viewing_quantity: i64,
    pub channel__created_at: i64,
}
pub struct By1 {
    pub channel__id: i64,
}
pub struct By2<'a> {
    pub channel__name: &'a str,
}
pub struct By3<'a> {
    pub channel__linked_name: &'a str,
}
