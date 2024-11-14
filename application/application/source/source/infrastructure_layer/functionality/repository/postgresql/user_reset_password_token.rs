use super::{
    Postgresql,
    PreparedStatementParameterStorage,
};
use crate::{
    domain_layer::data::entity::user_reset_password_token::{
        UserResetPasswordToken,
        UserResetPasswordToken_1,
        UserResetPasswordToken_2,
        UserResetPasswordToken_3,
    },
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
impl Repository<Postgresql<UserResetPasswordToken<'_>>> {
    pub fn create_1<'a, 'b>(
        database_2_client: &'a Client,
        user_reset_password_token: &'a UserResetPasswordToken<'b>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
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
            let statement = database_2_client
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
            database_2_client
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
    pub fn delete_2<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_reset_password_token AS urpt \
                WHERE \
                    urpt.user__id = $1 \
                    AND urpt.user_device__id = $2;";
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn update_1<'a>(database_2_client: &'a Client, update_1: Update1<'a>, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &update_1.user_reset_password_token__value,
                    Type::TEXT,
                )
                .add(
                    &update_1.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update_1.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &update_1.user_reset_password_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update_1.user_reset_password_token__can_be_resent_from,
                    Type::INT8,
                )
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn update_2<'a>(database_2_client: &'a Client, update_2: Update2, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &update_2.user_reset_password_token__can_be_resent_from,
                    Type::INT8,
                )
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn update_3<'a>(database_2_client: &'a Client, update_3: Update3<'a>, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &update_3.user_reset_password_token__value,
                    Type::TEXT,
                )
                .add(
                    &update_3.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update_3.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &update_3.user_reset_password_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn update_4<'a>(database_2_client: &'a Client, update_4: Update4, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_reset_password_token AS urpt \
                SET (\
                    wrong_enter_tries_quantity\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    urpt.user__id = $2 \
                    AND urpt.user_device__id = $3;";
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &update_4.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn update_5<'a>(database_2_client: &'a Client, update_5: Update5, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &update_5.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            database_2_client
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
    pub fn find_1<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserResetPasswordToken_1>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            let row_registry = database_2_client
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    UserResetPasswordToken_1 {
                        value: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        is_approved: row_registry[0].try_get::<'_, usize, bool>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        expires_at: row_registry[0].try_get::<'_, usize, i64>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(4).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                    },
                ),
            );
        };
    }
    pub fn find_2<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserResetPasswordToken_2>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            let row_registry = database_2_client
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    UserResetPasswordToken_2 {
                        value: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        is_approved: row_registry[0].try_get::<'_, usize, bool>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        expires_at: row_registry[0].try_get::<'_, usize, i64>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                    },
                ),
            );
        };
    }
    pub fn find_3<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserResetPasswordToken_3>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut prepared_statemant_parameter_storage = PreparedStatementParameterStorage::new();
            prepared_statemant_parameter_storage
                .add(
                    &by_1.user__id,
                    Type::INT8,
                )
                .add(
                    &by_1.user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_client
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
            let row_registry = database_2_client
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
                return Result::Ok(Option::None);
            }
            return Result::Ok(
                Option::Some(
                    UserResetPasswordToken_3 {
                        value: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        is_approved: row_registry[0].try_get::<'_, usize, bool>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        expires_at: row_registry[0].try_get::<'_, usize, i64>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
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
pub struct Update4 {
    pub user_reset_password_token__wrong_enter_tries_quantity: i16,
}
pub struct Update5 {
    pub user_reset_password_token__is_approved: bool,
}
pub struct By1<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
}
