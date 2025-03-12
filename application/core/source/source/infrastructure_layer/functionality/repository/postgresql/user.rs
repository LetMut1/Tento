use {
    super::{
        ParameterStorage,
        Postgresql,
    },
    crate::{
        domain_layer::data::entity::user::User,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::Repository,
        },
    },
    deadpool_postgres::Client,
    std::future::Future,
    tokio_postgres::types::Type,
};
impl Repository<Postgresql<User>> {
    // user__id: i64,
    pub fn create_1<'a>(database_1_client: &'a Client, insert: Insert1<'a>) -> impl Future<Output = Result<i64, AggregateError>> + Send + use<'a> {
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
                        nextval('public.user__1'),\
                        $1,\
                        $2,\
                        $3,\
                        $4\
                    ) \
                RETURNING \
                    u.id AS i;";
            let mut parameter_storage = ParameterStorage::new(4);
            parameter_storage
                .add(
                    &insert.user__email,
                    Type::TEXT,
                )
                .add(
                    &insert.user__nickname,
                    Type::TEXT,
                )
                .add(
                    &insert.user__password_hash,
                    Type::TEXT,
                )
                .add(
                    &insert.user__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            if rows.is_empty() {
                return Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(
                crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0))
            );
        };
    }
    pub fn create_2<'a>(database_1_client: &'a Client, insert: Insert2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
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
                        $1,\
                        $2,\
                        $3,\
                        $4,\
                        $5\
                    ) \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(5);
            parameter_storage
                .add(
                    &insert.user__id,
                    Type::INT8,
                )
                .add(
                    &insert.user__email,
                    Type::TEXT,
                )
                .add(
                    &insert.user__nickname,
                    Type::TEXT,
                )
                .add(
                    &insert.user__password_hash,
                    Type::TEXT,
                )
                .add(
                    &insert.user__created_at,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    pub fn update<'a>(database_1_client: &'a Client, update: Update<'a>, by: By3) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
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
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(2);
            parameter_storage
                .add(
                    &update.user__password_hash,
                    Type::TEXT,
                )
                .add(
                    &by.user__id,
                    Type::INT8,
                );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    pub fn delete<'a>(database_1_client: &'a Client, by: By3) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                DELETE FROM ONLY \
                    public.user_ AS u \
                WHERE \
                    u.id = $1 \
                RETURNING \
                    true AS _;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    // user__id: i64,
    pub fn find_1<'a>(database_1_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__nickname,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                    crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                ),
            );
        };
    }
    // user__id: i64,
    // user__email: String,
    // user__password_hash: String,
    pub fn find_2<'a>(database_1_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<Option<(i64, String, String)>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i,\
                    u.email AS e,\
                    u.password_hash AS ph \
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__nickname,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                    ),
                ),
            );
        };
    }
    // user__id: i64,
    // user__nickname: String,
    // user__password_hash: String,
    pub fn find_3<'a>(database_1_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<Option<(i64, String, String)>, AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__email,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                    ),
                ),
            );
        };
    }
    // user__id: i64,
    pub fn find_4<'a>(database_1_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<Option<i64>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.email = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__email,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                    crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)),
                ),
            );
        };
    }
    // user__email: String,
    // user__nickname: String,
    // user__password_hash: String,
    pub fn find_5<'a>(database_1_client: &'a Client, by: By3) -> impl Future<Output = Result<Option<(String, String, String)>, AggregateError>> + Send + use<'a> {
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
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(1)),
                        crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(2)),
                    ),
                ),
            );
        };
    }
    // user__email: String,
    pub fn find_6<'a>(database_1_client: &'a Client, by: By3) -> impl Future<Output = Result<Option<String>, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.email AS e \
                FROM \
                    public.user_ u \
                WHERE \
                    u.id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
                    crate::result_return_logic!(rows[0].try_get::<'_, usize, String>(0)),
                ),
            );
        };
    }
    pub fn is_exist_1<'a>(database_1_client: &'a Client, by: By1<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.nickname = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__nickname,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    pub fn is_exist_2<'a>(database_1_client: &'a Client, by: By2<'a>) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.email = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__email,
                Type::TEXT,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    pub fn is_exist_3<'a>(database_1_client: &'a Client, by: By3) -> impl Future<Output = Result<bool, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT \
                    u.id AS i \
                FROM \
                    public.user_ u \
                WHERE \
                    u.id = $1;";
            let mut parameter_storage = ParameterStorage::new(1);
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
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
    // user__id: i64,
    pub fn get_id<'a>(database_1_client: &'a Client) -> impl Future<Output = Result<i64, AggregateError>> + Send + use<'a> {
        return async move {
            let query = "\
                SELECT nextval('public.user__1') AS i;";
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query,
                    [].as_slice(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    [].as_slice(),
                )
                .await
            );
            if rows.is_empty() {
                return Err(crate::new_logic_unreachable_state!());
            }
            return Result::Ok(crate::result_return_logic!(rows[0].try_get::<'_, usize, i64>(0)));
        };
    }
}
pub struct Insert1<'a> {
    pub user__email: &'a str,
    pub user__nickname: &'a str,
    pub user__password_hash: &'a str,
    pub user__created_at: i64,
}
pub struct Insert2<'a> {
    pub user__id: i64,
    pub user__email: &'a str,
    pub user__nickname: &'a str,
    pub user__password_hash: &'a str,
    pub user__created_at: i64,
}
pub struct Update<'a> {
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
