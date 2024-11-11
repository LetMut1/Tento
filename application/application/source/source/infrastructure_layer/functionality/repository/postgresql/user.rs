use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::user::{
        User,
        User_1,
        User_2,
        User_3,
        User_4,
        User_5,
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
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use dedicated_crate::void::Void;
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::types::Type;
use deadpool_postgres::Client;
impl PostgresqlRepository<User<'_>> {
    pub fn create_1<'a>(database_1_client: &'a Client, insert_1: Insert1) -> impl Future<Output = Result<User<'static>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                INSERT INTO \
                    public.user_ AS u (\
                    id,\
                    email,\
                    nickname,\
                    password_hash,\
                    created_at\
                ) VALUES (\
                    nextval('public.user_1'),\
                    $1,\
                    $2,\
                    $3,\
                    $4 \
                ) \
                RETURNING \
                    u.id AS i;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &insert_1.user__email,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user__nickname,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user__password_hash,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.user__created_at,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                User::new(
                    row_registry[0].try_get::<'_, usize, i64>(0).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    insert_1.user__email,
                    Cow::Owned(insert_1.user__nickname),
                    insert_1.user__password_hash,
                    insert_1.user__created_at,
                ),
            );
        };
    }
    pub fn update_1<'a>(database_1_client: &'a Client, update_1: Update1<'a>, by_3: By3) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                UPDATE ONLY \
                    public.user_ AS u \
                SET (\
                    password_hash\
                ) = ROW(\
                    $1\
                ) \
                WHERE \
                    u.id = $2 \
                RETURNING \
                    u.id AS i;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &update_1.user__password_hash,
                    Type::TEXT,
                )
                .add_parameter(
                    &by_3.user__id,
                    Type::INT8,
                );
            let statement = database_1_client
                .prepare_typed_cached(
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
            database_1_client
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
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_1.user__nickname,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
    pub fn is_exist_2<'a>(database_1_client: &'a Client, by_2: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.email = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_2.user__email,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
    pub fn is_exist_3<'a>(database_1_client: &'a Client, by_3: By3) -> impl Future<Output = Result<bool, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.id = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.user__id,
                Type::INT8,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
    pub fn find_1<'a, 'b>(database_1_client: &'a Client, by_1: By1<'b>) -> impl Future<Output = Result<Option<User<'b>>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i,\
                    u.email AS e,\
                    u.password_hash AS ph,\
                    u.created_at AS ca \
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_1.user__nickname,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User::new(
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
                        Cow::Borrowed(by_1.user__nickname),
                        row_registry[0].try_get::<'_, usize, String>(2).into_logic(
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
    pub fn find_2<'a>(database_1_client: &'a Client, by_1: By1<'a>) -> impl Future<Output = Result<Option<User_1>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i,\
                    u.email AS e,\
                    u.password_hash AS ph\
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_1.user__nickname,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User_1 {
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
    pub fn find_3<'a>(database_1_client: &'a Client, by_2: By2<'a>) -> impl Future<Output = Result<Option<User_2>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i,\
                    u.nickname AS n,\
                    u.password_hash AS ph \
                FROM \
                    public.user_ u \
                WHERE \
                    u.email = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_2.user__email,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User_2 {
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
    pub fn find_4<'a>(database_1_client: &'a Client, by_2: By2<'a>) -> impl Future<Output = Result<Option<User_3>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.email = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_2.user__email,
                Type::TEXT,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User_3 {
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
    pub fn find_5<'a>(database_1_client: &'a Client, by_3: By3) -> impl Future<Output = Result<Option<User_4>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.email AS e,\
                    u.nickname AS n,\
                    u.password_hash AS ph \
                FROM \
                    public.user_ u \
                WHERE \
                    u.id = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.user__id,
                Type::INT8,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User_4 {
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
    pub fn find_6<'a>(database_1_client: &'a Client, by_3: By3) -> impl Future<Output = Result<Option<User_5>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "\
                SELECT \
                    u.email AS e \
                FROM \
                    public.user_ u \
                WHERE \
                    u.id = $1;";
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &by_3.user__id,
                Type::INT8,
            );
            let statement = database_1_client
                .prepare_typed_cached(
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
            let row_registry = database_1_client
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
                    User_5 {
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
    pub user__email: String,
    pub user__nickname: String,
    pub user__password_hash: String,
    pub user__created_at: i64,
}
pub struct Update1<'a> {
    pub user__password_hash: &'a str,
}
pub struct By1<'a> {
    pub user__nickname: &'a str,
}
pub struct By2<'a> {
    pub user__email: &'a str,
}
pub struct By3 {
    pub user__id: i64,
}
