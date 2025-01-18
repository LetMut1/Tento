use super::{
    Postgresql,
    ParameterStorage,
};
use crate::{
    domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken,
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
use dedicated::void::Void;
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::types::Type;
impl Repository<Postgresql<UserAccessRefreshToken<'_>>> {
    pub fn create_1<'a, 'b>(
        database_2_client: &'a Client,
        user_access_refresh_token: &'a UserAccessRefreshToken<'b>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
                    );";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &user_access_refresh_token.user__id,
                    Type::INT8,
                )
                .add(
                    &user_access_refresh_token.user_device__id,
                    Type::TEXT,
                )
                .add(
                    &user_access_refresh_token.user_access_token__id,
                    Type::TEXT,
                )
                .add(
                    &user_access_refresh_token.obfuscation_value,
                    Type::TEXT,
                )
                .add(
                    &user_access_refresh_token.expires_at,
                    Type::INT8,
                )
                .add(
                    &user_access_refresh_token.updated_at,
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
    pub fn update_1<'a>(database_2_client: &'a Client, update_1: Update1<'a>, by_2: By2<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
                    AND uart.user_device__id = $6;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &update_1.user_access_token__id,
                    Type::TEXT,
                )
                .add(
                    &update_1.user_access_refresh_token__obfuscation_value,
                    Type::TEXT,
                )
                .add(
                    &update_1.user_access_refresh_token__expires_at,
                    Type::INT8,
                )
                .add(
                    &update_1.user_access_refresh_token__updated_at,
                    Type::INT8,
                )
                .add(
                    &by_2.user__id,
                    Type::INT8,
                )
                .add(
                    &by_2.user_device__id,
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
    pub fn delete_1<'a>(database_2_client: &'a Client, by_2: By2<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
            DELETE FROM ONLY \
                public.user_access_refresh_token AS uart \
            WHERE \
                uart.user__id = $1 \
                AND uart.user_device__id = $2;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_2.user__id,
                    Type::INT8,
                )
                .add(
                    &by_2.user_device__id,
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
    pub fn delete_2<'a>(database_2_client: &'a Client, by_1: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_access_refresh_token AS uart \
                WHERE \
                    uart.user__id = $1;";
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by_1.user__id,
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
    pub fn find_1<'a, 'b>(
        database_2_client: &'a Client,
        by_2: By2<'b>,
    ) -> impl Future<Output = Result<Option<UserAccessRefreshToken<'b>>, AggregateError>> + Send + Capture<&'a Void> {
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
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by_2.user__id,
                    Type::INT8,
                )
                .add(
                    &by_2.user_device__id,
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
                    UserAccessRefreshToken::new(
                        by_2.user__id,
                        by_2.user_device__id,
                        Cow::Owned(
                            row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            )?,
                        ),
                        row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, i64>(3).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                    ),
                ),
            );
        };
    }
}
pub struct Update1<'a> {
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
