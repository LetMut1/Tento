use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
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
pub struct ApplicationUserLogInToken_PostgresqlRepository;

impl ApplicationUserLogInToken_PostgresqlRepository {
    pub async fn create<'a, 'b>(authorization_connection: &'a Connection, insert: Insert<'b>) -> Result<ApplicationUserLogInToken<'b>, ErrorAuditor> {
        let wrong_enter_tries_quantity_ = insert.application_user_log_in_token_wrong_enter_tries_quantity as i16;

        let quantity_of_minute_for_expiration = ApplicationUserLogInToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_log_in_token AS aulit ( \
                application_user_id, \
                device_id, \
                value, \
                wrong_enter_tries_quantity, \
                expires_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                current_timestamp(6) + (INTERVAL '1 MINUTE' * $5)::INTERVAL \
            ) \
            RETURNING \
                aulit.expires_at::TEXT AS ea;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_id, Type::INT8)
            .add_parameter(&insert.application_user_log_in_token_device_id, Type::TEXT)
            .add_parameter(&insert.application_user_log_in_token_value, Type::TEXT)
            .add_parameter(&wrong_enter_tries_quantity_, Type::INT2)
            .add_parameter(&quantity_of_minute_for_expiration, Type::INT2);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let application_user_log_in_token_expires_at = match row_registry[0].try_get::<'_, usize, String>(0) {
                                Ok(application_user_log_in_token_expires_at_) => application_user_log_in_token_expires_at_,
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
                                ApplicationUserLogInToken::new(
                                    insert.application_user_id,
                                    insert.application_user_log_in_token_device_id,
                                    insert.application_user_log_in_token_value,
                                    insert.application_user_log_in_token_wrong_enter_tries_quantity,
                                    application_user_log_in_token_expires_at
                                )
                            );
                        }

                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserLogInToken can not be inserted into Postgresql database.") },
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

    pub async fn update<'a>(authorization_connection: &'a Connection, application_user_log_in_token: &'a ApplicationUserLogInToken<'_>) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user_log_in_token.get_application_user_id();

        let application_user_log_in_token_device_id = application_user_log_in_token.get_device_id();

        let application_user_log_in_token_value = application_user_log_in_token.get_value();

        let application_user_log_in_token_wrong_enter_tries_quantity = application_user_log_in_token.get_wrong_enter_tries_quantity() as i16;

        let application_user_log_in_token_expires_at = application_user_log_in_token.get_expires_at();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
        "UPDATE ONLY public.application_user_log_in_token AS aulit \
        SET ( \
            value, \
            wrong_enter_tries_quantity, \
            expires_at \
        ) = ROW( \
            $1, \
            $2, \
            $3::TIMESTAMP(6) WITH TIME ZONE \
        ) \
        WHERE aulit.application_user_id = $4 AND aulit.device_id = $5 \
        RETURNING \
            aulit.application_user_id AS aui;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_log_in_token_value, Type::TEXT)
            .add_parameter(&application_user_log_in_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_log_in_token_expires_at, Type::TEXT)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserLogInToken can not be updated in Postgresql database.") },
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
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_log_in_token AS aulit \
            WHERE aulit.application_user_id = $1 AND aulit.device_id = $2;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);
        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

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
    ) -> Result<Option<ApplicationUserLogInToken<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aulit.value AS v, \
                aulit.wrong_enter_tries_quantity AS wetq, \
                aulit.expires_at::TEXT AS ca \
            FROM public.application_user_log_in_token aulit \
            WHERE aulit.application_user_id = $1 AND aulit.device_id = $2;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);
        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_log_in_token_device_id, Type::TEXT);

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

                            let expires_at = match row_registry[0].try_get::<'_, usize, String>(2) {
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
                                    ApplicationUserLogInToken::new(
                                        application_user_id,
                                        application_user_log_in_token_device_id,
                                        value,
                                        wrong_enter_tries_quantity,
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
    pub application_user_id: i64,
    pub application_user_log_in_token_device_id: &'a str,
    pub application_user_log_in_token_value: String,
    pub application_user_log_in_token_wrong_enter_tries_quantity: u8
}