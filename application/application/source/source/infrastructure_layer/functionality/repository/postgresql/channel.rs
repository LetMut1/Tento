use super::Postgresql;
use crate::infrastructure_layer::functionality::repository::Repository;
use crate::{
    domain_layer::data::entity::channel::Channel,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
    },
};
use super::PostgresqlPreparedStatementParameterStorage;
use dedicated_crate::void::Void;
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl Repository<Postgresql<Channel<'_>>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<Channel<'static>, AggregateError>> + Send + Capture<&'a Void> {
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
            let channel__description = insert_1.channel__description.as_ref();
            let channel__orientation = insert_1.channel__orientation.as_slice();
            let channel__cover_image_path = insert_1.channel__cover_image_path.as_ref();
            let channel__background_image_path = insert_1.channel__background_image_path.as_ref();
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &insert_1.channel__owner,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel__name,
                    Type::TEXT,
                )
                .add(
                    &insert_1.channel__linked_name,
                    Type::TEXT,
                )
                .add(
                    &channel__description,
                    Type::TEXT,
                )
                .add(
                    &insert_1.channel__access_modifier,
                    Type::INT2,
                )
                .add(
                    &insert_1.channel__visability_modifier,
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
                    &insert_1.channel__subscribers_quantity,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel__marks_quantity,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel__viewing_quantity,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel__created_at,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(
                Channel::new(
                    row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    insert_1.channel__owner,
                    Cow::Owned(insert_1.channel__name),
                    insert_1.channel__linked_name,
                    insert_1.channel__description,
                    insert_1.channel__access_modifier,
                    insert_1.channel__visability_modifier,
                    insert_1.channel__orientation,
                    insert_1.channel__cover_image_path,
                    insert_1.channel__background_image_path,
                    insert_1.channel__subscribers_quantity,
                    insert_1.channel__marks_quantity,
                    insert_1.channel__viewing_quantity,
                    insert_1.channel__created_at,
                ),
            );
        };
    }
    pub fn update_1<'a>(database_1_client: &'a Client, by_1: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.channel AS c \
                SET \
                    subscribers_quantity = subscribers_quantity + 1 \
                WHERE \
                    c.id = $1;";
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &by_1.channel__id,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(());
        };
    }
    pub fn find_1<'a>(database_1_client: &'a Client, by_1: By1) -> impl Future<Output = Result<Option<Channel<'static>>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage.add(
                &by_1.channel__id,
                Type::INT8,
            );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            if row_registry.is_empty() {
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    Channel::new(
                        by_1.channel__id,
                        row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        Cow::Owned(
                            row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            )?,
                        ),
                        row_registry[0].try_get::<'_, usize, String>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i16>(4).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i16>(5).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Vec<i16>>(6).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(7).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(8).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(9).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(10).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(11).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(12).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                    ),
                ),
            );
        };
    }
    pub fn find_2<'a, 'b>(database_1_client: &'a Client, by_2: By2<'b>) -> impl Future<Output = Result<Option<Channel<'b>>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage.add(
                &by_2.channel__name,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            if row_registry.is_empty() {
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    Channel::new(
                        row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        Cow::Borrowed(by_2.channel__name),
                        row_registry[0].try_get::<'_, usize, String>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i16>(4).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i16>(5).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Vec<i16>>(6).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(7).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, Option<String>>(8).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(9).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(10).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(11).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(12).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                    ),
                ),
            );
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by_2: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.name = $1;";
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage.add(
                &by_2.channel__name,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            if row_registry.is_empty() {
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
    pub fn is_exist_2<'a>(database_1_client: &'a Client, by_3: By3<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    c.id AS i \
                FROM \
                    public.channel c \
                WHERE \
                    c.linked_name = $1;";
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage.add(
                &by_3.channel__linked_name,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_1_client
                .query(
                    &statement,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            if row_registry.is_empty() {
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
