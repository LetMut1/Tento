use super::by::By4;
use super::insert::Insert6;
use super::update::Update12;
use super::update::Update13;
use super::update::Update14;
use super::update::Update15;
use super::update::Update16;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken1;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken4;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken5;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken6;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserResetPasswordToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_6: Insert6<'a>,
    ) -> Result<ApplicationUserResetPasswordToken<'a>, Auditor<Error>> {
        let application_user_reset_password_token_value = insert_6.application_user_reset_password_token_value.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_reset_password_token AS aurpt ( \
                application_user_id, \
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
                &insert_6.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &insert_6.application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_reset_password_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_6.application_user_reset_password_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &insert_6.application_user_reset_password_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &insert_6.application_user_reset_password_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_6.application_user_reset_password_token_can_be_resent_from.0,
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
            ApplicationUserResetPasswordToken::new(
                insert_6.application_user_id,
                Cow::Borrowed(insert_6.application_user_device_id),
                insert_6.application_user_reset_password_token_value,
                insert_6.application_user_reset_password_token_wrong_enter_tries_quantity,
                insert_6.application_user_reset_password_token_is_approved,
                insert_6.application_user_reset_password_token_expires_at,
                insert_6.application_user_reset_password_token_can_be_resent_from,
            ),
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_reset_password_token AS aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

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
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken1> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_12: &'a Update12<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at, \
                can_be_resent_from \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5 \
            ) \
            WHERE aurpt.application_user_id = $6 AND aurpt.application_user_device_id = $7;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_12.application_user_reset_password_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_12.application_user_reset_password_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_12.application_user_reset_password_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &update_12.application_user_reset_password_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &update_12.application_user_reset_password_token_can_be_resent_from.0,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserResetPasswordToken1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurpt.value AS v, \
                aurpt.wrong_enter_tries_quantity AS wetq, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea,
                aurpt.can_be_resent_from AS cbrf \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

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
                ApplicationUserResetPasswordToken1 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(2).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: ApplicationUserResetPasswordToken_ExpiresAt(row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?),
                    can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom(row_registry[0].try_get::<'_, usize, i64>(4).convert(Backtrace::new(line!(), file!()))?),
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken2> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_13: &'a Update13,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_13.application_user_reset_password_token_can_be_resent_from.0,
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
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken3> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_14: &'a Update14<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_reset_password_token AS aurpt
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
            WHERE aurpt.application_user_id = $5 AND aurpt.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_14.application_user_reset_password_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_14.application_user_reset_password_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &update_14.application_user_reset_password_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &update_14.application_user_reset_password_token_expires_at.0,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserResetPasswordToken3>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurpt.value AS v, \
                aurpt.wrong_enter_tries_quantity AS wetq, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

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
                ApplicationUserResetPasswordToken3 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    wrong_enter_tries_quantity: row_registry[0].try_get::<'_, usize, i16>(1).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(2).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: ApplicationUserResetPasswordToken_ExpiresAt(row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?),
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken4> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_15: &'a Update15,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_15.application_user_reset_password_token_wrong_enter_tries_quantity,
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
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken5> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_16: &'a Update16,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                is_approved \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_16.application_user_reset_password_token_is_approved,
                Type::BOOL,
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
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken6> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserResetPasswordToken6>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                aurpt.value AS v, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea,
                aurpt.can_be_resent_from AS cbrf \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

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
                ApplicationUserResetPasswordToken6 {
                    value: row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?,
                    is_approved: row_registry[0].try_get::<'_, usize, bool>(1).convert(Backtrace::new(line!(), file!()))?,
                    expires_at: ApplicationUserResetPasswordToken_ExpiresAt( row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?),
                    can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom(row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?),
                },
            ),
        );
    }
}
