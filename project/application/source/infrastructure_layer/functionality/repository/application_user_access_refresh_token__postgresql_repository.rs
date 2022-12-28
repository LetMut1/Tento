use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use std::borrow::Cow;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessRefreshToken_PostgresqlRepository;

impl ApplicationUserAccessRefreshToken_PostgresqlRepository {
    pub async fn create<'a, 'b>(authorization_connection: &'a Connection, insert: Insert<'b>) -> Result<ApplicationUserAccessRefreshToken<'b>, ErrorAuditor> {
        let application_user_log_in_token_device_id_ = insert.application_user_log_in_token_device_id.as_ref();

        let application_user_access_token_id_ = insert.application_user_access_token_id.as_ref();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_access_refresh_token AS auart ( \
                application_user_id, \
                application_user_log_in_token_device_id, \
                application_user_access_token_id, \
                obfuscation_value, \
                expires_at, \
                updated_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                extract(EPOCH FROM (current_timestamp(0) + (INTERVAL '1 MINUTE' * $5)::INTERVAL)), \
                current_timestamp(6) \
            ) \
            RETURNING \
                auart.expires_at AS ea, \
                auart.updated_at::TEXT as ua;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_id, Type::INT8)
            .add_parameter(&application_user_log_in_token_device_id_, Type::TEXT)
            .add_parameter(&application_user_access_token_id_, Type::TEXT)
            .add_parameter(&insert.application_user_access_refresh_token_obfuscation_value, Type::TEXT)
            .add_parameter(&ApplicationUserAccessRefreshToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION, Type::INT4);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let application_user_access_refresh_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(0) {
                                Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(application_user_access_refresh_token_updated_at_) => application_user_access_refresh_token_updated_at_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            return Ok(
                                ApplicationUserAccessRefreshToken::new(
                                    insert.application_user_id,
                                    insert.application_user_log_in_token_device_id,
                                    insert.application_user_access_token_id,
                                    insert.application_user_access_refresh_token_obfuscation_value,
                                    application_user_access_refresh_token_expires_at,
                                    application_user_access_refresh_token_updated_at
                                )
                            );
                        }

                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserAccessRefreshToken can not be inserted into Postgresql database.") },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn update<'a, 'b>(
        authorization_connection: &'a Connection,
        application_user_access_refresh_token: &'a mut ApplicationUserAccessRefreshToken<'b>,
        update: Update
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user_access_refresh_token.get_application_user_id();

        let application_user_log_in_token_device_id = application_user_access_refresh_token.get_application_user_log_in_token_device_id();

        let application_user_access_token_id = application_user_access_refresh_token.get_application_user_access_token_id();

        let application_user_access_refresh_token_obfuscation_value = application_user_access_refresh_token.get_obfuscation_value();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_access_token_id, Type::TEXT)
            .add_parameter(&application_user_access_refresh_token_obfuscation_value, Type::TEXT);

        if update.application_user_access_refresh_token_expires_at {
            if update.application_user_access_refresh_token_updated_at {
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
                        extract(EPOCH FROM (current_timestamp(0) + (INTERVAL '1 MINUTE' * $3)::INTERVAL)), \
                        current_timestamp(6) \
                    ) \
                    WHERE auart.application_user_id = $4 AND auart.application_user_log_in_token_device_id = $5 \
                    RETURNING \
                        auart.expires_at AS ea, \
                        auart.updated_at::TEXT as ua;";

                prepared_statemant_parameter_convertation_resolver
                    .add_parameter(&ApplicationUserAccessRefreshToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION, Type::INT4)
                    .add_parameter(&application_user_id, Type::INT8)
                    .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

                match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
                    Ok(ref statement) => {
                        match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                            Ok(row_registry) => {
                                if !row_registry.is_empty() {
                                    let application_user_access_refresh_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(0) {
                                        Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
                                        Err(error) => {
                                            return Err(
                                                ErrorAuditor::new(
                                                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                    };

                                    let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                        Ok(application_user_access_refresh_token_updated_at_) => application_user_access_refresh_token_updated_at_,
                                        Err(error) => {
                                            return Err(
                                                ErrorAuditor::new(
                                                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                    };

                                    application_user_access_refresh_token
                                        .set_expires_at(application_user_access_refresh_token_expires_at)
                                        .set_updated_at(application_user_access_refresh_token_updated_at);
                                } else {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserAccessRefreshToken can not be updated into Postgresql database.") },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            } else {
                let query =
                    "UPDATE ONLY public.application_user_access_refresh_token AS auart \
                    SET ( \
                        application_user_access_token_id, \
                        obfuscation_value, \
                        expires_at \
                    ) = ROW( \
                        $1, \
                        $2, \
                        extract(EPOCH FROM (current_timestamp(0) + (INTERVAL '1 MINUTE' * $3)::INTERVAL)) \
                    ) \
                    WHERE auart.application_user_id = $4 AND auart.application_user_log_in_token_device_id = $5 \
                    RETURNING \
                        auart.expires_at AS ea;";

                prepared_statemant_parameter_convertation_resolver
                    .add_parameter(&ApplicationUserAccessRefreshToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION, Type::INT4)
                    .add_parameter(&application_user_id, Type::INT8)
                    .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

                match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
                    Ok(ref statement) => {
                        match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                            Ok(row_registry) => {
                                if !row_registry.is_empty() {
                                    let application_user_access_refresh_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(0) {
                                        Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
                                        Err(error) => {
                                            return Err(
                                                ErrorAuditor::new(
                                                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                    };

                                    application_user_access_refresh_token
                                        .set_expires_at(application_user_access_refresh_token_expires_at);
                                } else {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserAccessRefreshToken can not be updated into Postgresql database.") },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
        } else {
            if update.application_user_access_refresh_token_updated_at {
                let query =
                    "UPDATE ONLY public.application_user_access_refresh_token AS auart \
                    SET ( \
                        application_user_access_token_id, \
                        obfuscation_value, \
                        updated_at \
                    ) = ROW( \
                        $1, \
                        $2, \
                        current_timestamp(6) \
                    ) \
                    WHERE auart.application_user_id = $3 AND auart.application_user_log_in_token_device_id = $4 \
                    RETURNING \
                        auart.updated_at::TEXT as ua;";

                prepared_statemant_parameter_convertation_resolver
                    .add_parameter(&application_user_id, Type::INT8)
                    .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

                match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
                    Ok(ref statement) => {
                        match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                            Ok(row_registry) => {
                                if !row_registry.is_empty() {
                                    let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                        Ok(application_user_access_refresh_token_updated_at_) => application_user_access_refresh_token_updated_at_,
                                        Err(error) => {
                                            return Err(
                                                ErrorAuditor::new(
                                                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                    };

                                    application_user_access_refresh_token
                                        .set_updated_at(application_user_access_refresh_token_updated_at);
                                } else {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserAccessRefreshToken can not be updated into Postgresql database.") },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            } else {
                let query =
                    "UPDATE ONLY public.application_user_access_refresh_token AS auart \
                    SET ( \
                        application_user_access_token_id, \
                        obfuscation_value \
                    ) = ROW( \
                        $1, \
                        $2 \
                    ) \
                    WHERE auart.application_user_id = $3 AND auart.application_user_log_in_token_device_id = $4 \
                    RETURNING \
                        auart.application_user_id as aui;";

                prepared_statemant_parameter_convertation_resolver
                    .add_parameter(&application_user_id, Type::INT8)
                    .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

                match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
                    Ok(ref statement) => {
                        match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                            Ok(row_registry) => {
                                if row_registry.is_empty() {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserAccessRefreshToken can not be updated in Postgresql database.") },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
        };

        return Ok(());
    }

    pub async fn delete_1<'a>(
        authorization_connection: &'a Connection,
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1 AND auart.application_user_log_in_token_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
        .add_parameter(&application_user_id, Type::INT8)
        .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn delete_2<'a>(authorization_connection: &'a Connection, application_user_id: i64) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_access_refresh_token AS auart  \
            WHERE auart.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn find_1<'a>(
        authorization_connection: &'a Connection,
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str
    ) -> Result<Option<ApplicationUserAccessRefreshToken<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                auart.application_user_access_token_id AS auati, \
                auart.obfuscation_value AS ov, \
                auart.expires_at AS ea, \
                auart.updated_at AS ua::TEXT \
            FROM public.application_user_access_refresh_token auart \
            WHERE auart.application_user_id = $1 AND auart.application_user_log_in_token_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);
        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let application_user_access_token_id = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(application_user_access_token_id_) => application_user_access_token_id_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            let application_user_access_refresh_token_updated_at = match row_registry[0].try_get::<'_, usize, String>(2) {
                                Ok(application_user_access_refresh_token_updated_at_) => application_user_access_refresh_token_updated_at_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            return Ok(
                                Some(
                                    ApplicationUserAccessRefreshToken::new(
                                        application_user_id,
                                        Cow::Borrowed(application_user_log_in_token_device_id),
                                        Cow::Owned(application_user_access_token_id),
                                        application_user_access_refresh_token_obfuscation_value,
                                        application_user_access_refresh_token_expires_at,
                                        application_user_access_refresh_token_updated_at
                                    )
                                )
                            );
                        }

                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}

pub struct Insert<'a> {
    pub application_user_id: i64,
    pub application_user_log_in_token_device_id: Cow<'a, str>,
    pub application_user_access_token_id: Cow<'a, str>,
    pub application_user_access_refresh_token_obfuscation_value: String,
}

pub struct Update {
    pub application_user_access_refresh_token_expires_at: bool,
    pub application_user_access_refresh_token_updated_at: bool
}