use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::{
            repository::Repository,
            service::counter::{
                Counter,
                Counter_,
            },
        },
    },
    deadpool_postgres::Client,
    dedicated::action_processor_incoming_outcoming::{
        Channel1,
        Common1,
    },
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<Common1>> {
    pub fn find_1<'a>(database_1_client: &'a Client, by_1: By1<'a>, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
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
            let wildcard = format!("{}%", by_1.channel__name,);
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.channel__visability_modifier,
                    Type::INT2,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by_1.requery___channel__name {
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
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameter_type_registry(),
                )
                .await
            );
            let row_registry = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameter_registry(),
                )
                .await
            );
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: by_1.channel__visability_modifier,
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(4)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                };
                let is_user_subscribed = crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(6)).is_some();
                let common = Common1 {
                    channel,
                    is_user_subscribed,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
    pub fn find_2<'a>(database_1_client: &'a Client, by_2: By2<'a>, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + use<'a> {
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
                WHERE c.name LIKE $2"
                .to_string();
            let mut counter = Counter::<u8>::new(
                2,
                1,
            );
            let wildcard = format!("{}%", by_2.channel__name,);
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_2.user__id,
                    Type::INT8,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by_2.requery___channel__name {
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
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameter_type_registry(),
                )
                .await
            );
            let row_registry = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameter_registry(),
                )
                .await
            );
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(4)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                };
                let common = Common1 {
                    channel,
                    is_user_subscribed: true,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
    pub fn find_3<'a>(database_1_client: &'a Client, by_3: By3, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by_3.user__id,
                Type::INT8,
            );
            let requery___channel__id: i64;
            if let Option::Some(requery___channel__id_) = by_3.requery___channel__id {
                requery___channel__id = requery___channel__id_;
                query = format!(
                    "{} \
                    WHERE cs.channel__id > ${}",
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
                ORDER BY cs.channel__id ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameter_type_registry(),
                )
                .await
            );
            let row_registry = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameter_registry(),
                )
                .await
            );
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(4)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                };
                let common = Common1 {
                    channel,
                    is_user_subscribed: true,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
}
pub struct By1<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
    pub channel__visability_modifier: i16,
}
pub struct By2<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
}
pub struct By3 {
    pub user__id: i64,
    pub requery___channel__id: Option<i64>,
}
