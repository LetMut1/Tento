use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::channel_subscription::ChannelSubscription,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::service::postgresql_prepared_statemant_parameter_convertation_resolver::PostgresqlPreparedStatementParameterConvertationResolver,
    },
};
use dedicated_crate::void::Void;
use std::future::Future;
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl PostgresqlRepository<ChannelSubscription> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<ChannelSubscription, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_subscription AS cs (\
                        user__id,\
                        channel__id,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    );";
            let mut postgresql_prepared_statemant_parameter_convertation_resolver = PostgresqlPreparedStatementParameterConvertationResolver::new();
            postgresql_prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.channel__id,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.channel_subscription__created_at,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let channel_subscription = ChannelSubscription::new(
                insert_1.user__id,
                insert_1.channel__id,
                insert_1.channel_subscription__created_at,
            );
            return Result::Ok(channel_subscription);
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by_1: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    cs.user__id AS ui \
                FROM \
                    public.channel_subscription cs \
                WHERE \
                    cs.user__id = $1 \
                    AND cs.channel__id = $2;";
            let mut postgresql_prepared_statemant_parameter_convertation_resolver = PostgresqlPreparedStatementParameterConvertationResolver::new();
            postgresql_prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.channel__id,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
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
    pub user__id: i64,
    pub channel__id: i64,
    pub channel_subscription__created_at: i64,
}
pub struct By1 {
    pub user__id: i64,
    pub channel__id: i64,
}
