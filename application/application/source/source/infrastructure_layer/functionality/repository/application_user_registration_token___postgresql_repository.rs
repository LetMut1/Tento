use super::postgresql_repository::by::By5;
use super::postgresql_repository::insert::Insert5;
use super::postgresql_repository::update::Update10;
use super::postgresql_repository::update::Update11;
use super::postgresql_repository::update::Update7;
use super::postgresql_repository::update::Update8;
use super::postgresql_repository::update::Update9;
use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken1;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken2;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken4;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken5;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken6;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserRegistrationToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_5: Insert5<'a>,
    ) -> Result<ApplicationUserRegistrationToken<'a>, ErrorAuditor> {
        let application_user_email = insert_5.application_user_email.0.as_str();

        let application_user_device_id = insert_5.application_user_device_id.0.as_str();

        let application_user_registration_token_value = insert_5.application_user_registration_token_value.0.as_str();

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
                &application_user_email,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_is_approved.0,
                Type::BOOL,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_5.application_user_registration_token_can_be_resent_from.0,
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
            ApplicationUserRegistrationToken {
                application_user_email: Cow::Borrowed(insert_5.application_user_email),
                application_user_device_id: Cow::Borrowed(insert_5.application_user_device_id),
                value: insert_5.application_user_registration_token_value,
                wrong_enter_tries_quantity: insert_5.application_user_registration_token_wrong_enter_tries_quantity,
                is_approved: insert_5.application_user_registration_token_is_approved,
                expires_at: insert_5.application_user_registration_token_expires_at,
                can_be_resent_from: insert_5.application_user_registration_token_can_be_resent_from,
            },
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_registration_token AS aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken1> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_7: &'a Update7<'_>,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

        let application_user_registration_token_value = update_7.application_user_registration_token_value.0.as_str();

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
                &application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_7.application_user_registration_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &update_7.application_user_registration_token_is_approved.0,
                Type::BOOL,
            )
            .add_parameter(
                &update_7.application_user_registration_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &update_7.application_user_registration_token_can_be_resent_from.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken1>, ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &application_user_email,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value(application_user_registration_token_value_),
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

        let application_user_registration_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_registration_token_wrong_enter_tries_quantity_) => ApplicationUserRegistrationToken_WrongEnterTriesQuantity(application_user_registration_token_wrong_enter_tries_quantity_),
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

        let application_user_registration_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved(application_user_registration_token_is_approved_),
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

        let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt(application_user_registration_token_expires_at_),
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

        let application_user_registration_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(4) {
            Ok(application_user_registration_token_can_be_resent_from_) => ApplicationUserRegistrationToken_CanBeResentFrom(application_user_registration_token_can_be_resent_from_),
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
                ApplicationUserRegistrationToken1 {
                    value: application_user_registration_token_value,
                    wrong_enter_tries_quantity: application_user_registration_token_wrong_enter_tries_quantity,
                    is_approved: application_user_registration_token_is_approved,
                    expires_at: application_user_registration_token_expires_at,
                    can_be_resent_from: application_user_registration_token_can_be_resent_from,
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserRegistrationToken2> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_8: &'a Update8,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &update_8.application_user_registration_token_can_be_resent_from.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken3> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_9: &'a Update9<'_>,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

        let application_user_registration_token_value = update_9.application_user_registration_token_value.0.as_str();

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
                &application_user_registration_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_9.application_user_registration_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &update_9.application_user_registration_token_is_approved.0,
                Type::BOOL,
            )
            .add_parameter(
                &update_9.application_user_registration_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken3>, ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &application_user_email,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value(application_user_registration_token_value_),
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

        let application_user_registration_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_registration_token_wrong_enter_tries_quantity_) => ApplicationUserRegistrationToken_WrongEnterTriesQuantity(application_user_registration_token_wrong_enter_tries_quantity_),
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

        let application_user_registration_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved(application_user_registration_token_is_approved_),
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

        let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt(application_user_registration_token_expires_at_),
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
                ApplicationUserRegistrationToken3 {
                    value: application_user_registration_token_value,
                    wrong_enter_tries_quantity: application_user_registration_token_wrong_enter_tries_quantity,
                    is_approved: application_user_registration_token_is_approved,
                    expires_at: application_user_registration_token_expires_at,
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserRegistrationToken4> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_10: &'a Update10,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &update_10.application_user_registration_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken5> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_11: &'a Update11,
        by_5: &'a By5<'_>,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &update_11.application_user_registration_token_is_approved.0,
                Type::BOOL,
            )
            .add_parameter(
                &application_user_email,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken6> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_5: &'a By5<'_>,
    ) -> Result<Option<ApplicationUserRegistrationToken6>, ErrorAuditor> {
        let application_user_email = by_5.application_user_email.0.as_str();

        let application_user_device_id = by_5.application_user_device_id.0.as_str();

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
                &application_user_email,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value(application_user_registration_token_value_),
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

        let application_user_registration_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(1) {
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved(application_user_registration_token_is_approved_),
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

        let application_user_registration_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt(application_user_registration_token_expires_at_),
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

        let application_user_registration_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_registration_token_can_be_resent_from_) => ApplicationUserRegistrationToken_CanBeResentFrom(application_user_registration_token_can_be_resent_from_),
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
                ApplicationUserRegistrationToken6 {
                    value: application_user_registration_token_value,
                    is_approved: application_user_registration_token_is_approved,
                    expires_at: application_user_registration_token_expires_at,
                    can_be_resent_from: application_user_registration_token_can_be_resent_from,
                },
            ),
        );
    }
}
