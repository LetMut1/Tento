use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::user_reset_password_token::{
            UserResetPasswordToken,
            derivative::{
                UserResetPasswordToken2,
                UserResetPasswordToken3,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<UserResetPasswordToken<'_>>> {
    pub fn create_1<'a>(
        database_2_client: &'a Client,
        user_reset_password_token: &'a UserResetPasswordToken<'_>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.user_reset_password_token AS urpt (\
                        user__id,\
                        user_device__id,\
                        value,\
                        wrong_enter_tries_quantity,\
                        is_approved,\
                        expires_at,\
                        can_be_resent_from\
                    ) VALUES (\
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5,\
                        $6,\
                        $7\
                    );";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &user_reset_password_token.user__id,
                    Type::INT8,
                )
                .add(
                    &user_reset_password_token.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &user_reset_password_token.value,
                    Type::TEXT,
                )
                .add(
                    &user_reset_password_token.wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &user_reset_password_token.is_approved,
                    Type::BOOL,
                )
                .add(
                    &user_reset_password_token.expires_at,
                    Type::INT8,
                )
                .add(
                    &user_reset_password_token.can_be_resent_from,
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn delete_2<'a>(database_2_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_reset_password_token AS urpt \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn update_1<'a>(database_2_client: &'a Client, update: Update1<'a>, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET (\
                    value,\
                    wrong_enter_tries_quantity,\
                    is_approved,\
                    expires_at,\
                    can_be_resent_from\
                ) = ROW(\
                    $1,\
                    $2,\
                    $3,\
                    $4,\
                    $5\
                ) \
                WHERE \
                    urpt.user__id = $6 \
                    AND urpt.user_device__id = $7;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update.user_reset_password_token__value,
                    Type::TEXT,
                )
                .add(
                    &update.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &update.user_reset_password_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update.user_reset_password_token__can_be_resent_from,
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn update_2<'a>(database_2_client: &'a Client, update: Update2, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET (\
                    can_be_resent_from\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    urpt.user__id = $2 \
                    AND urpt.user_device__id = $3;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update.user_reset_password_token__can_be_resent_from,
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn update_3<'a>(database_2_client: &'a Client, update: Update3<'a>, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET (\
                    value,\
                    wrong_enter_tries_quantity,\
                    is_approved, \
                    expires_at\
                ) = ROW(\
                    $1,\
                    $2,\
                    $3,\
                    $4 \
                ) \
                WHERE \
                    urpt.user__id = $5 \
                    AND urpt.user_device__id = $6;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update.user_reset_password_token__value,
                    Type::TEXT,
                )
                .add(
                    &update.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &update.user_reset_password_token__expires_at,
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn update_4<'a>(database_2_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET \
                    wrong_enter_tries_quantity = wrong_enter_tries_quantity + 1 \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    pub fn update_5<'a>(database_2_client: &'a Client, update: Update5, by: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET (\
                    is_approved\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    urpt.user__id = $2 \
                    AND urpt.user_device__id = $3;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update.user_reset_password_token__is_approved,
                    Type::BOOL,
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
            crate::result_return_runtime!(
                database_2_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            return Result::Ok(());
        };
    }
    // Return values:
    // user_reset_password_token__value: String,
    // user_reset_password_token__wrong_enter_tries_quantity: i16,
    // user_reset_password_token__is_approved: bool,
    // user_reset_password_token__expires_at: i64,
    // user_reset_password_token__can_be_resent_from: i64,
    pub fn find_1<'a>(database_2_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<Option<(String, i16, bool, i64, i64)>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    urpt.value AS v,\
                    urpt.wrong_enter_tries_quantity AS wetq,\
                    urpt.is_approved AS ia,\
                    urpt.expires_at AS ea,\
                    urpt.can_be_resent_from AS cbrf \
                FROM \
                    public.user_reset_password_token urpt \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
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
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, bool>(2)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(4)),
                    ),
                ),
            );
        };
    }
    pub fn find_2<'a>(database_2_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<Option<UserResetPasswordToken2>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    urpt.value AS v,\
                    urpt.wrong_enter_tries_quantity AS wetq,\
                    urpt.is_approved AS ia,\
                    urpt.expires_at AS ea \
                FROM \
                    public.user_reset_password_token urpt \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
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
                    UserResetPasswordToken2 {
                        value: crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                        wrong_enter_tries_quantity: crate::result_return_logic!(rows[0].try_get::<'_, usize, i16>(1)),
                        is_approved: crate::result_return_logic!(rows[0].try_get::<'_, usize, bool>(2)),
                        expires_at: crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                    },
                ),
            );
        };
    }
    pub fn find_3<'a>(database_2_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<Option<UserResetPasswordToken3>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    urpt.value AS v,\
                    urpt.is_approved AS ia,\
                    urpt.expires_at AS ea,\
                    urpt.can_be_resent_from AS cbrf \
                FROM \
                    public.user_reset_password_token urpt \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
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
                    UserResetPasswordToken3 {
                        value: crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                        is_approved: crate::result_return_logic!(rows[0].try_get::<'_, usize, bool>(1)),
                        expires_at: crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(2)),
                        can_be_resent_from: crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(3)),
                    },
                ),
            );
        };
    }
}
pub struct Update1<'a> {
    pub user_reset_password_token__value: &'a str,
    pub user_reset_password_token__wrong_enter_tries_quantity: i16,
    pub user_reset_password_token__is_approved: bool,
    pub user_reset_password_token__expires_at: i64,
    pub user_reset_password_token__can_be_resent_from: i64,
}
pub struct Update2 {
    pub user_reset_password_token__can_be_resent_from: i64,
}
pub struct Update3<'a> {
    pub user_reset_password_token__value: &'a str,
    pub user_reset_password_token__wrong_enter_tries_quantity: i16,
    pub user_reset_password_token__is_approved: bool,
    pub user_reset_password_token__expires_at: i64,
}
pub struct Update5 {
    pub user_reset_password_token__is_approved: bool,
}
pub struct By1<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
}
