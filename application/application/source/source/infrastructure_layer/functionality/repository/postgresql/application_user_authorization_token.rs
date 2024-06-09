use super::by::By4;
use super::update::Update3;
use super::update::Update4;
use super::update::Update5;
use super::update::Update6;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::derivative::ApplicationUserAuthorizationToken1;
use crate::domain_layer::data::entity::application_user_authorization_token::derivative::ApplicationUserAuthorizationToken2;
use crate::domain_layer::data::entity::application_user_authorization_token::derivative::ApplicationUserAuthorizationToken3;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserAuthorizationToken<'_>> {
    pub async fn create_1<'a>(
        database_2_connection: &'a Connection,
        insert_3: Insert3<'a>,
    ) -> Result<ApplicationUserAuthorizationToken<'a>, Auditor<Error>> {
        let application_user_authorization_token_value = insert_3.application_user_authorization_token_value.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_authorization_token AS auat ( \
                application_user_id, \
                application_user_device_id, \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
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
                &insert_3.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &insert_3.application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_can_be_resent_from,
                Type::INT8,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(
            ApplicationUserAuthorizationToken::new(
                insert_3.application_user_id,
                Cow::Borrowed(insert_3.application_user_device_id),
                insert_3.application_user_authorization_token_value,
                insert_3.application_user_authorization_token_wrong_enter_tries_quantity,
                insert_3.application_user_authorization_token_expires_at,
                insert_3.application_user_authorization_token_can_be_resent_from,
            ),
        );
    }

    pub async fn delete_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_authorization_token AS auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn update_1<'a>(
        database_2_connection: &'a Connection,
        update_3: &'a Update3<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4 \
            ) \
            WHERE auat.application_user_id = $5 AND auat.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_3.application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn update_2<'a>(
        database_2_connection: &'a Connection,
        update_4: &'a Update4<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at \
            ) = ROW( \
                $1, \
                $2, \
                $3 \
            ) \
            WHERE auat.application_user_id = $4 AND auat.application_user_device_id = $5;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_4.application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_4.application_user_authorization_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_4.application_user_authorization_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn update_3<'a>(
        database_2_connection: &'a Connection,
        update_5: &'a Update5,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_5.application_user_authorization_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn update_4<'a>(
        database_2_connection: &'a Connection,
        update_6: &'a Update6,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_6.application_user_authorization_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_2_connection
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
                ApplicationUserAuthorizationToken1 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?,
                    can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_2<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken2>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_2_connection
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
                ApplicationUserAuthorizationToken2 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_3<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken3>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_2_connection
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
                ApplicationUserAuthorizationToken3 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(1).convert(Backtrace::new(line!(), file!()))?,
                    can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }
}

pub struct Insert3<'a> {
    pub application_user_id: i64,
    pub application_user_device_id: &'a str,
    pub application_user_authorization_token_value: String,
    pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
    pub application_user_authorization_token_expires_at: i64,
    pub application_user_authorization_token_can_be_resent_from: i64,
}