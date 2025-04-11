use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_delayed_deletion::ChannelDelayedDeletion,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelDelayedDeletion>> {
    pub fn create<'a>(client_database_4: &'a Client, insert: Insert) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.channel_delayed_deletion (\
                        channel__id,\
                        can_be_deleted_from,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    )\
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
                .add(
                    &insert.channel__id,
                    Type::INT8,
                )
                .add(
                    &insert.channel_delayed_deletion__can_be_deleted_from,
                    Type::INT8,
                )
                .add(
                    &insert.channel_delayed_deletion__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_4
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_4
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(false)
            }
            return Result::Ok(true);
        };
    }
}
pub struct Insert {
    pub channel__id: i64,
    pub channel_delayed_deletion__can_be_deleted_from: i64,
    pub channel_delayed_deletion__created_at: i64,
}