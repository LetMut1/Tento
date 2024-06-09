use super::by::By5;
use super::update::Update10;
use super::update::Update11;
use super::update::Update7;
use super::update::Update8;
use super::update::Update9;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::derivative::ApplicationUserRegistrationToken1;
use crate::domain_layer::data::entity::application_user_registration_token::derivative::ApplicationUserRegistrationToken2;
use crate::domain_layer::data::entity::application_user_registration_token::derivative::ApplicationUserRegistrationToken3;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserRegistrationToken<'_>> {
    pub async fn create_1<'a>(
        database_2_connection: &'a Connection,
        insert_5: Insert5<'a>,
    ) -> Result<ApplicationUserRegistrationToken<'a>, Auditor<Error>> {
        let application_user_registration_token_value = insert_5.application_user_registration_token_value.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_registration_token AS aurt ( \
                application_user_email, \
                application_user_device_id, \
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

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &insert_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &insert_5.application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_can_be_resent_from,
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
            ApplicationUserRegistrationToken::new(
                Cow::Borrowed(insert_5.application_user_email),
                Cow::Borrowed(insert_5.application_user_device_id),
                insert_5.application_user_registration_token_value,
                insert_5.application_user_registration_token_wrong_enter_tries_quantity,
                insert_5.application_user_registration_token_is_approved,
                insert_5.application_user_registration_token_expires_at,
                insert_5.application_user_registration_token_can_be_resent_from,
            ),
        );
    }

    pub async fn delete_2<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_registration_token AS aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
        update_7: &'a Update7<'_>,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_registration_token AS aurt
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
            WHERE aurt.application_user_email = $6 AND aurt.application_user_device_id = $7;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_7.application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_7.application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_7.application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &update_7.application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &update_7.application_user_registration_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
        update_8: &'a Update8,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_8.application_user_registration_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
        update_9: &'a Update9<'_>,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_registration_token AS aurt
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
            WHERE aurt.application_user_email = $5 AND aurt.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_9.application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_9.application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_9.application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &update_9.application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
        update_10: &'a Update10,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_10.application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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

    pub async fn update_5<'a>(
        database_2_connection: &'a Connection,
        update_11: &'a Update11,
        by_5: &'a By5<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                is_approved \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_11.application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurt.value AS v, \
                aurt.wrong_enter_tries_quantity AS wetq, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea, \
                aurt.can_be_resent_from as cbrf \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
                ApplicationUserRegistrationToken1 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(2).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?,
                    can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(4).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_2<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken2>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurt.value AS v, \
                aurt.wrong_enter_tries_quantity AS wetq, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
                ApplicationUserRegistrationToken2 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(2).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }

    pub async fn find_3<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken3>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurt.value AS v, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea, \
                aurt.can_be_resent_from as cbrf \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_5.application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &by_5.application_user_device_id,
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
                ApplicationUserRegistrationToken3 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(1).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?,
                    can_be_resent_from: row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?,
                },
            ),
        );
    }
}

pub struct Insert5<'a> {
    pub application_user_email: &'a str,
    pub application_user_device_id: &'a str,
    pub application_user_registration_token_value: String,
    pub application_user_registration_token_wrong_enter_tries_quantity: i16,
    pub application_user_registration_token_is_approved: bool,
    pub application_user_registration_token_expires_at: i64,
    pub application_user_registration_token_can_be_resent_from: i64,
}