use super::{
    Postgresql,
    ParameterStorage,
};
use crate::{
    domain_layer::data::entity::user_authorization_token::{
        UserAuthorizationToken,
        UserAuthorizationToken_1,
        UserAuthorizationToken_2,
        UserAuthorizationToken_3,
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
impl Repository<Postgresql<UserAuthorizationToken<'_>>> {
    pub fn create_1<'a, 'b>(
        database_2_client: &'a Client,
        user_authorization_token: &'a UserAuthorizationToken<'b>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.user_authorization_token AS uat (\
                        user__id,\
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
                    );";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &user_authorization_token.user__id,
                    Type::INT8,
                )
                .add(
                    &user_authorization_token.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &user_authorization_token.value,
                    Type::TEXT,
                )
                .add(
                    &user_authorization_token.wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &user_authorization_token.expires_at,
                    Type::INT8,
                )
                .add(
                    &user_authorization_token.can_be_resent_from,
                    Type::INT8,
                );
            let statement = database_2_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
    pub fn delete_1<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_authorization_token AS uat \
                WHERE \
                    uat.user__id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
                    uat.user__id = $5 \
                    AND uat.user_device__id = $6;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update_1.user_authorization_token__value,
                    Type::TEXT,
                )
                .add(
                    &update_1.user_authorization_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update_1.user_authorization_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update_1.user_authorization_token__can_be_resent_from,
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
    pub fn update_2<'a>(database_2_client: &'a Client, update_2: Update2<'a>, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
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
                    uat.user__id = $4 \
                    AND uat.user_device__id = $5;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update_2.user_authorization_token__value,
                    Type::TEXT,
                )
                .add(
                    &update_2.user_authorization_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add(
                    &update_2.user_authorization_token__expires_at,
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
    pub fn update_3<'a>(database_2_client: &'a Client, update_3: Update3, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET (\
                    can_be_resent_from\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    uat.user__id = $2 \
                    AND uat.user_device__id = $3;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update_3.user_authorization_token__can_be_resent_from,
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
    pub fn update_4<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_authorization_token AS uat \
                SET \
                    wrong_enter_tries_quantity = wrong_enter_tries_quantity + 1 \
                WHERE \
                    uat.user__id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
    pub fn find_1<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserAuthorizationToken_1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    uat.value AS v,\
                    uat.wrong_enter_tries_quantity AS wetq,\
                    uat.expires_at AS ea,\
                    uat.can_be_resent_from AS cbrf \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
                    UserAuthorizationToken_1 {
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
    pub fn find_2<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserAuthorizationToken_2>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    uat.value AS v,\
                    uat.wrong_enter_tries_quantity AS wetq,\
                    uat.expires_at AS ea \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
                    UserAuthorizationToken_2 {
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
                        expires_at: row_registry[0].try_get::<'_, usize, i64>(2).into_logic(
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
    pub fn find_3<'a>(database_2_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<UserAuthorizationToken_3>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    uat.value AS v,\
                    uat.expires_at AS ea,\
                    uat.can_be_resent_from AS cbrf \
                FROM \
                    public.user_authorization_token uat \
                WHERE \
                    uat.user__id = $1 \
                    AND uat.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
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
                    parameter_storage.get_parameter_type_registry(),
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
                    parameter_storage.get_parameter_registry(),
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
                    UserAuthorizationToken_3 {
                        value: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        expires_at: row_registry[0].try_get::<'_, usize, i64>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(2).into_logic(
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
    pub user_authorization_token__value: &'a str,
    pub user_authorization_token__wrong_enter_tries_quantity: i16,
    pub user_authorization_token__expires_at: i64,
    pub user_authorization_token__can_be_resent_from: i64,
}
pub struct Update2<'a> {
    pub user_authorization_token__value: &'a str,
    pub user_authorization_token__wrong_enter_tries_quantity: i16,
    pub user_authorization_token__expires_at: i64,
}
pub struct Update3 {
    pub user_authorization_token__can_be_resent_from: i64,
}
pub struct By1<'a> {
    pub user__id: i64,
    pub user_device__id: &'a str,
}
