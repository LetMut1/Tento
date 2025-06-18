use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::quantity_limiter::QuantityLimiter,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<QuantityLimiter>> {
    pub fn create<'a>(client_database_3: &'a Client, insert: Insert) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.quantity_limiter (\
                        user__id,\
                        owned_channels_quantity,\
                        created_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
            let quantity_limiter__owned_channels_quantity = insert.quantity_limiter__owned_channels_quantity as i16;
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
                .add(
                    &insert.user__id,
                    Type::INT8,
                )
                .add(
                    &quantity_limiter__owned_channels_quantity,
                    Type::INT2,
                )
                .add(
                    &insert.quantity_limiter__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_3
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
    pub fn update_1<'a>(client_database_3: &'a Client, update: Update, by: By) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.quantity_limiter AS ql \
                SET \
                    owned_channels_quantity = owned_channels_quantity + 1 \
                WHERE \
                    ql.user__id = $1 \
                    AND ql.owned_channels_quantity < $2 \
                RETURNING \
                    true AS _;";
            let quantity_limiter_maximum_owned_channels_quantity = update.quantity_limiter__owned_channels_quantity___maximum_value as i16;
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &quantity_limiter_maximum_owned_channels_quantity,
                    Type::INT2,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_3
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
    pub fn update_2<'a>(client_database_3: &'a Client, by: By) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.quantity_limiter AS ql \
                SET \
                    owned_channels_quantity = owned_channels_quantity - 1 \
                WHERE \
                    ql.user__id = $1 \
                    AND ql.owned_channels_quantity > $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &(u8::MIN as i16),
                    Type::INT2,
                );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_3
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
    // quantity_limiter__owned_channels_quantity: u8,
    pub fn find<'a>(client_database_3: &'a Client, by: By) -> impl Future<Output = Result<Option<u8>, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                SELECT \
                    ql.owned_channels_quantity AS ocq \
                FROM \
                    public.quantity_limiter ql \
                WHERE \
                    ql.user__id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                client_database_3
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_3
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(Option::None);
            }
            let quantity_limiter__owned_channels_quantity = crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1));
            if quantity_limiter__owned_channels_quantity < u8::MIN as i16 || quantity_limiter__owned_channels_quantity > u8::MAX as i16 {
                return Result::Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(Option::Some(quantity_limiter__owned_channels_quantity as u8));
        };
    }
}
pub struct Insert {
    pub user__id: i64,
    pub quantity_limiter__owned_channels_quantity: u8,
    pub quantity_limiter__created_at: i64,
}
pub struct Update {
    pub quantity_limiter__owned_channels_quantity___maximum_value: u8,
}
pub struct By {
    pub user__id: i64,
}
