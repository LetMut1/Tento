use super::by::By3;
use super::by::By4;
use super::insert::Insert2;
use super::update::Update2;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserAccessRefreshToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_2: Insert2<'a>,
    ) -> Result<ApplicationUserAccessRefreshToken<'a>, Auditor<Error>> {
        let application_user_device_id = insert_2.application_user_device_id.0.as_str();

        let application_user_access_token_id = insert_2.application_user_access_token_id.0.as_str();

        let application_user_access_refresh_token_obfuscation_value = insert_2.application_user_access_refresh_token_obfuscation_value.0.as_str();

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
                &insert_2.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_access_token_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_access_refresh_token_obfuscation_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_2.application_user_access_refresh_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_2.application_user_access_refresh_token_updated_at.0,
                Type::INT8,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        return Ok(
            ApplicationUserAccessRefreshToken {
                application_user_id: insert_2.application_user_id,
                application_user_device_id: Cow::Borrowed(insert_2.application_user_device_id),
                application_user_access_token_id: Cow::Borrowed(insert_2.application_user_access_token_id),
                obfuscation_value: insert_2.application_user_access_refresh_token_obfuscation_value,
                expires_at: insert_2.application_user_access_refresh_token_expires_at,
                updated_at: insert_2.application_user_access_refresh_token_updated_at,
            },
        );
    }

    pub async fn delete_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

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
            &by_3.application_user_id.0,
            Type::INT8,
        );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        return Ok(());
    }

    pub async fn find_1<'a, 'b>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'b>,
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'b>>, Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

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
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(
            Some(
                ApplicationUserAccessRefreshToken {
                    application_user_id: by_4.application_user_id,
                    application_user_device_id: Cow::Borrowed(by_4.application_user_device_id),
                    application_user_access_token_id: Cow::Owned(ApplicationUserAccessToken_Id(row_registry[0].try_get::<'_, usize, String>(0).convert(BacktracePart::new(line!(), file!()))?)),
                    obfuscation_value: ApplicationUserAccessRefreshToken_ObfuscationValue(row_registry[0].try_get::<'_, usize, String>(1).convert(BacktracePart::new(line!(), file!()))?),
                    expires_at: ApplicationUserAccessRefreshToken_ExpiresAt(row_registry[0].try_get::<'_, usize, i64>(2).convert(BacktracePart::new(line!(), file!()))?),
                    updated_at: ApplicationUserAccessRefreshToken_UpdatedAt(row_registry[0].try_get::<'_, usize, i64>(3).convert(BacktracePart::new(line!(), file!()))?),
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserAccessRefreshToken1> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_2: &'a Update2<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let application_user_access_token_id = update_2.application_user_access_token_id.0.as_str();

        let application_user_access_refresh_token_obfuscation_value = update_2.application_user_access_refresh_token_obfuscation_value.0.as_str();

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
                &application_user_access_token_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_access_refresh_token_obfuscation_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_2.application_user_access_refresh_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &update_2.application_user_access_refresh_token_updated_at.0,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(BacktracePart::new(line!(), file!()))?;

        return Ok(());
    }
}
