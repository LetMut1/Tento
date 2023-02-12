use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationToken_PostgresqlRepository;

impl ApplicationUserRegistrationToken_PostgresqlRepository {
    pub async fn create<'a, 'b>(database_2_connection: &'a Connection, insert: Insert<'b>) -> Result<ApplicationUserRegistrationToken<'b>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_registration_token AS aurt ( \
                application_user_email, \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                extract(EPOCH FROM (current_timestamp(0) + (INTERVAL '1 MINUTE' * $5)::INTERVAL)) \
            ) \
            RETURNING \
                aurt.expires_at AS ea;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_email, Type::TEXT)
            .add_parameter(&insert.application_user_registration_token_value, Type::TEXT)
            .add_parameter(&insert.application_user_registration_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&insert.application_user_registration_token_is_approved, Type::BOOL)
            .add_parameter(&ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION, Type::INT2);

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
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationToken can not be inserted into Postgresql database.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
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
            ApplicationUserRegistrationToken::new(
                insert.application_user_email,
                insert.application_user_registration_token_value,
                insert.application_user_registration_token_wrong_enter_tries_quantity,
                insert.application_user_registration_token_is_approved,
                application_user_registration_token_expires_at
            )
        );
    }

    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        application_user_registration_token: &'a mut ApplicationUserRegistrationToken<'_>,
        update: Update
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_token.get_application_user_email();

        let application_user_registration_token_value = application_user_registration_token.get_value();

        let application_user_registration_token_wrong_enter_tries_quantity = application_user_registration_token.get_wrong_enter_tries_quantity();

        let application_user_registration_token_is_approved = application_user_registration_token.get_is_approved();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        if update.application_user_registration_token_expires_at {
            let query =
            "UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                extract(EPOCH FROM (current_timestamp(0) + (INTERVAL '1 MINUTE' * $4)::INTERVAL)) \
            ) \
            WHERE aurt.application_user_email = $5 \
            RETURNING \
                aurt.expires_at AS ae;";

            prepared_statemant_parameter_convertation_resolver
                .add_parameter(&application_user_registration_token_value, Type::TEXT)
                .add_parameter(&application_user_registration_token_wrong_enter_tries_quantity, Type::INT2)
                .add_parameter(&application_user_registration_token_is_approved, Type::BOOL)
                .add_parameter(&ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION, Type::INT2)
                .add_parameter(&application_user_email, Type::TEXT);

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
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationToken can not be updated in Postgresql database.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(0) {
                Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            application_user_registration_token.set_expires_at(application_user_registration_token_expires_at);
        } else {
            let query =
            "UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                is_approved \
            ) = ROW( \
                $1, \
                $2, \
                $3 \
            ) \
            WHERE aurt.application_user_email = $4 \
            RETURNING \
                aurt.application_user_email AS aue;";

            prepared_statemant_parameter_convertation_resolver
                .add_parameter(&application_user_registration_token_value, Type::TEXT)
                .add_parameter(&application_user_registration_token_wrong_enter_tries_quantity, Type::INT2)
                .add_parameter(&application_user_registration_token_is_approved, Type::BOOL)
                .add_parameter(&application_user_email, Type::TEXT);

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
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationToken can not be updated in Postgresql database.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }

        return Ok(());
    }

    pub async fn delete<'a>(database_2_connection: &'a Connection, application_user_email: &'a str) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_registration_token AS aurt \
            WHERE aurt.application_user_email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

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
        application_user_email: &'a str
    ) -> Result<Option<ApplicationUserRegistrationToken<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurt.value AS v, \
                aurt.wrong_enter_tries_quantity AS wetq, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => application_user_registration_token_value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_registration_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_registration_token_wrong_enter_tries_quantity_) => application_user_registration_token_wrong_enter_tries_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_registration_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
            Ok(application_user_registration_token_is_approved_) => application_user_registration_token_is_approved_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
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
                ApplicationUserRegistrationToken::new(
                    application_user_email,
                    application_user_registration_token_value,
                    application_user_registration_token_wrong_enter_tries_quantity,
                    application_user_registration_token_is_approved,
                    application_user_registration_token_expires_at
                )
            )
        );
    }
}

pub struct Insert<'a> {
    pub application_user_email: &'a str,
    pub application_user_registration_token_value: String,
    pub application_user_registration_token_wrong_enter_tries_quantity: i16,
    pub application_user_registration_token_is_approved: bool
}

pub struct Update {
    pub application_user_registration_token_expires_at: bool
}