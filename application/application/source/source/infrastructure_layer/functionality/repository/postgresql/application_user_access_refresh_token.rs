use super::by::By3;
use super::by::By4;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserAccessRefreshToken<'_>> {
    pub async fn create_1<'a>(
        database_2_connection: &'a Connection,
        insert_1: Insert1<'a>,
    ) -> Result<ApplicationUserAccessRefreshToken<'a>, Auditor<Error>> {
        let application_user_access_refresh_token_obfuscation_value = insert_1.application_user_access_refresh_token_obfuscation_value.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_access_refresh_token AS auart ( \
                application_user_id, \
                application_user_device_id, \
                application_user_access_token_id, \
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
                &insert_1.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &insert_1.application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &insert_1.application_user_access_token_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_access_refresh_token_obfuscation_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_1.application_user_access_refresh_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &insert_1.application_user_access_refresh_token_updated_at,
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
            ApplicationUserAccessRefreshToken::new(
                insert_1.application_user_id,
                Cow::Borrowed(insert_1.application_user_device_id),
                Cow::Borrowed(insert_1.application_user_access_token_id),
                insert_1.application_user_access_refresh_token_obfuscation_value,
                insert_1.application_user_access_refresh_token_expires_at,
                insert_1.application_user_access_refresh_token_updated_at,
            ),
        );
    }

    pub async fn update_1<'a>(
        database_2_connection: &'a Connection,
        update_2: &'a Update2<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_access_refresh_token AS auart \
            SET ( \
                application_user_access_token_id, \
                obfuscation_value, \
                expires_at, \
                updated_at
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4 \
            ) \
            WHERE auart.application_user_id = $5 AND auart.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_2.application_user_access_token_id,
                Type::TEXT,
            )
            .add_parameter(
                &update_2.application_user_access_refresh_token_obfuscation_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_2.application_user_access_refresh_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &update_2.application_user_access_refresh_token_updated_at,
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

    pub async fn delete_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

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

    pub async fn delete_2<'a>(
        database_2_connection: &'a Connection,
        by_3: &'a By3,
    ) -> Result<(), Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_3.application_user_id,
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

        return Ok(());
    }

    pub async fn find_1<'a, 'b>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'b>,
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'b>>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auart.application_user_access_token_id AS auati, \
                auart.obfuscation_value AS ov, \
                auart.expires_at AS ea, \
                auart.updated_at AS ua \
            FROM public.application_user_access_refresh_token auart \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

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
                ApplicationUserAccessRefreshToken::new(
                    by_4.application_user_id,
                    Cow::Borrowed(by_4.application_user_device_id),
                    Cow::Owned(row_registry[0].try_get::<'_, usize, String>(0).convert(Backtrace::new(line!(), file!()))?),
                    row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                    row_registry[0].try_get::<'_, usize, i64>(2).convert(Backtrace::new(line!(), file!()))?,
                    row_registry[0].try_get::<'_, usize, i64>(3).convert(Backtrace::new(line!(), file!()))?,
                ),
            ),
        );
    }
}

pub struct Insert1<'a> {
    pub application_user_id: i64,
    pub application_user_device_id: &'a str,
    pub application_user_access_token_id: &'a str,
    pub application_user_access_refresh_token_obfuscation_value: String,
    pub application_user_access_refresh_token_expires_at: i64,
    pub application_user_access_refresh_token_updated_at: i64,
}

pub struct Update2<'a> {
    pub application_user_access_token_id: &'a str,
    pub application_user_access_refresh_token_obfuscation_value: &'a str,
    pub application_user_access_refresh_token_expires_at: i64,
    pub application_user_access_refresh_token_updated_at: i64,
}