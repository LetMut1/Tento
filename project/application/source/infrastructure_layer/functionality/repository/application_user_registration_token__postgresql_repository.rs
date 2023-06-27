use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_1;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_2;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_4;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_5;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_6;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
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

impl PostgresqlRepository<ApplicationUserRegistrationToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert: Insert<'a>,
    ) -> Result<ApplicationUserRegistrationToken<'a>, ErrorAuditor> {
        let application_user_email = insert.application_user_email.as_ref().get();

        let application_user_device_id = insert.application_user_device_id.as_ref().get();

        let application_user_registration_token_value = insert.application_user_registration_token_value.get();

        let application_user_registration_token_wrong_enter_tries_quantity = insert
            .application_user_registration_token_wrong_enter_tries_quantity
            .get();

        let application_user_registration_token_is_approved = insert.application_user_registration_token_is_approved.get();

        let application_user_registration_token_expires_at = insert.application_user_registration_token_expires_at.get();

        let application_user_registration_token_can_be_resent_from = insert
            .application_user_registration_token_can_be_resent_from
            .get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "INSERT INTO public.application_user_registration_token AS aurt ( \
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
                &application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_registration_token_can_be_resent_from,
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
            ApplicationUserRegistrationToken::new(
                insert.application_user_email,
                insert.application_user_device_id,
                insert.application_user_registration_token_value,
                insert.application_user_registration_token_wrong_enter_tries_quantity,
                insert.application_user_registration_token_is_approved,
                insert.application_user_registration_token_expires_at,
                insert.application_user_registration_token_can_be_resent_from,
            ),
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "DELETE FROM ONLY public.application_user_registration_token AS aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken_1> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUserRegistrationToken_Value>,
        T: Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>,
        T: Getter<'a, ApplicationUserRegistrationToken_IsApproved>,
        T: Getter<'a, ApplicationUserRegistrationToken_ExpiresAt>,
        T: Getter<'a, ApplicationUserRegistrationToken_CanBeResentFrom>,
    {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_registration_token_value = <T as Getter<'a, &'a ApplicationUserRegistrationToken_Value>>::get(subject).get();

        let application_user_registration_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>>::get(subject).get();

        let application_user_registration_token_is_approved = <T as Getter<'a, ApplicationUserRegistrationToken_IsApproved>>::get(subject).get();

        let application_user_registration_token_expires_at = <T as Getter<'a, ApplicationUserRegistrationToken_ExpiresAt>>::get(subject).get();

        let application_user_registration_token_can_be_resent_from = <T as Getter<'a, ApplicationUserRegistrationToken_CanBeResentFrom>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_registration_token AS aurt
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
                &application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_registration_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<Option<ApplicationUserRegistrationToken_1>, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                aurt.value AS v, \
                aurt.wrong_enter_tries_quantity AS wetq, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea, \
                aurt.can_be_resent_from as cbrf \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value::new(application_user_registration_token_value_),
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
            Ok(application_user_registration_token_wrong_enter_tries_quantity_) => ApplicationUserRegistrationToken_WrongEnterTriesQuantity::new(application_user_registration_token_wrong_enter_tries_quantity_),
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
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved::new(application_user_registration_token_is_approved_),
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
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt::new(application_user_registration_token_expires_at_),
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
            Ok(application_user_registration_token_can_be_resent_from_) => ApplicationUserRegistrationToken_CanBeResentFrom::new(application_user_registration_token_can_be_resent_from_),
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
                ApplicationUserRegistrationToken_1::new(
                    application_user_registration_token_value,
                    application_user_registration_token_wrong_enter_tries_quantity,
                    application_user_registration_token_is_approved,
                    application_user_registration_token_expires_at,
                    application_user_registration_token_can_be_resent_from,
                ),
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserRegistrationToken_2> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, ApplicationUserRegistrationToken_CanBeResentFrom>,
    {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_registration_token_can_be_resent_from = <T as Getter<'a, ApplicationUserRegistrationToken_CanBeResentFrom>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_registration_token_can_be_resent_from,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken_3> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUserRegistrationToken_Value>,
        T: Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>,
        T: Getter<'a, ApplicationUserRegistrationToken_IsApproved>,
        T: Getter<'a, ApplicationUserRegistrationToken_ExpiresAt>,
    {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_registration_token_value = <T as Getter<'a, &'a ApplicationUserRegistrationToken_Value>>::get(subject).get();

        let application_user_registration_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>>::get(subject).get();

        let application_user_registration_token_is_approved = <T as Getter<'a, ApplicationUserRegistrationToken_IsApproved>>::get(subject).get();

        let application_user_registration_token_expires_at = <T as Getter<'a, ApplicationUserRegistrationToken_ExpiresAt>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_registration_token AS aurt
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
                &application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &application_user_registration_token_expires_at,
                Type::INT8,
            )
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<Option<ApplicationUserRegistrationToken_3>, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                aurt.value AS v, \
                aurt.wrong_enter_tries_quantity AS wetq, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value::new(application_user_registration_token_value_),
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
            Ok(application_user_registration_token_wrong_enter_tries_quantity_) => ApplicationUserRegistrationToken_WrongEnterTriesQuantity::new(application_user_registration_token_wrong_enter_tries_quantity_),
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
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved::new(application_user_registration_token_is_approved_),
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
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt::new(application_user_registration_token_expires_at_),
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
                ApplicationUserRegistrationToken_3::new(
                    application_user_registration_token_value,
                    application_user_registration_token_wrong_enter_tries_quantity,
                    application_user_registration_token_is_approved,
                    application_user_registration_token_expires_at,
                ),
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserRegistrationToken_4> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>,
    {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_registration_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserRegistrationToken_WrongEnterTriesQuantity>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_registration_token_wrong_enter_tries_quantity,
                Type::INT2,
            )
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken_5> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, ApplicationUserRegistrationToken_IsApproved>,
    {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_registration_token_is_approved = <T as Getter<'a, ApplicationUserRegistrationToken_IsApproved>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "UPDATE ONLY public.application_user_registration_token AS aurt
            SET ( \
                is_approved \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurt.application_user_email = $2 AND aurt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_registration_token_is_approved,
                Type::BOOL,
            )
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

impl PostgresqlRepository<ApplicationUserRegistrationToken_6> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<Option<ApplicationUserRegistrationToken_6>, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                aurt.value AS v, \
                aurt.is_approved AS ia, \
                aurt.expires_at AS ea, \
                aurt.can_be_resent_from as cbrf \
            FROM public.application_user_registration_token aurt \
            WHERE aurt.application_user_email = $1 AND aurt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_email_,
                Type::TEXT,
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

        let application_user_registration_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_registration_token_value_) => ApplicationUserRegistrationToken_Value::new(application_user_registration_token_value_),
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
            Ok(application_user_registration_token_is_approved_) => ApplicationUserRegistrationToken_IsApproved::new(application_user_registration_token_is_approved_),
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
            Ok(application_user_registration_token_expires_at_) => ApplicationUserRegistrationToken_ExpiresAt::new(application_user_registration_token_expires_at_),
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
            Ok(application_user_registration_token_can_be_resent_from_) => ApplicationUserRegistrationToken_CanBeResentFrom::new(application_user_registration_token_can_be_resent_from_),
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
                ApplicationUserRegistrationToken_6::new(
                    application_user_registration_token_value,
                    application_user_registration_token_is_approved,
                    application_user_registration_token_expires_at,
                    application_user_registration_token_can_be_resent_from,
                ),
            ),
        );
    }
}

pub struct Insert<'a> {
    pub application_user_email: Cow<'a, ApplicationUser_Email>,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
    pub application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
    pub application_user_registration_token_is_approved: ApplicationUserRegistrationToken_IsApproved,
    pub application_user_registration_token_expires_at: ApplicationUserRegistrationToken_ExpiresAt,
    pub application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
}
