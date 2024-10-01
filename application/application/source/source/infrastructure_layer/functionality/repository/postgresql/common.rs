use super::PostgresqlRepository;
use crate::infrastructure_layer::{
    data::capture::Capture,
    functionality::service::{
        counter::Counter,
        prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use action_processor_incoming_outcoming::{
    Channel1,
    Common1,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use std::future::Future;
use tokio_postgres::{
    types::Type,
    Client as Connection,
};
use void::Void;
impl PostgresqlRepository<Common1> {
    pub fn find_1<'a>(database_1_connection: &'a Connection, by_1: By1<'a>, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut counter = Counter::<i16>::new_classic();
            let mut query = format!(
                "SELECT \
                    c.id AS i, \
                    c.name AS n, \
                    c.linked_name AS ln, \
                    c.access_modifier AS am, \
                    c.cover_image_path AS cip, \
                    c.background_image_path AS bip, \
                    cs.channel__id AS ca \
                FROM public.channel c LEFT OUTER JOIN public.channel_subscription cs \
                ON cs.user__id = ${} AND c.id = cs.channel__id \
                WHERE c.visability_modifier = ${} AND c.name LIKE ${}",
                counter.get_next_value()?,
                counter.get_next_value()?,
                counter.get_next_value()?,
            );
            let wildcard = format!("{}%", by_1.channel__name,);
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.channel__visability_modifier,
                    Type::INT2,
                )
                .add_parameter(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by_1.requery___channel__name {
                query = format!(
                    "{} AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value()?,
                );
                prepared_statemant_parameter_convertation_resolver.add_parameter(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value()?
            );
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &limit,
                Type::INT2,
            );
            let statement = database_1_connection
                .prepare_typed(
                    query.as_str(),
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: row.try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__name: row.try_get::<'_, usize, String>(1).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__linked_name: row.try_get::<'_, usize, String>(2).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__access_modifier: row.try_get::<'_, usize, i16>(3).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__visability_modifier: by_1.channel__visability_modifier,
                    channel__cover_image_path: row.try_get::<'_, usize, Option<String>>(4).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__background_image_path: row.try_get::<'_, usize, Option<String>>(5).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                };
                let is_application_user_subscribed = match row.try_get::<'_, usize, Option<i64>>(6).into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )? {
                    Option::Some(_) => true,
                    Option::None => false,
                };
                let common = Common1 {
                    channel,
                    is_application_user_subscribed,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
    pub fn find_2<'a>(database_1_connection: &'a Connection, by_2: By2<'a>, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut counter = Counter::<i16>::new_classic();
            let mut query = format!(
                "SELECT \
                    c.id AS i, \
                    c.name AS n, \
                    c.linked_name AS ln, \
                    c.access_modifier AS am, \
                    c.visability_modifier AS vm, \
                    c.cover_image_path AS cip, \
                    c.background_image_path AS bip \
                FROM public.channel c INNER JOIN public.channel_subscription cs \
                ON cs.user__id = ${} AND c.id = cs.channel__id \
                WHERE c.name LIKE ${}",
                counter.get_next_value()?,
                counter.get_next_value()?,
            );
            let wildcard = format!("{}%", by_2.channel__name,);
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_2.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by_2.requery___channel__name {
                query = format!(
                    "{} AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value()?,
                );
                prepared_statemant_parameter_convertation_resolver.add_parameter(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value()?,
            );
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &limit,
                Type::INT2,
            );
            let statement = database_1_connection
                .prepare_typed(
                    query.as_str(),
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: row.try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__name: row.try_get::<'_, usize, String>(1).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__linked_name: row.try_get::<'_, usize, String>(2).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__access_modifier: row.try_get::<'_, usize, i16>(3).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__visability_modifier: row.try_get::<'_, usize, i16>(4).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__cover_image_path: row.try_get::<'_, usize, Option<String>>(5).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__background_image_path: row.try_get::<'_, usize, Option<String>>(6).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                };
                let common = Common1 {
                    channel,
                    is_application_user_subscribed: true,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
    pub fn find_3<'a>(database_1_connection: &'a Connection, by_3: By3, limit: i16) -> impl Future<Output = Result<Vec<Common1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut counter = Counter::<i16>::new_classic();
            let mut query = format!(
                "SELECT \
                    c.id AS i, \
                    c.name AS n, \
                    c.linked_name AS ln, \
                    c.access_modifier AS am, \
                    c.visability_modifier AS vm, \
                    c.cover_image_path AS cip, \
                    c.background_image_path AS bip \
                FROM public.channel c INNER JOIN public.channel_subscription cs \
                ON cs.user__id = ${} AND c.id = cs.channel__id",
                counter.get_next_value()?,
            );
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.application_user__id,
                Type::INT8,
            );
            let requery___channel__id: i64;
            if let Option::Some(requery___channel__id_) = by_3.requery___channel__id {
                requery___channel__id = requery___channel__id_;
                query = format!(
                    "{} \
                    WHERE cs.channel__id > ${}",
                    query.as_str(),
                    counter.get_next_value()?,
                );
                prepared_statemant_parameter_convertation_resolver.add_parameter(
                    &requery___channel__id,
                    Type::INT8,
                );
            }
            query = format!(
                "{} \
                ORDER BY cs.channel__id ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value()?,
            );
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &limit,
                Type::INT2,
            );
            let statement = database_1_connection
                .prepare_typed(
                    query.as_str(),
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let mut common_registry: Vec<Common1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(common_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel = Channel1 {
                    channel__id: row.try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__name: row.try_get::<'_, usize, String>(1).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__linked_name: row.try_get::<'_, usize, String>(2).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__access_modifier: row.try_get::<'_, usize, i16>(3).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__visability_modifier: row.try_get::<'_, usize, i16>(4).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__cover_image_path: row.try_get::<'_, usize, Option<String>>(5).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel__background_image_path: row.try_get::<'_, usize, Option<String>>(6).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                };
                let common = Common1 {
                    channel,
                    is_application_user_subscribed: true,
                };
                common_registry.push(common);
            }
            return Result::Ok(common_registry);
        };
    }
}
pub struct By1<'a> {
    pub application_user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
    pub channel__visability_modifier: i16,
}
pub struct By2<'a> {
    pub application_user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
}
pub struct By3 {
    pub application_user__id: i64,
    pub requery___channel__id: Option<i64>,
}
