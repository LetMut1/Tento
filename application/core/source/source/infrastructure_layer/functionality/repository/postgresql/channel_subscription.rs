use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_subscription::ChannelSubscription,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelSubscription>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert: Insert) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
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
                    &insert.user__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel_subscription__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by: By) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
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
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.channel__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
pub struct Insert {
    pub user__id: i64,
    pub channel__id: i64,
    pub channel_subscription__created_at: i64,
}
pub struct By {
    pub user__id: i64,
    pub channel__id: i64,
}
