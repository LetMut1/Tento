use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel::Channel,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::Repository,
                service::counter::{
                    Counter,
                    Counter_,
                },
            },
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::{
        Row,
        types::Type,
    },
};
// channel__id: i64,
impl Repository<Postgresql<Channel>> {
    pub fn create<'a>(client_database_3: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
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
                        created_at\
                    ) VALUES (\
                        nextval('public.channel_1'), \
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
                        $11\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    c.id AS i;";
            let channel__subscribers_quantity = insert.channel__subscribers_quantity;
            let mut parameter_storage = ParameterStorage::new(11);
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
                    &insert.channel__description,
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
                    &insert.channel__orientation,
                    Type::INT2_ARRAY,
                )
                .add(
                    &insert.channel__cover_image_path,
                    Type::TEXT,
                )
                .add(
                    &insert.channel__background_image_path,
                    Type::TEXT,
                )
                .add(
                    &channel__subscribers_quantity,
                    Type::INT8,
                )
                .add(
                    &insert.channel__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(Option::Some(crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0))));
        };
    }
    pub fn update_1<'a>(client_database_3: &'a Client, by: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.channel AS c \
                SET \
                    subscribers_quantity = subscribers_quantity + 1 \
                WHERE \
                    c.id = $1 \
                    AND c.subscribers_quantity < $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.channel__id,
                    Type::INT8,
                )
                .add(
                    &(u32::MAX as i64),
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
    pub fn update_2<'a>(client_database_3: &'a Client, by: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.channel AS c \
                SET \
                    subscribers_quantity = subscribers_quantity - 1 \
                WHERE \
                    c.id = $1 \
                    AND c.subscribers_quantity > $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.channel__id,
                    Type::INT8,
                )
                .add(
                    &(u32::MIN as i64),
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
    // channel__owner: i64,
    // channel__name: String,
    // channel__linked_name: String,
    // channel__description: Option<String>,
    // channel__access_modifier: i16,
    // channel__visability_modifier: i16,
    // channel__orientation: Vec<i16>,
    // channel__cover_image_path: Option<String>,
    // channel__background_image_path: Option<String>,
    // channel__subscribers_quantity: u32,
    pub fn find_1<'a>(
        client_database_3: &'a Client,
        by: By1,
    ) -> impl Future<
        Output = Result<
            Option<(
                i64,
                String,
                String,
                Option<String>,
                i16,
                i16,
                Vec<i16>,
                Option<String>,
                Option<String>,
                u32,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.owner AS o1,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.description AS d,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.orientation AS o2,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    c.subscribers_quantity AS sq \
                FROM \
                    public.channel c \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
                return Result::Ok(Option::None);
            }
            let channel__subscribers_quantity = crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(9));
            if channel__subscribers_quantity < (u32::MIN as i64) || channel__subscribers_quantity > (u32::MAX as i64) {
                return Result::Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(3)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(4)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(5)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Vec<i16>>(6)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(7)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, Option<String>>(8)),
                        channel__subscribers_quantity as u32,
                    ),
                ),
            );
        };
    }
    // channel__owner: i64,
    // channel__access_modifier: i16,
    pub fn find_2<'a>(
        client_database_3: &'a Client,
        by: By1,
    ) -> impl Future<
        Output = Result<
            Option<(
                i64,
                i16,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.owner AS ow,\
                    c.access_modifier AS am \
                FROM \
                    public.channel c \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1)),
                    ),
                ),
            );
        };
    }
    pub fn find_3<'a>(client_database_3: &'a Client, by: By4<'a>, limit: i16) -> impl Future<Output = Result<Vec<Row>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.owner AS o, \
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    cs.channel__id AS ca \
                FROM \
                    public.channel c \
                LEFT OUTER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id \
                WHERE \
                    c.visability_modifier = $2 \
                    AND c.name LIKE $3"
                .to_string();
            let mut counter = Counter::<u8>::new(
                3,
                1,
            );
            let wildcard = format!("{}%", by.channel__name,);
            let mut parameter_storage = ParameterStorage::new(5);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.channel__visability_modifier,
                    Type::INT2,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by.requery___channel__name {
                query = format!(
                    "{} \
                    AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY \
                    c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            return crate::result_into_runtime!(
                client_database_3
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
        };
    }
    pub fn find_4<'a>(client_database_3: &'a Client, by: By5<'a>, limit: i16) -> impl Future<Output = Result<Vec<Row>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip \
                FROM \
                    public.channel c \
                INNER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id \
                WHERE \
                    c.name LIKE $2"
                .to_string();
            let mut counter = Counter::<u8>::new(
                2,
                1,
            );
            let wildcard = format!("{}%", by.channel__name,);
            let mut parameter_storage = ParameterStorage::new(4);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by.requery___channel__name {
                query = format!(
                    "{} \
                    AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY \
                    c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            return crate::result_into_runtime!(
                client_database_3
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
        };
    }
    pub fn find_5<'a>(client_database_3: &'a Client, by: By6, limit: i16) -> impl Future<Output = Result<Vec<Row>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip \
                FROM \
                    public.channel c \
                INNER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id"
                .to_string();
            let mut counter = Counter::<u8>::new(
                1,
                1,
            );
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let requery___channel__id: i64;
            if let Option::Some(requery___channel__id_) = by.requery___channel__id {
                requery___channel__id = requery___channel__id_;
                query = format!(
                    "{} \
                    WHERE \
                        cs.channel__id > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    &requery___channel__id,
                    Type::INT8,
                );
            }
            query = format!(
                "{} \
                ORDER BY \
                    cs.channel__id ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            return crate::result_into_runtime!(
                client_database_3
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
        };
    }
    // channel__owner: i64,
    // channel__access_modifier: i16,
    pub fn find_6<'a>(
        client_database_3: &'a Client,
        by: By1,
    ) -> impl Future<
        Output = Result<
            Option<(
                i64,
                i16,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.owner AS o,\
                    c.access_modifier AS am \
                FROM \
                    public.channel c \
                WHERE \
                    c.id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1)),
                    ),
                ),
            );
        };
    }
    pub fn is_exist_1<'a>(client_database_3: &'a Client, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.name = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel__name,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
    pub fn is_exist_2<'a>(client_database_3: &'a Client, by: By3<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.linked_name = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.channel__linked_name,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    query,
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
pub struct Insert<'a> {
    pub channel__owner: i64,
    pub channel__name: &'a str,
    pub channel__linked_name: &'a str,
    pub channel__description: Option<&'a str>,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
    pub channel__orientation: &'a [i16],
    pub channel__cover_image_path: Option<&'a str>,
    pub channel__background_image_path: Option<&'a str>,
    pub channel__subscribers_quantity: u32,
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
pub struct By4<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
    pub channel__visability_modifier: i16,
}
pub struct By5<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
}
pub struct By6 {
    pub user__id: i64,
    pub requery___channel__id: Option<i64>,
}
