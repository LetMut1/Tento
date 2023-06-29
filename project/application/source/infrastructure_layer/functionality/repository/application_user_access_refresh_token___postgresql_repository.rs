use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::getter::Getter;
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
    pub async fn create<'a, 'b>(
        database_2_connection: &'a Connection,
        insert: Insert<'b>,
    ) -> Result<ApplicationUserAccessRefreshToken<'b>, ErrorAuditor> {
        let application_user_id = insert.application_user_id.get();

        let application_user_device_id = insert.application_user_device_id.as_ref().get();

        let application_user_access_token_id = insert.application_user_access_token_id.as_ref().get();

        let application_user_access_refresh_token_obfuscation_value = insert.application_user_access_refresh_token_obfuscation_value.get();

        let application_user_access_refresh_token_expires_at = insert.application_user_access_refresh_token_expires_at.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let application_user_access_refresh_token_updated_at = insert.application_user_access_refresh_token_updated_at.get();

        let query = "INSERT INTO public.application_user_access_refresh_token AS auart ( \
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
            ApplicationUserAccessRefreshToken::new(
                insert.application_user_id,
                insert.application_user_device_id,
                insert.application_user_access_token_id,
                insert.application_user_access_refresh_token_obfuscation_value,
                insert.application_user_access_refresh_token_expires_at,
                insert.application_user_access_refresh_token_updated_at,
            ),
        );
    }

    pub async fn delete_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_id_,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id_,
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
        application_user_id: ApplicationUser_Id,
    ) -> Result<(), ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_id_,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'a>>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                auart.application_user_access_token_id AS auati, \
                auart.obfuscation_value AS ov, \
                auart.expires_at AS ea, \
                auart.updated_at AS ua \
            FROM public.application_user_access_refresh_token auart \
            WHERE auart.application_user_id = $1 AND auart.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_id_,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id_,
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
            Ok(application_user_access_token_id_) => ApplicationUserAccessToken_Id::new(application_user_access_token_id_),
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
            Ok(application_user_access_refresh_token_obfuscation_value_) => ApplicationUserAccessRefreshToken_ObfuscationValue::new(application_user_access_refresh_token_obfuscation_value_),
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
            Ok(application_user_access_refresh_token_expires_at_) => ApplicationUserAccessRefreshToken_ExpiresAt::new(application_user_access_refresh_token_expires_at_),
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
            Ok(application_user_access_refresh_token_updated_at_) => ApplicationUserAccessRefreshToken_UpdatedAt::new(application_user_access_refresh_token_updated_at_),
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
                ApplicationUserAccessRefreshToken::new(
                    application_user_id,
                    Cow::Borrowed(application_user_device_id),
                    Cow::Owned(application_user_access_token_id),
                    application_user_access_refresh_token_obfuscation_value,
                    application_user_access_refresh_token_expires_at,
                    application_user_access_refresh_token_updated_at,
                ),
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserAccessRefreshToken_1> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUserAccessToken_Id>,
        T: Getter<'a, &'a ApplicationUserAccessRefreshToken_ObfuscationValue>,
        T: Getter<'a, ApplicationUserAccessRefreshToken_ExpiresAt>,
        T: Getter<'a, ApplicationUserAccessRefreshToken_UpdatedAt>,
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_access_token_id = <T as Getter<'a, &'a ApplicationUserAccessToken_Id>>::get(subject).get();

        let application_user_access_refresh_token_obfuscation_value = <T as Getter<'a, &'a ApplicationUserAccessRefreshToken_ObfuscationValue>>::get(subject).get();

        let application_user_access_refresh_token_expires_at = <T as Getter<'a, ApplicationUserAccessRefreshToken_ExpiresAt>>::get(subject).get();

        let application_user_access_refresh_token_updated_at = <T as Getter<'a, ApplicationUserAccessRefreshToken_UpdatedAt>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_access_refresh_token AS auart \
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
                &application_user_id_,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id_,
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

pub struct Insert<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub application_user_access_token_id: Cow<'a, ApplicationUserAccessToken_Id>,
    pub application_user_access_refresh_token_obfuscation_value: ApplicationUserAccessRefreshToken_ObfuscationValue,
    pub application_user_access_refresh_token_expires_at: ApplicationUserAccessRefreshToken_ExpiresAt,
    pub application_user_access_refresh_token_updated_at: ApplicationUserAccessRefreshToken_UpdatedAt,
}
