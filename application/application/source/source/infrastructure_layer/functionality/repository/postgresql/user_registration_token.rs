use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::user_registration_token::{
        UserRegistrationToken_1,
        UserRegistrationToken_2,
        UserRegistrationToken_3,
        UserRegistrationToken,
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use crate::infrastructure_layer::data::aggregate_error::{
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
use dedicated_crate::void::Void;
impl PostgresqlRepository<UserRegistrationToken<'_>> {
    pub fn create_1<'a, 'b>(
        database_2_connection: &'a Connection,
        insert_1: Insert1<'b>,
    ) -> impl Future<Output = Result<UserRegistrationToken<'b>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO public.user_registration_token AS urt ( \
                    user__email, \
                    user_device__id, \
                    value, \
                    wrong_enter_tries_quantity, \
                    is_approved, \
                    expires_at, \
                    can_be_resent_from \
                ) VALUES ( \
                    $1, \
                    $2, \
                    $3, \
                    $4, \
                    $5, \
                    $6, \
                    $7 \
                );";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user_device__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user_registration_token__value,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user_registration_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add_parameter(
                    &insert_1.user_registration_token__is_approved,
                    Type::BOOL,
                )
                .add_parameter(
                    &insert_1.user_registration_token__expires_at,
                    Type::INT8,
                )
                .add_parameter(
                    &insert_1.user_registration_token__can_be_resent_from,
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
                UserRegistrationToken::new(
                    Cow::Borrowed(insert_1.user__email),
                    Cow::Borrowed(insert_1.user_device__id),
                    insert_1.user_registration_token__value,
                    insert_1.user_registration_token__wrong_enter_tries_quantity,
                    insert_1.user_registration_token__is_approved,
                    insert_1.user_registration_token__expires_at,
                    insert_1.user_registration_token__can_be_resent_from,
                ),
            );
        };
    }
    pub fn delete_2<'a>(database_2_connection: &'a Connection, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                DELETE FROM ONLY public.user_registration_token AS urt \
                WHERE urt.user__email = $1 AND urt.user_device__id = $2;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn update_1<'a>(
        database_2_connection: &'a Connection,
        update_1: Update1<'a>,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY public.user_registration_token AS urt
                SET ( \
                    value, \
                    wrong_enter_tries_quantity, \
                    is_approved, \
                    can_be_resent_from, \
                    expires_at \
                ) = ROW( \
                    $1, \
                    $2, \
                    $3, \
                    $4, \
                    $5 \
                ) \
                WHERE urt.user__email = $6 AND urt.user_device__id = $7;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_1.user_registration_token__value,
                    Type::TEXT,
                )
                .add_parameter(
                    &update_1.user_registration_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add_parameter(
                    &update_1.user_registration_token__is_approved,
                    Type::BOOL,
                )
                .add_parameter(
                    &update_1.user_registration_token__expires_at,
                    Type::INT8,
                )
                .add_parameter(
                    &update_1.user_registration_token__can_be_resent_from,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn update_2<'a>(database_2_connection: &'a Connection, update_2: Update2, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY public.user_registration_token AS urt
                SET ( \
                    can_be_resent_from \
                ) = ROW( \
                    $1 \
                ) \
                WHERE urt.user__email = $2 AND urt.user_device__id = $3;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_2.user_registration_token__can_be_resent_from,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn update_3<'a>(
        database_2_connection: &'a Connection,
        update_3: Update3<'a>,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY public.user_registration_token AS urt
                SET ( \
                    value, \
                    wrong_enter_tries_quantity, \
                    is_approved, \
                    expires_at \
                ) = ROW( \
                    $1, \
                    $2, \
                    $3, \
                    $4 \
                ) \
                WHERE urt.user__email = $5 AND urt.user_device__id = $6;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_3.user_registration_token__value,
                    Type::TEXT,
                )
                .add_parameter(
                    &update_3.user_registration_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add_parameter(
                    &update_3.user_registration_token__is_approved,
                    Type::BOOL,
                )
                .add_parameter(
                    &update_3.user_registration_token__expires_at,
                    Type::INT8,
                )
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn update_4<'a>(database_2_connection: &'a Connection, update_4: Update4, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY public.user_registration_token AS urt
                SET ( \
                    wrong_enter_tries_quantity \
                ) = ROW( \
                    $1 \
                ) \
                WHERE urt.user__email = $2 AND urt.user_device__id = $3;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_4.user_registration_token__wrong_enter_tries_quantity,
                    Type::INT2,
                )
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn update_5<'a>(database_2_connection: &'a Connection, update_5: Update5, by_1: By1<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY public.user_registration_token AS urt
                SET ( \
                    is_approved \
                ) = ROW( \
                    $1 \
                ) \
                WHERE urt.user__email = $2 AND urt.user_device__id = $3;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_5.user_registration_token__is_approved,
                    Type::BOOL,
                )
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
    pub fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserRegistrationToken_1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    urt.value AS v, \
                    urt.wrong_enter_tries_quantity AS wetq, \
                    urt.is_approved AS ia, \
                    urt.expires_at AS ea, \
                    urt.can_be_resent_from as cbrf \
                FROM public.user_registration_token urt \
                WHERE urt.user__email = $1 AND urt.user_device__id = $2;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
                    UserRegistrationToken_1 {
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
        database_2_connection: &'a Connection,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserRegistrationToken_2>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    urt.value AS v, \
                    urt.wrong_enter_tries_quantity AS wetq, \
                    urt.is_approved AS ia, \
                    urt.expires_at AS ea \
                FROM public.user_registration_token urt \
                WHERE urt.user__email = $1 AND urt.user_device__id = $2;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
                    UserRegistrationToken_2 {
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
        database_2_connection: &'a Connection,
        by_1: By1<'a>,
    ) -> impl Future<Output = Result<Option<UserRegistrationToken_3>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    urt.value AS v, \
                    urt.is_approved AS ia, \
                    urt.expires_at AS ea, \
                    urt.can_be_resent_from as cbrf \
                FROM public.user_registration_token urt \
                WHERE urt.user__email = $1 AND urt.user_device__id = $2;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &by_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_1.user_device__id,
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
                    UserRegistrationToken_3 {
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
    pub user__email: &'a str,
    pub user_device__id: &'a str,
    pub user_registration_token__value: String,
    pub user_registration_token__wrong_enter_tries_quantity: i16,
    pub user_registration_token__is_approved: bool,
    pub user_registration_token__expires_at: i64,
    pub user_registration_token__can_be_resent_from: i64,
}
pub struct Update1<'a> {
    pub user_registration_token__value: &'a str,
    pub user_registration_token__wrong_enter_tries_quantity: i16,
    pub user_registration_token__is_approved: bool,
    pub user_registration_token__expires_at: i64,
    pub user_registration_token__can_be_resent_from: i64,
}
pub struct Update2 {
    pub user_registration_token__can_be_resent_from: i64,
}
pub struct Update3<'a> {
    pub user_registration_token__value: &'a str,
    pub user_registration_token__wrong_enter_tries_quantity: i16,
    pub user_registration_token__is_approved: bool,
    pub user_registration_token__expires_at: i64,
}
pub struct Update4 {
    pub user_registration_token__wrong_enter_tries_quantity: i16,
}
pub struct Update5 {
    pub user_registration_token__is_approved: bool,
}
pub struct By1<'a> {
    pub user__email: &'a str,
    pub user_device__id: &'a str,
}
