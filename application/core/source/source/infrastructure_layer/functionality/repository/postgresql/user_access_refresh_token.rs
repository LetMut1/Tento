use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<UserAccessRefreshToken>> {
    pub fn create<'a>(database_2_client: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.user_access_refresh_token AS uart (\
                        user__id,\
                        user_device__id,\
                        user_access_token__id,\
                        obfuscation_value,\
                        expires_at,\
                        updated_at\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6\
                    ) \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(6);
            parameter_storage
                .add(
                    &insert.user__id,
                    Type::INT8,
                )
                .add(
                    &insert.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert.user_access_token__id,
                    Type::TEXT,
                )
                .add(
                    &insert.user_access_refresh_token__obfuscation_value,
                    Type::TEXT,
                )
                .add(
                    &insert.user_access_refresh_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &insert.user_access_refresh_token__updated_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
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
    pub fn update<'a>(database_2_client: &'a Client, update: Update<'a>, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_access_refresh_token AS uart \
                SET (\
                    user_access_token__id,\
                    obfuscation_value,\
                    expires_at,\
                    updated_at\
                ) = ROW(\
                    $1,\
                    $2,\
                    $3,\
                    $4\
                ) \
                WHERE \
                    uart.user__id = $5 \
                    AND uart.user_device__id = $6 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(6);
            parameter_storage
                .add(
                    &update.user_access_token__id,
                    Type::TEXT,
                )
                .add(
                    &update.user_access_refresh_token__obfuscation_value,
                    Type::TEXT,
                )
                .add(
                    &update.user_access_refresh_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update.user_access_refresh_token__updated_at,
                    Type::INT8,
                )
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
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
    pub fn delete_1<'a>(database_2_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
            DELETE FROM ONLY \
                public.user_access_refresh_token AS uart \
            WHERE \
                uart.user__id = $1 \
                AND uart.user_device__id = $2 \
            RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
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
    pub fn delete_2<'a>(database_2_client: &'a Client, by: By1) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_access_refresh_token AS uart \
                WHERE \
                    uart.user__id = $1 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
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
    // user_access_token__id: String,
    // user_access_refresh_token__obfuscation_value: String,
    // user_access_refresh_token__expires_at: i64,
    // user_access_refresh_token__updated_at: i64,
    pub fn find<'a>(
        database_2_client: &'a Client,
        by: By2<'a>,
    ) -> impl Future<
        Output = Result<
            Option<(
                String,
                String,
                i64,
                i64,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    uart.user_access_token__id AS uati,\
                    uart.obfuscation_value AS ov,\
                    uart.expires_at AS ea,\
                    uart.updated_at AS ua \
                FROM \
                    public.user_access_refresh_token uart \
                WHERE \
                    uart.user__id = $1 \
                    AND uart.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                    ),
                ),
            );
        };
    }
    pub fn is_exist<'a>(database_2_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    uart.user__id AS ui \
                FROM \
                    public.user_access_refresh_token uart \
                WHERE \
                    uart.user__id = $1 \
                    AND uart.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_2_client
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
pub struct Insert<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
    pub user_access_token__id: &'a str,
    pub user_access_refresh_token__obfuscation_value: &'a str,
    pub user_access_refresh_token__expires_at: i64,
    pub user_access_refresh_token__updated_at: i64,
}
pub struct Update<'a> {
    pub user_access_token__id: &'a str,
    pub user_access_refresh_token__obfuscation_value: &'a str,
    pub user_access_refresh_token__expires_at: i64,
    pub user_access_refresh_token__updated_at: i64,
}
pub struct By1 {
    pub user__id: i64,
}
pub struct By2<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
}
