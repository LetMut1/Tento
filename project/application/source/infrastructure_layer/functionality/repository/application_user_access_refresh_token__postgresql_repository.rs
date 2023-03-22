use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use std::borrow::Cow;

pub struct ApplicationUserAccessRefreshToken_PostgresqlRepository;

impl ApplicationUserAccessRefreshToken_PostgresqlRepository {
    pub async fn create<'a, 'b>(database_2_connection: &'a Connection, insert: Insert<'b>) -> Result<ApplicationUserAccessRefreshToken<'b>, ErrorAuditor> {
        let application_user_device_id_ = insert.application_user_device_id.as_ref();

        let application_user_access_token_id_ = insert.application_user_access_token_id.as_ref();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_access_refresh_token AS auart ( \
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
            .add_parameter(&insert.application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT)
            .add_parameter(&application_user_access_token_id_, Type::TEXT)
            .add_parameter(&insert.application_user_access_refresh_token_obfuscation_value, Type::TEXT)
            .add_parameter(&insert.application_user_access_refresh_token_expires_at, Type::INT8)
            .add_parameter(&insert.application_user_access_refresh_token_updated_at, Type::INT8);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(
            ApplicationUserAccessRefreshToken::new(
                insert.application_user_id,
                insert.application_user_device_id,
                insert.application_user_access_token_id,
                insert.application_user_access_refresh_token_obfuscation_value,
                insert.application_user_access_refresh_token_expires_at,
                insert.application_user_access_refresh_token_updated_at
            )
        );
    }

    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user_access_refresh_token.get_application_user_id();

        let application_user_device_id = application_user_access_refresh_token.get_application_user_device_id();

        let application_user_access_token_id = application_user_access_refresh_token.get_application_user_access_token_id();

        let application_user_access_refresh_token_obfuscation_value = application_user_access_refresh_token.get_obfuscation_value();

        let application_user_access_refresh_token_expires_at = application_user_access_refresh_token.get_expires_at();

        let application_user_access_refresh_token_updated_at = application_user_access_refresh_token.get_updated_at();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_access_refresh_token AS auart \
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
            .add_parameter(&application_user_access_token_id, Type::TEXT)
            .add_parameter(&application_user_access_refresh_token_obfuscation_value, Type::TEXT)
            .add_parameter(&application_user_access_refresh_token_expires_at, Type::INT8)
            .add_parameter(&application_user_access_refresh_token_updated_at, Type::INT8)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }

    pub async fn delete_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }

    pub async fn delete_2<'a>(database_2_connection: &'a Connection, application_user_id: i64) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                auart.application_user_access_token_id AS auati, \
                auart.obfuscation_value AS ov, \
                auart.expires_at AS ea, \
                auart.updated_at AS ua \
            FROM public.application_user_access_refresh_token auart \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_access_token_id = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_access_token_id_) => application_user_access_token_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_access_refresh_token_obfuscation_value = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_access_refresh_token_obfuscation_value_) => application_user_access_refresh_token_obfuscation_value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_access_refresh_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_access_refresh_token_updated_at_) => application_user_access_refresh_token_updated_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAccessRefreshToken::new(
                    application_user_id,
                    Cow::Borrowed(application_user_device_id),
                    Cow::Owned(application_user_access_token_id),
                    application_user_access_refresh_token_obfuscation_value,
                    application_user_access_refresh_token_expires_at,
                    application_user_access_refresh_token_updated_at
                )
            )
        );
    }
}

pub struct Insert<'a> {
    pub application_user_id: i64,
    pub application_user_device_id: Cow<'a, str>,
    pub application_user_access_token_id: Cow<'a, str>,
    pub application_user_access_refresh_token_obfuscation_value: String,
    pub application_user_access_refresh_token_expires_at: i64,
    pub application_user_access_refresh_token_updated_at: i64
}