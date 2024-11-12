use super::Postgresql;
use crate::infrastructure_layer::functionality::repository::Repository;
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
        functionality::service::postgresql_prepared_statemant_parameter_storage::PostgresqlPreparedStatementParameterStorage,
    },
};
use dedicated_crate::void::Void;
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl Repository<Postgresql<UserResetPasswordToken<'_>>> {
    pub fn create_1<'a, 'b>(
        database_2_client: &'a Client,
        insert_1: Insert1<'b>,
    ) -> impl Future<Output = Result<UserResetPasswordToken<'b>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
                .add(
                    &insert_1.user__id,
                    Type::INT8,
                )
                .add(
                    &insert_1.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &insert_1.user_reset_password_token__value,
                    Type::TEXT,
                )
                .add(
                    &insert_1.user_reset_password_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &insert_1.user_reset_password_token__is_approved,
                    Type::BOOL,
                )
                .add(
                    &insert_1.user_reset_password_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &insert_1.user_reset_password_token__can_be_resent_from,
                    Type::INT8,
                );
            let statement = database_2_client
                .prepare_typed_cached(
                    query,
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(
                UserResetPasswordToken::new(
                    insert_1.user__id,
                    Cow::Borrowed(insert_1.user_device__id),
                    insert_1.user_reset_password_token__value,
                    insert_1.user_reset_password_token__wrong_enter_tries_quantity,
                    insert_1.user_reset_password_token__is_approved,
                    insert_1.user_reset_password_token__expires_at,
                    insert_1.user_reset_password_token__can_be_resent_from,
                ),
            );
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
    pub fn update_1<'a>(
        database_2_client: &'a Client,
        update_1: Update1<'a>,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
    pub fn update_3<'a>(
        database_2_client: &'a Client,
        update_3: Update3<'a>,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
    pub fn find_1<'a>(
        database_2_client: &'a Client,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserResetPasswordToken_1>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
    pub fn find_2<'a>(
        database_2_client: &'a Client,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserResetPasswordToken_2>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
    pub fn find_3<'a>(
        database_2_client: &'a Client,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserResetPasswordToken_3>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut postgresql_prepared_statemant_parameter_storage = PostgresqlPreparedStatementParameterStorage::new();
            postgresql_prepared_statemant_parameter_storage
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_type_registry(),
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
                    postgresql_prepared_statemant_parameter_storage.get_parameter_registry(),
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
pub struct Insert1<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
    pub user_reset_password_token__value: String,
    pub user_reset_password_token__wrong_enter_tries_quantity: i16,
    pub user_reset_password_token__is_approved: bool,
    pub user_reset_password_token__expires_at: i64,
    pub user_reset_password_token__can_be_resent_from: i64,
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
