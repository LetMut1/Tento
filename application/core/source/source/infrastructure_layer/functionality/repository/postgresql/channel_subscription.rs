use super::{
    Postgresql,
    ParameterStorage,
};
use crate::{
    domain_layer::data::entity::channel_subscription::ChannelSubscription,
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::repository::Repository,
    },
};
use deadpool_postgres::Client;
use std::future::Future;
use tokio_postgres::types::Type;
impl Repository<Postgresql<ChannelSubscription>> {
    pub fn create_1<'a>(database_1_client: &'a Client, channel_subscription: &'a ChannelSubscription) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameter_type_registry(),
                )
                .await
            );
            crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameter_registry(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by_1: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    cs.user__id AS ui \
                FROM \
                    public.channel_subscription cs \
                WHERE \
                    cs.user__id = $1 \
                    AND cs.channel__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.channel__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
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
