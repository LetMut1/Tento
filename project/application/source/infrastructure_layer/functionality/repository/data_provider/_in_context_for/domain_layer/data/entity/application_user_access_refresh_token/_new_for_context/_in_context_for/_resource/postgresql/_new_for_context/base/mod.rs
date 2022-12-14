use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
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