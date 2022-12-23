use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::integer_type_converter::IntegerTypeConverter;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationConfirmationToken_PostgresqlRepository;

impl ApplicationUserRegistrationConfirmationToken_PostgresqlRepository {
    pub async fn create<'a, 'b>(
        authorization_connection: &'a Connection,
        insert: Insert<'b>
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'b>, ErrorAuditor> {
        let (
            application_user_email,
            application_user_registration_confirmation_token_value,
            application_user_registration_confirmation_token_wrong_enter_tries_quantity,
            application_user_registration_confirmation_token_is_approved
        ) = insert.into_inner();

        let wrong_enter_tries_quantity_ = application_user_registration_confirmation_token_wrong_enter_tries_quantity as i16;

        let quantity_of_minute_for_expiration = ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_registration_confirmation_token AS aurct ( \
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
                current_timestamp(6) + (INTERVAL '1 MINUTE' * $5)::INTERVAL \
            ) \
            RETURNING \
                aurct.expires_at::TEXT AS ea;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_email, Type::TEXT)
            .add_parameter(&application_user_registration_confirmation_token_value, Type::TEXT)
            .add_parameter(&wrong_enter_tries_quantity_, Type::INT2)
            .add_parameter(&application_user_registration_confirmation_token_is_approved, Type::BOOL)
            .add_parameter(&quantity_of_minute_for_expiration, Type::INT2);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let application_user_registration_confirmation_token_expires_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(application_user_registration_confirmation_token_expires_at_) => application_user_registration_confirmation_token_expires_at_,
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
                                ApplicationUserRegistrationConfirmationToken::new(
                                    application_user_email,
                                    application_user_registration_confirmation_token_value,
                                    application_user_registration_confirmation_token_wrong_enter_tries_quantity,
                                    application_user_registration_confirmation_token_is_approved,
                                    application_user_registration_confirmation_token_expires_at
                                )
                            );
                        }

                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be inserted into Postgresql database.") },
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

    pub async fn update<'a>(
        authorization_connection: &'a Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_confirmation_token.get_application_user_email();

        let application_user_registration_confirmation_token_value = application_user_registration_confirmation_token.get_value();

        let application_user_registration_confirmation_token_wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() as i16;

        let application_user_registration_confirmation_token_is_approved = application_user_registration_confirmation_token.get_is_approved();

        let application_user_registration_confirmation_token_expires_at = application_user_registration_confirmation_token.get_expires_at();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
        "UPDATE ONLY public.application_user_registration_confirmation_token AS aurct
        SET ( \
            value, \
            wrong_enter_tries_quantity, \
            is_approved, \
            expires_at \
        ) = ROW( \
            $1, \
            $2, \
            $3, \
            $4::TIMESTAMP(6) WITH TIME ZONE \
        ) \
        WHERE aurct.application_user_email = $5 \
        RETURNING \
            aurct.application_user_email AS aue;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_registration_confirmation_token_value, Type::TEXT)
            .add_parameter(&application_user_registration_confirmation_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_registration_confirmation_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_registration_confirmation_token_expires_at, Type::TEXT)
            .add_parameter(&application_user_email, Type::TEXT);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be updated in Postgresql database.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }

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

    pub async fn delete<'a>(
        authorization_connection: &'a Connection,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_registration_confirmation_token AS aurct \
            WHERE aurct.application_user_email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

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
        application_user_email: &'a str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurct.value AS v, \
                aurct.wrong_enter_tries_quantity AS wetq, \
                aurct.is_approved AS ia, \
                aurct.expires_at::TEXT AS ea \
            FROM public.application_user_registration_confirmation_token aurct \
            WHERE aurct.application_user_email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let value = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(value_) => value_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            let wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
                                Ok(wrong_enter_tries_quantity_) => {
                                    match IntegerTypeConverter::convert_i16_to_u8(wrong_enter_tries_quantity_) {
                                        Ok(wrong_enter_tries_quantity__) => wrong_enter_tries_quantity__,
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                            return Err(error);
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
                            };

                            let is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
                                Ok(is_approved_) => is_approved_,
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            };

                            let expires_at = match row_registry[0].try_get::<'_, usize, String>(3) {
                                Ok(expires_at_) => expires_at_,
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
                                    ApplicationUserRegistrationConfirmationToken::new(
                                        application_user_email,
                                        value,
                                        wrong_enter_tries_quantity,
                                        is_approved,
                                        expires_at
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
    application_user_email: &'a str,
    application_user_registration_confirmation_token_value: String,
    application_user_registration_confirmation_token_wrong_enter_tries_quantity: u8,
    application_user_registration_confirmation_token_is_approved: bool
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_email: &'a str,
        application_user_registration_confirmation_token_value: String,
        application_user_registration_confirmation_token_wrong_enter_tries_quantity: u8,
        application_user_registration_confirmation_token_is_approved: bool
    ) -> Self {
        return Self {
            application_user_email,
            application_user_registration_confirmation_token_value,
            application_user_registration_confirmation_token_wrong_enter_tries_quantity,
            application_user_registration_confirmation_token_is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (&'a str, String, u8, bool) {
        return (
            self.application_user_email,
            self.application_user_registration_confirmation_token_value,
            self.application_user_registration_confirmation_token_wrong_enter_tries_quantity,
            self.application_user_registration_confirmation_token_is_approved
        );
    }
}