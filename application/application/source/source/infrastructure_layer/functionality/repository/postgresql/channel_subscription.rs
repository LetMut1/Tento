use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::channel_subscription::ChannelSubscription,
    infrastructure_layer::{
        data::capture::Capture,
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
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
impl PostgresqlRepository<ChannelSubscription> {
    pub fn create_1<'a>(database_1_connection: &'a Connection, insert_1: Insert1) -> impl Future<Output = Result<ChannelSubscription, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                INSERT INTO public.channel_subscription AS cs ( \
                    application_user__id, \
                    channel__id, \
                    created_at \
                ) VALUES ( \
                    $1, \
                    $2, \
                    current_timestamp(6) \
                ) \
                RETURNING \
                    cs.created_at::TEXT AS ca;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.channel__id,
                    Type::INT8,
                );
            let statement = database_1_connection
                .prepare_typed(
                    query,
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
            let channel_subscription = ChannelSubscription::new(
                insert_1.application_user__id,
                insert_1.channel__id,
                row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?,
            );
            return Result::Ok(channel_subscription);
        };
    }
    pub fn is_exist_1<'a>(database_1_connection: &'a Connection, by_1: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    cs.application_user__id AS aui \
                FROM public.channel_subscription cs \
                WHERE cs.application_user__id = $1 AND cs.channel__id = $2;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.channel__id,
                    Type::INT8,
                );
            let statement = database_1_connection
                .prepare_typed(
                    query,
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
            if row_registry.is_empty() {
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
}
pub struct Insert1 {
    pub application_user__id: i64,
    pub channel__id: i64,
}
pub struct By1 {
    pub application_user__id: i64,
    pub channel__id: i64,
}
