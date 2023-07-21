use super::postgresql_repository::by::By3;
use super::postgresql_repository::by::By4;
use super::postgresql_repository::update::Update2;
use super::postgresql_repository::PostgresqlRepository;
use super::postgresql_repository::insert::Insert2;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;
use std::borrow::Cow;

impl PostgresqlRepository<ApplicationUserAccessRefreshToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_2: Insert2<'a>,
    ) -> Result<ApplicationUserAccessRefreshToken<'a>, ErrorAuditor> {
        let application_user_id = insert_2.application_user_id.0;

        let application_user_device_id = insert_2.application_user_device_id.0.as_str();

        let application_user_access_token_id = insert_2.application_user_access_token_id.0.as_str();

        let application_user_access_refresh_token_obfuscation_value = insert_2.application_user_access_refresh_token_obfuscation_value.0.as_str();

        let application_user_access_refresh_token_expires_at = insert_2.application_user_access_refresh_token_expires_at.0;

        let application_user_access_refresh_token_updated_at = insert_2.application_user_access_refresh_token_updated_at.0;

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
                &application_user_id,
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
                &application_user_access_refresh_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_access_refresh_token_updated_at,
                Type::INT8,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::PostgresqlError {
                                postgresql_error: error,
                            },
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        };

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
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = by_4.application_user_id.0;

        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::PostgresqlError {
                                postgresql_error: error,
                            },
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        };

        return Ok(());
    }

    pub async fn delete_2<'a>(
        database_2_connection: &'a Connection,
        by_3: &'a By3,
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = by_3.application_user_id.0;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_id,
            Type::INT8,
        );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::PostgresqlError {
                                postgresql_error: error,
                            },
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a, 'b>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'b>,
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'b>>, ErrorAuditor> {
        let application_user_id = by_4.application_user_id.0;

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
                &application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_access_token_id = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_access_token_id_) => ApplicationUserAccessToken_Id(application_user_access_token_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_user_access_refresh_token_obfuscation_value = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_access_refresh_token_obfuscation_value_) => ApplicationUserAccessRefreshToken_ObfuscationValue(application_user_access_refresh_token_obfuscation_value_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_user_access_refresh_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_access_refresh_token_expires_at_) => ApplicationUserAccessRefreshToken_ExpiresAt(application_user_access_refresh_token_expires_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_access_refresh_token_updated_at_) => ApplicationUserAccessRefreshToken_UpdatedAt(application_user_access_refresh_token_updated_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAccessRefreshToken {
                    application_user_id: by_4.application_user_id,
                    application_user_device_id: Cow::Borrowed(by_4.application_user_device_id),
                    application_user_access_token_id: Cow::Owned(application_user_access_token_id),
                    obfuscation_value: application_user_access_refresh_token_obfuscation_value,
                    expires_at: application_user_access_refresh_token_expires_at,
                    updated_at: application_user_access_refresh_token_updated_at,
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
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = by_4.application_user_id.0;

        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let application_user_access_token_id = update_2.application_user_access_token_id.0.as_str();

        let application_user_access_refresh_token_obfuscation_value = update_2.application_user_access_refresh_token_obfuscation_value.0.as_str();

        let application_user_access_refresh_token_expires_at = update_2.application_user_access_refresh_token_expires_at.0;

        let application_user_access_refresh_token_updated_at = update_2.application_user_access_refresh_token_updated_at.0;

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
                &application_user_access_refresh_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_access_refresh_token_updated_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::PostgresqlError {
                                postgresql_error: error,
                            },
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        };

        return Ok(());
    }
}
