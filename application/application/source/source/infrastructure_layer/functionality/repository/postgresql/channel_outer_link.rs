use super::{
    Postgresql,
    PreparedStatementParameterStorage,
};
use crate::{
    domain_layer::data::entity::channel_outer_link::ChannelOuterLink,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::repository::Repository,
    },
};
use deadpool_postgres::Client;
use dedicated_crate::{
    action_processor_incoming_outcoming::ChannelOuterLink1,
    void::Void,
};
use std::future::Future;
use tokio_postgres::types::Type;
impl Repository<Postgresql<ChannelOuterLink>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<ChannelOuterLink, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_inner_link AS cil (\
                        from_,\
                        alias,\
                        address,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4\
                    );";
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &insert_1.channel_outer_link__from,
                    Type::INT8,
                )
                .add(
                    &insert_1.channel_outer_link__alias,
                    Type::TEXT,
                )
                .add(
                    &insert_1.channel_outer_link__address,
                    Type::TEXT,
                )
                .add(
                    &insert_1.channel_outer_link__created_at,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let channel_outer_link = ChannelOuterLink::new(
                insert_1.channel_outer_link__from,
                insert_1.channel_outer_link__alias,
                insert_1.channel_outer_link__address,
                insert_1.channel_outer_link__created_at,
            );
            return Result::Ok(channel_outer_link);
        };
    }
    pub fn find_1<'a>(database_1_client: &'a Client, by_1: By1, limit: i16) -> impl Future<Output = Result<Vec<ChannelOuterLink1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    col.alias AS al,\
                    col.address AS ad \
                FROM \
                    public.channel_outer_link col \
                WHERE \
                    col.from_ = $1 \
                LIMIT $2";
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.channel_outer_link__from,
                    Type::INT8,
                )
                .add(
                    &limit,
                    Type::INT2,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(channel_outer_link_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel_outer_link = ChannelOuterLink1 {
                    channel_outer_link__alias: row.try_get::<'_, usize, String>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    channel_outer_link__address: row.try_get::<'_, usize, String>(1).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                };
                channel_outer_link_registry.push(channel_outer_link);
            }
            return Result::Ok(channel_outer_link_registry);
        };
    }
}
pub struct Insert1 {
    pub channel_outer_link__from: i64,
    pub channel_outer_link__alias: String,
    pub channel_outer_link__address: String,
    pub channel_outer_link__created_at: i64,
}
pub struct By1 {
    pub channel_outer_link__from: i64,
}
