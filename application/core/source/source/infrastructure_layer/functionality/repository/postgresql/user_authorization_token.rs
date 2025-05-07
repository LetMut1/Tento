use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::user_authorization_token::UserAuthorizationToken,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<UserAuthorizationToken>> {
    pub fn create<'a>(client_database_2: &'a Client, insert: Insert<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                INSERT INTO \
                    public.user_authorization_token (\
                        user__obfuscated_id,\
                        user_device__id,\
                        value,\
                        wrong_enter_tries_quantity,\
                        expires_at,\
                        can_be_resent_from\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6\
                    ) \
                ON CONFLICT DO NOTHING \
                RETURNING \
                    true AS _;";
            let user_authorization_token__wrong_enter_tries_quantity = insert.user_authorization_token__wrong_enter_tries_quantity as i16;
            let mut parameter_storage = ParameterStorage::new(6);
            parameter_storage
                .add(
                    &insert.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &insert.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert.user_authorization_token__value,
                    Type::TEXT,
                )
                .add(
                    &user_authorization_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &insert.user_authorization_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &insert.user_authorization_token__can_be_resent_from,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    pub fn delete<'a>(client_database_2: &'a Client, by: By<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                DELETE FROM ONLY \
                    public.user_authorization_token AS uat \
                WHERE \
                    uat.user__obfuscated_id = $1 \
                    AND uat.user_device__id = $2 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    pub fn update_1<'a>(client_database_2: &'a Client, update: Update1<'a>, by: By<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET (\
                    value,\
                    wrong_enter_tries_quantity,\
                    expires_at,\
                    can_be_resent_from\
                ) = ROW(\
                    $1,\
                    $2,\
                    $3,\
                    $4\
                ) \
                WHERE \
                    uat.user__obfuscated_id = $5 \
                    AND uat.user_device__id = $6 \
                RETURNING \
                    true AS _;";
            let user_authorization_token__wrong_enter_tries_quantity = update.user_authorization_token__wrong_enter_tries_quantity as i16;
            let mut parameter_storage = ParameterStorage::new(6);
            parameter_storage
                .add(
                    &update.user_authorization_token__value,
                    Type::TEXT,
                )
                .add(
                    &user_authorization_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update.user_authorization_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update.user_authorization_token__can_be_resent_from,
                    Type::INT8,
                )
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    pub fn update_2<'a>(client_database_2: &'a Client, update: Update2<'a>, by: By<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET (\
                    value,\
                    wrong_enter_tries_quantity,\
                    expires_at\
                ) = ROW(\
                    $1,\
                    $2,\
                    $3\
                ) \
                WHERE \
                    uat.user__obfuscated_id = $4 \
                    AND uat.user_device__id = $5 \
                RETURNING \
                    true AS _;";
            let user_authorization_token__wrong_enter_tries_quantity = update.user_authorization_token__wrong_enter_tries_quantity as i16;
            let mut parameter_storage = ParameterStorage::new(5);
            parameter_storage
                .add(
                    &update.user_authorization_token__value,
                    Type::TEXT,
                )
                .add(
                    &user_authorization_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update.user_authorization_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    pub fn update_3<'a>(client_database_2: &'a Client, update: Update3, by: By<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET (\
                    can_be_resent_from\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    uat.user__obfuscated_id = $2 \
                    AND uat.user_device__id = $3 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
                .add(
                    &update.user_authorization_token__can_be_resent_from,
                    Type::INT8,
                )
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    pub fn update_4<'a>(client_database_2: &'a Client, by: By<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET \
                    wrong_enter_tries_quantity = wrong_enter_tries_quantity + 1 \
                WHERE \
                    uat.user__obfuscated_id = $1 \
                    AND uat.user_device__id = $2 \
                    AND uat.wrong_enter_tries_quantity < $3 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(3);
            parameter_storage
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &(u8::MAX as i16),
                    Type::INT2,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
    // user_authorization_token__value: String,
    // user_authorization_token__wrong_enter_tries_quantity: u8,
    // user_authorization_token__expires_at: i64,
    // user_authorization_token__can_be_resent_from: i64,
    pub fn find_1<'a>(
        client_database_2: &'a Client,
        by: By<'a>,
    ) -> impl Future<
        Output = Result<
            Option<(
                String,
                u8,
                i64,
                i64,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                SELECT \
                    uat.value AS v,\
                    uat.wrong_enter_tries_quantity AS wetq,\
                    uat.expires_at AS ea,\
                    uat.can_be_resent_from AS cbrf \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__obfuscated_id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(Option::None);
            }
            let user_authorization_token__wrong_enter_tries_quantity = crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1));
            if user_authorization_token__wrong_enter_tries_quantity < u8::MIN as i16 || user_authorization_token__wrong_enter_tries_quantity > u8::MAX as i16 {
                return Result::Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                        user_authorization_token__wrong_enter_tries_quantity as u8,
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                    ),
                ),
            );
        };
    }
    // user_authorization_token__value: String,
    // user_authorization_token__wrong_enter_tries_quantity: u8,
    // user_authorization_token__expires_at: i64,
    pub fn find_2<'a>(
        client_database_2: &'a Client,
        by: By<'a>,
    ) -> impl Future<
        Output = Result<
            Option<(
                String,
                u8,
                i64,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                SELECT \
                    uat.value AS v,\
                    uat.wrong_enter_tries_quantity AS wetq,\
                    uat.expires_at AS ea \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__obfuscated_id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Result::Ok(Option::None);
            }
            let user_authorization_token__wrong_enter_tries_quantity = crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1));
            if user_authorization_token__wrong_enter_tries_quantity < u8::MIN as i16 || user_authorization_token__wrong_enter_tries_quantity > u8::MAX as i16 {
                return Result::Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(
                Option::Some(
                    (
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                        user_authorization_token__wrong_enter_tries_quantity as u8,
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(2)),
                    ),
                ),
            );
        };
    }
    // user_authorization_token__value: String,
    // user_authorization_token__expires_at: i64,
    // user_authorization_token__can_be_resent_from: i64,
    pub fn find_3<'a>(
        client_database_2: &'a Client,
        by: By<'a>,
    ) -> impl Future<
        Output = Result<
            Option<(
                String,
                i64,
                i64,
            )>,
            AggregateError,
        >,
    > + Send
    + use<'a> {
        return async move {
            const QUERY: &'static str = "\
                SELECT \
                    uat.value AS v,\
                    uat.expires_at AS ea,\
                    uat.can_be_resent_from AS cbrf \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__obfuscated_id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &by.user__obfuscated_id,
                    Type::INT8,
                )
                .add(
                    &by.user_device__id,
                    Type::TEXT,
                );
            let statement = crate::result_return_logic!(
                client_database_2
                .prepare_typed_cached(
                    QUERY,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                client_database_2
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
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(2)),
                    ),
                ),
            );
        };
    }
}
pub struct Insert<'a> {
    pub user__obfuscated_id: i64,
    pub user_device__id: &'a str,
    pub user_authorization_token__value: &'a str,
    pub user_authorization_token__wrong_enter_tries_quantity: u8,
    pub user_authorization_token__expires_at: i64,
    pub user_authorization_token__can_be_resent_from: i64,
}
pub struct Update1<'a> {
    pub user_authorization_token__value: &'a str,
    pub user_authorization_token__wrong_enter_tries_quantity: u8,
    pub user_authorization_token__expires_at: i64,
    pub user_authorization_token__can_be_resent_from: i64,
}
pub struct Update2<'a> {
    pub user_authorization_token__value: &'a str,
    pub user_authorization_token__wrong_enter_tries_quantity: u8,
    pub user_authorization_token__expires_at: i64,
}
pub struct Update3 {
    pub user_authorization_token__can_be_resent_from: i64,
}
pub struct By<'a> {
    pub user__obfuscated_id: i64,
    pub user_device__id: &'a str,
}
