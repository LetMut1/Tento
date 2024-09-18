use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::application_user::{
        derivative::{
            ApplicationUser1,
            ApplicationUser2,
            ApplicationUser3,
            ApplicationUser4,
            ApplicationUser5,
        },
        ApplicationUser,
    },
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
impl PostgresqlRepository<ApplicationUser<'_>> {
    pub fn create_1<'a>(
        database_1_connection: &'a Connection,
        insert_1: Insert1,
    ) -> impl Future<Output = Result<ApplicationUser<'static>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__email = insert_1.application_user__email.as_str();
            let application_user__nickname = insert_1.application_user__nickname.as_str();
            let application_user__password_hash = insert_1.application_user__password_hash.as_str();
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                INSERT INTO public.application_user AS au ( \
                    id, \
                    email, \
                    nickname, \
                    password_hash, \
                    created_at \
                ) VALUES ( \
                    nextval('public.application_user1'), \
                    $1, \
                    $2, \
                    $3, \
                    current_timestamp(6) \
                ) \
                RETURNING \
                    au.id AS i,
                    au.created_at::TEXT AS ca;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &application_user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &application_user__nickname,
                    Type::TEXT,
                )
                .add_parameter(
                    &application_user__password_hash,
                    Type::TEXT,
                );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                ApplicationUser::new(
                    row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    insert_1.application_user__email,
                    Cow::Owned(insert_1.application_user__nickname),
                    insert_1.application_user__password_hash,
                    row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                ),
            );
        };
    }
    pub fn update_1<'a>(database_1_connection: &'a Connection, update_1: Update1<'a>, by_3: By3) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                UPDATE ONLY public.application_user AS au \
                SET ( \
                    password_hash \
                ) = ROW( \
                    $1 \
                ) \
                WHERE au.id = $2 \
                RETURNING \
                    au.id AS i;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_1.application_user__password_hash,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_3.application_user__id,
                    Type::INT8,
                );
            let statement = database_1_connection
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
            database_1_connection
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
    pub fn is_exist_1<'a>(database_1_connection: &'a Connection, by_1: By1<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__nickname = by_1.application_user__nickname;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i \
                FROM public.application_user au \
                WHERE au.nickname = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__nickname,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
    pub fn is_exist_2<'a>(database_1_connection: &'a Connection, by_2: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__email = by_2.application_user__email;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i \
                FROM public.application_user au \
                WHERE au.email = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__email,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
    pub fn is_exist_3<'a>(database_1_connection: &'a Connection, by_3: By3) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i \
                FROM public.application_user au \
                WHERE au.id = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.application_user__id,
                Type::INT8,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(false);
            }
            return Result::Ok(true);
        };
    }
    pub fn find_1<'a, 'b>(
        database_1_connection: &'a Connection,
        by_1: By1<'b>,
    ) -> impl Future<Output = Result<Option<ApplicationUser<'b>>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__nickname = by_1.application_user__nickname;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i, \
                    au.email AS e, \
                    au.password_hash AS ph, \
                    au.created_at::TEXT AS ca \
                FROM public.application_user au \
                WHERE au.nickname = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__nickname,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser::new(
                        row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        Cow::Borrowed(by_1.application_user__nickname),
                        row_registry[0].try_get::<'_, usize, String>(2).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        row_registry[0].try_get::<'_, usize, String>(3).into_logic(
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
    pub fn find_2<'a>(database_1_connection: &'a Connection, by_1: By1<'a>) -> impl Future<Output = Result<Option<ApplicationUser1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__nickname = by_1.application_user__nickname;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i, \
                    au.email AS e, \
                    au.password_hash AS ph \
                FROM public.application_user au \
                WHERE au.nickname = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__nickname,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser1 {
                        id: row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        email: row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        password_hash: row_registry[0].try_get::<'_, usize, String>(2).into_logic(
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
    pub fn find_3<'a>(database_1_connection: &'a Connection, by_2: By2<'a>) -> impl Future<Output = Result<Option<ApplicationUser2>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__email = by_2.application_user__email;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i, \
                    au.nickname AS n, \
                    au.password_hash AS ph \
                FROM public.application_user au \
                WHERE au.email = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__email,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser2 {
                        id: row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        nickname: row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        password_hash: row_registry[0].try_get::<'_, usize, String>(2).into_logic(
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
    pub fn find_4<'a>(database_1_connection: &'a Connection, by_2: By2<'a>) -> impl Future<Output = Result<Option<ApplicationUser3>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user__email = by_2.application_user__email;
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.id AS i \
                FROM public.application_user au \
                WHERE au.email = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &application_user__email,
                Type::TEXT,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser3 {
                        id: row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
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
    pub fn find_5<'a>(database_1_connection: &'a Connection, by_3: By3) -> impl Future<Output = Result<Option<ApplicationUser4>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.email AS e, \
                    au.nickname AS n, \
                    au.password_hash AS ph \
                FROM public.application_user au \
                WHERE au.id = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.application_user__id,
                Type::INT8,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser4 {
                        email: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        nickname: row_registry[0].try_get::<'_, usize, String>(1).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        password_hash: row_registry[0].try_get::<'_, usize, String>(2).into_logic(
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
    pub fn find_6<'a>(database_1_connection: &'a Connection, by_3: By3) -> impl Future<Output = Result<Option<ApplicationUser5>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                SELECT \
                    au.email AS e \
                FROM public.application_user au \
                WHERE au.id = $1;";
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.application_user__id,
                Type::INT8,
            );
            let statement = database_1_connection
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
            let row_registry = database_1_connection
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
                return Result::Ok(None);
            }
            return Result::Ok(
                Some(
                    ApplicationUser5 {
                        email: row_registry[0].try_get::<'_, usize, String>(0).into_logic(
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
pub struct Insert1 {
    pub application_user__email: String,
    pub application_user__nickname: String,
    pub application_user__password_hash: String,
}
pub struct Update1<'a> {
    pub application_user__password_hash: &'a str,
}
pub struct By1<'a> {
    pub application_user__nickname: &'a str,
}
pub struct By2<'a> {
    pub application_user__email: &'a str,
}
pub struct By3 {
    pub application_user__id: i64,
}
