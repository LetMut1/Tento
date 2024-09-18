use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
    infrastructure_layer::{
        data::capture::Capture,
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::{
    types::Type,
    Client as Connection,
};
use void::Void;
impl PostgresqlRepository<ApplicationUserAccessRefreshToken<'_>> {
    pub fn create_1<'a, 'b>(
        database_2_connection: &'a Connection,
        insert_1: Insert1<'b>,
    ) -> impl Future<Output = Result<ApplicationUserAccessRefreshToken<'b>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user_access_refresh_token__obfuscation_value = insert_1.application_user_access_refresh_token__obfuscation_value.as_str();
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                INSERT INTO public.application_user_access_refresh_token AS auart ( \
                    application_user__id, \
                    application_user_device__id, \
                    application_user_access_token__id, \
                    obfuscation_value, \
                    expires_at, \
                    updated_at \
                ) VALUES ( \
                    $1, \
                    $2, \
                    $3, \
                    $4, \
                    $5, \
                    $6 \
                );";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.application_user_device__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.application_user_access_token__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &application_user_access_refresh_token__obfuscation_value,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.application_user_access_refresh_token__expires_at,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.application_user_access_refresh_token__updated_at,
                    Type::INT8,
                );
            let statement = database_2_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_2_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(
                ApplicationUserAccessRefreshToken::new(
                    insert_1.application_user__id,
                    Cow::Borrowed(insert_1.application_user_device__id),
                    Cow::Borrowed(insert_1.application_user_access_token__id),
                    insert_1.application_user_access_refresh_token__obfuscation_value,
                    insert_1.application_user_access_refresh_token__expires_at,
                    insert_1.application_user_access_refresh_token__updated_at,
                ),
            );
        };
    }
    pub fn update_1<'a, 'b>(
        database_2_connection: &'a Connection,
        update_1: Update1<'a>,
        by_2: By2<'a>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                UPDATE ONLY public.application_user_access_refresh_token AS auart \
                SET ( \
                    application_user_access_token__id, \
                    obfuscation_value, \
                    expires_at, \
                    updated_at
                ) = ROW( \
                    $1, \
                    $2, \
                    $3, \
                    $4 \
                ) \
                WHERE auart.application_user__id = $5 AND auart.application_user_device__id = $6;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_1.application_user_access_token__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &update_1.application_user_access_refresh_token__obfuscation_value,
                    Type::TEXT,
                )
                .add_parameter(
                    &update_1.application_user_access_refresh_token__expires_at,
                    Type::INT8,
                )
                .add_parameter(
                    &update_1.application_user_access_refresh_token__updated_at,
                    Type::INT8,
                )
                .add_parameter(
                    &by_2.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_2.application_user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_2_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
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
    pub fn delete_1<'a>(database_2_connection: &'a Connection, by_2: By2<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
                WHERE auart.application_user__id = $1 AND auart.application_user_device__id = $2;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_2.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_2.application_user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_2_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
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
    pub fn delete_2<'a>(database_2_connection: &'a Connection, by_1: By1) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
                WHERE auart.application_user__id = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_1.application_user__id,
                Type::INT8,
            );
            let statement = database_2_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_2_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
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
        database_2_connection: &'a Connection,
        by_2: By2<'b>,
    ) -> impl Future<Output = Result<Option<ApplicationUserAccessRefreshToken<'b>>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    auart.application_user_access_token__id AS auati, \
                    auart.obfuscation_value AS ov, \
                    auart.expires_at AS ea, \
                    auart.updated_at AS ua \
                FROM public.application_user_access_refresh_token auart \
                WHERE auart.application_user__id = $1 AND auart.application_user_device__id = $2;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_2.application_user__id,
                    Type::INT8,
                )
                .add_parameter(
                    &by_2.application_user_device__id,
                    Type::TEXT,
                );
            let statement = database_2_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            let row_registry = database_2_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
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
                    ApplicationUserAccessRefreshToken::new(
                        by_2.application_user__id,
                        Cow::Borrowed(by_2.application_user_device__id),
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
pub struct Insert1<'a> {
    pub application_user__id: i64,
    pub application_user_device__id: &'a str,
    pub application_user_access_token__id: &'a str,
    pub application_user_access_refresh_token__obfuscation_value: String,
    pub application_user_access_refresh_token__expires_at: i64,
    pub application_user_access_refresh_token__updated_at: i64,
}
pub struct Update1<'a> {
    pub application_user_access_token__id: &'a str,
    pub application_user_access_refresh_token__obfuscation_value: &'a str,
    pub application_user_access_refresh_token__expires_at: i64,
    pub application_user_access_refresh_token__updated_at: i64,
}
pub struct By1 {
    pub application_user__id: i64,
}
pub struct By2<'a> {
    pub application_user__id: i64,
    pub application_user_device__id: &'a str,
}
