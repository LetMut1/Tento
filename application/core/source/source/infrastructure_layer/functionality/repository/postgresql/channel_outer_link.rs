use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::channel_outer_link::ChannelOuterLink,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    dedicated::action_processor_incoming_outcoming::ChannelOuterLink1,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<ChannelOuterLink>> {
    pub fn create_1<'a>(database_1_client: &'a Client, channel_outer_link: &'a ChannelOuterLink) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &channel_outer_link.from,
                    Type::INT8,
                )
                .add(
                    &channel_outer_link.alias,
                    Type::TEXT,
                )
                .add(
                    &channel_outer_link.address,
                    Type::TEXT,
                )
                .add(
                    &channel_outer_link.created_at,
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
    pub fn find_1<'a>(database_1_client: &'a Client, by_1: By1, limit: i16) -> impl Future<Output = Result<Vec<ChannelOuterLink1>, AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_1.channel_outer_link__from,
                    Type::INT8,
                )
                .add(
                    &limit,
                    Type::INT2,
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
            let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];
            if row_registry.is_empty() {
                return Result::Ok(channel_outer_link_registry);
            }
            '_a: for row in row_registry.iter() {
                let channel_outer_link = ChannelOuterLink1 {
                    channel_outer_link__alias: crate::result_return_logic!(row.try_get::<'_, usize, String>(0)),
                    channel_outer_link__address: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                };
                channel_outer_link_registry.push(channel_outer_link);
            }
            return Result::Ok(channel_outer_link_registry);
        };
    }
}
pub struct By1 {
    pub channel_outer_link__from: i64,
}
