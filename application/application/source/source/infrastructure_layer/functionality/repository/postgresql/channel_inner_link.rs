use super::Postgresql;
use crate::infrastructure_layer::functionality::repository::Repository;
use crate::{
    domain_layer::data::entity::channel_inner_link::ChannelInnerLink,
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
use dedicated_crate::{
    action_processor_incoming_outcoming::ChannelInnerLink1,
    void::Void,
};
use std::future::Future;
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl Repository<Postgresql<ChannelInnerLink>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<ChannelInnerLink, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_inner_link (\
                        from_,\
                        to_,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    );";
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &insert_1.channel_inner_link__from,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel_inner_link__to,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel_inner_link__created_at,
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
            return Result::Ok(
                ChannelInnerLink::new(
                    insert_1.channel_inner_link__from,
                    insert_1.channel_inner_link__to,
                    insert_1.channel_inner_link__created_at,
                ),
            );
        };
    }
    pub fn find_1<'a>(
        database_1_client: &'a Client,
        by_1: By1,
        limit: i16,
    ) -> impl Future<Output = Result<Vec<ChannelInnerLink1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    cil.to_ AS t \
                FROM \
                    public.channel_inner_link cil \
                WHERE \
                    cil.from_ = $1 \
                LIMIT $2";
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &by_1.channel_inner_link__from,
                    Type::INT8,
                )
                .add(
                    &limit,
                    Type::INT2,
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
            let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(channel_inner_link_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel_inner_link = ChannelInnerLink1 {
                    channel_inner_link__to: row.try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                };
                channel_inner_link_registry.push(channel_inner_link);
            }
            return Result::Ok(channel_inner_link_registry);
        };
    }
}
pub struct Insert1 {
    pub channel_inner_link__from: i64,
    pub channel_inner_link__to: i64,
    pub channel_inner_link__created_at: i64,
}
pub struct By1 {
    pub channel_inner_link__from: i64,
}
