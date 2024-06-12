use super::by::By3;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::derivative::ApplicationUser1;
use crate::domain_layer::data::entity::application_user::derivative::ApplicationUser2;
use crate::domain_layer::data::entity::application_user::derivative::ApplicationUser3;
use crate::domain_layer::data::entity::application_user::derivative::ApplicationUser4;
use crate::domain_layer::data::entity::application_user::derivative::ApplicationUser5;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUser<'_>> {
    pub async fn create_1<'a>(
        database_1_connection: &'a Connection,
        insert_1: Insert1,
    ) -> Result<ApplicationUser<'static>, Auditor<Error>> {
        let application_user_email = insert_1.application_user_email.as_str();

        let application_user_nickname = insert_1.application_user_nickname.as_str();

        let application_user_password_hash = insert_1.application_user_password_hash.as_str();

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
                &application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_nickname,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_password_hash,
                Type::TEXT,
            );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        return Ok(
            ApplicationUser::new(
                row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                insert_1.application_user_email,
                Cow::Owned(insert_1.application_user_nickname),
                insert_1.application_user_password_hash,
                row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
            ),
        );
    }

    pub async fn update_1<'a>(
        database_1_connection: &'a Connection,
        update_1: &'a Update1<'_>,
        by_3: &'a By3,
    ) -> Result<(), Auditor<Error>> {
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
                &update_1.application_user_password_hash,
                Type::TEXT,
            )
            .add_parameter(
                &by_3.application_user_id,
                Type::INT8,
            );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn is_exist_1<'a>(
        database_1_connection: &'a Connection,
        by_1: &'a By1<'_>,
    ) -> Result<bool, Auditor<Error>> {
        let application_user_nickname = by_1.application_user_nickname;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_nickname,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn is_exist_2<'a>(
        database_1_connection: &'a Connection,
        by_2: &'a By2<'_>,
    ) -> Result<bool, Auditor<Error>> {
        let application_user_email = by_2.application_user_email;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_email,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn is_exist_3<'a>(
        database_1_connection: &'a Connection,
        by_3: &'a By3,
    ) -> Result<bool, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_3.application_user_id,
            Type::INT8,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn find_1<'a, 'b>(
        database_1_connection: &'a Connection,
        by_1: &'a By1<'b>,
    ) -> Result<Option<ApplicationUser<'b>>, Auditor<Error>> {
        let application_user_nickname = by_1.application_user_nickname;

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
            &application_user_nickname,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser::new(
                    row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                    row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                    Cow::Borrowed(by_1.application_user_nickname),
                    row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                    row_registry[0].try_get::<'_, usize, String>(3).convert(Backtrace::new(line!(), file!()))?,
                ),
            ),
        );
    }

    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        by_1: &'a By1<'_>,
    ) -> Result<Option<ApplicationUser1>, Auditor<Error>> {
        let application_user_nickname = by_1.application_user_nickname;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i, \
                au.email AS e, \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_nickname,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser1 {
                    id: row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                    email: row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                    password_hash: row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        by_2: &'a By2<'_>,
    ) -> Result<Option<ApplicationUser2>, Auditor<Error>> {
        let application_user_email = by_2.application_user_email;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i, \
                au.nickname AS n, \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_email,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser2 {
                    id: row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                    nickname: row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                    password_hash: row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_4<'a>(
        database_1_connection: &'a Connection,
        by_2: &'a By2<'_>,
    ) -> Result<Option<ApplicationUser3>, Auditor<Error>> {
        let application_user_email = by_2.application_user_email;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_email,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser3 {
                    id: row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_5<'a>(
        database_1_connection: &'a Connection,
        by_3: &'a By3,
    ) -> Result<Option<ApplicationUser4>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_3.application_user_id,
            Type::INT8,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser4 {
                    email: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    nickname: row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                    password_hash: row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_6<'a>(
        database_1_connection: &'a Connection,
        by_3: &'a By3,
    ) -> Result<Option<ApplicationUser5>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                au.email AS e \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_3.application_user_id,
            Type::INT8,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUser5 {
                    email: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }
}

pub struct Insert1 {
    pub application_user_email: String,
    pub application_user_nickname: String,
    pub application_user_password_hash: String,
}

pub struct Update1<'a> {
    pub application_user_password_hash: &'a str,
}

pub struct By1<'a> {
    pub application_user_nickname: &'a str,
}

pub struct By2<'a> {
    pub application_user_email: &'a str,
}