use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::integer_type_converter::IntegerTypeConverter;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
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