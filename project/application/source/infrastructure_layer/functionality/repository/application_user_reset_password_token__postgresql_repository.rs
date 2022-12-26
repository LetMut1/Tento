use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::integer_type_converter::IntegerTypeConverter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct ApplicationUserResetPasswordToken_PostgresqlRepository;

impl ApplicationUserResetPasswordToken_PostgresqlRepository {
    pub async fn create<'a>(
        authorization_connection: &'a Connection,
        insert: Insert
    ) -> Result<ApplicationUserResetPasswordToken, ErrorAuditor> {
        let (
            application_user_reset_password_token_application_user_id,
            application_user_reset_password_token_value,
            application_user_reset_password_token_wrong_enter_tries_quantity,
            application_user_reset_password_token_is_approved
        ) = insert.into_inner();

        let wrong_enter_tries_quantity_ = application_user_reset_password_token_wrong_enter_tries_quantity as i16;

        let quantity_of_minute_for_expiration = ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_reset_password_token AS aurpt ( \
                application_user_id, \
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
                aurpt.expires_at::TEXT AS ea;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_application_user_id, Type::INT8)
            .add_parameter(&application_user_reset_password_token_value, Type::TEXT)
            .add_parameter(&wrong_enter_tries_quantity_, Type::INT2)
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&quantity_of_minute_for_expiration, Type::INT2);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let application_user_reset_password_token_expires_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
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
                                ApplicationUserResetPasswordToken::new(
                                    application_user_reset_password_token_application_user_id,
                                    application_user_reset_password_token_value,
                                    application_user_reset_password_token_wrong_enter_tries_quantity,
                                    application_user_reset_password_token_is_approved,
                                    application_user_reset_password_token_expires_at
                                )
                            );
                        }

                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserResetPasswordToken can not be inserted into Postgresql database.") },
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
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user_reset_password_token.get_application_user_id();

        let application_user_reset_password_token_value = application_user_reset_password_token.get_value();

        let application_user_reset_password_token_wrong_enter_tries_quantity = application_user_reset_password_token.get_wrong_enter_tries_quantity() as i16;

        let application_user_reset_password_token_is_approved = application_user_reset_password_token.get_is_approved();

        let application_user_reset_password_token_expires_at = application_user_reset_password_token.get_expires_at();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
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
            WHERE aurpt.application_user_id = $5 \
            RETURNING \
                aurpt.application_user_id AS aui;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_value, Type::TEXT)
            .add_parameter(&application_user_reset_password_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_reset_password_token_expires_at, Type::TEXT)
            .add_parameter(&application_user_id, Type::INT8);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserResetPasswordToken can not be updated in Postgresql database.") },
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
        application_user_id: i64
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_reset_password_token AS aurpt \
            WHERE aurpt.application_user_id = $1;";

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
        application_user_id: i64
    ) -> Result<Option<ApplicationUserResetPasswordToken>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurpt.value AS v, \
                aurpt.wrong_enter_tries_quantity AS wetq, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at::TEXT AS ca \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

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
                                    ApplicationUserResetPasswordToken::new(
                                        application_user_id,
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

pub struct Insert {
    application_user_id: i64,
    application_user_reset_password_token_value: String,
    application_user_reset_password_token_wrong_enter_tries_quantity: u8,
    application_user_reset_password_token_is_approved: bool,
}

impl<'a> Insert {
    pub fn new(
        application_user_id: i64,
        application_user_reset_password_token_value: String,
        application_user_reset_password_token_wrong_enter_tries_quantity: u8,
        application_user_reset_password_token_is_approved: bool,
    ) -> Self {
        return Self {
            application_user_id,
            application_user_reset_password_token_value,
            application_user_reset_password_token_wrong_enter_tries_quantity,
            application_user_reset_password_token_is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (i64, String, u8, bool) {
        return (
            self.application_user_id,
            self.application_user_reset_password_token_value,
            self.application_user_reset_password_token_wrong_enter_tries_quantity,
            self.application_user_reset_password_token_is_approved
        );
    }
}