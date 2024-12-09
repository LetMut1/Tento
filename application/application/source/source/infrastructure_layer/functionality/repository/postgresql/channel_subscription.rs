use super::{
    Postgresql,
    PreparedStatementParameterStorage,
};
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
        functionality::repository::Repository,
    },
};
use deadpool_postgres::Client;
use dedicated_crate::void::Void;
use std::future::Future;
use tokio_postgres::types::Type;
impl Repository<Postgresql<ChannelSubscription>> {
    pub fn create_1<'a>(database_1_client: &'a Client, channel_subscription: &'a ChannelSubscription) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &channel_subscription.user__id,
                    Type::INT8,
                )
                .add(
                    &channel_subscription.channel__id,
                    Type::INT8,
                )
                .add(
                    &channel_subscription.created_at,
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
            return Result::Ok(());
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.channel__id,
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
            if row_registry.is_empty() {
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
}
pub struct By1 {
    pub user__id: i64,
    pub channel__id: i64,
}
