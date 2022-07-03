use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::counter_u8::CounterU8;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::base::Base as UpdateResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        authorization_connection: &'a Connection,
        application_user_log_in_token: &'a ApplicationUserLogInToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let applicaion_user_id = application_user_log_in_token.get_application_user_id();

        let device_id = application_user_log_in_token.get_device_id();

        let value = application_user_log_in_token.get_value();

        let wrong_enter_tries_quantity = application_user_log_in_token.get_wrong_enter_tries_quantity() as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "INSERT INTO public.application_user_log_in_token AS aulit ( \
                applicaion_user_id, \
                device_id, \
                value, \
                wrong_enter_tries_quantity, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                DEFAULT \
            ) \
            ON CONFLICT DO NOTHING \
            RETURNING \
                1::SMALLINT;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&applicaion_user_id, Type::INT8)
            .add_parameter(&device_id, Type::TEXT)
            .add_parameter(&value, Type::VARCHAR)
            .add_parameter(&wrong_enter_tries_quantity, Type::INT2);

        match authorization_connection.prepare_typed(query, &prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()[..]).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, &prepared_statemant_parameter_convertation_resolver.get_parameter_registry()[..]).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserLogInToken can not be inserted into Postgesql database.") },
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
        application_user_log_in_token: &'a ApplicationUserLogInToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let applicaion_user_id = application_user_log_in_token.get_application_user_id();

        let device_id = application_user_log_in_token.get_device_id();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "DELETE FROM ONLY public.application_user_log_in_token AS aulit \
            WHERE aulit.application_user_id = $1 AND aulit.device_id = $2 RETURNING \
                1::SMALLINT;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&applicaion_user_id, Type::INT8);
        prepared_statemant_parameter_convertation_resolver.add_parameter(&device_id, Type::TEXT);

        match authorization_connection.prepare_typed(query, &prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()[..]).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, &prepared_statemant_parameter_convertation_resolver.get_parameter_registry()[..]).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserLogInToken can not be deleted from Postgesql database.") },
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

    pub async fn update<'a>(
        authorization_connection: &'a Connection,
        application_user_log_in_token: &'a ApplicationUserLogInToken<'_>,
        update_resolver: UpdateResolver
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user_log_in_token.get_application_user_id();

        let devcie_id = application_user_log_in_token.get_device_id();

        let wrong_enter_tries_quantity = application_user_log_in_token.get_wrong_enter_tries_quantity() as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

        let mut column_name_for_value_registry: Option<(String, String)> = None;
        if update_resolver.is_update_wrong_enter_tries_quantity() {
            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }

            column_name_for_value_registry = Some(
                (
                    "wrong_enter_tries_quantity".to_string(),
                    "$".to_string() + counter_u8_value.to_string().as_str()
                )
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(&wrong_enter_tries_quantity, Type::INT2);
        }
        if update_resolver.is_update_created_at() {
            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }

            match column_name_for_value_registry {
                Some((mut column_name_registry, mut column_value_registry)) => {
                    column_name_registry = column_name_registry + ", created_at";
                    column_value_registry = column_value_registry + ", $" + counter_u8_value.to_string().as_str();

                    column_name_for_value_registry = Some(
                        (
                            column_name_registry,
                            column_value_registry
                        )
                    );
                }
                None => {
                    column_name_for_value_registry = Some(
                        (
                            "created_at".to_string(),
                            "$".to_string() + counter_u8_value.to_string().as_str()
                        )
                    );
                }
            }

            prepared_statemant_parameter_convertation_resolver.add_parameter(&"DEFAULT", Type::TEXT);
        }

        let mut query: String;
        match column_name_for_value_registry {
            Some((column_name_registry, column_value_registry)) => {
                match counter_u8.get_next() {
                    Ok(counter_) => {
                        counter_u8_value = counter_;
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
                
                query = 
                    "UPDATE ONLY public.application_user_log_in_token AS aulit \
                    SET ("
                    .to_string()
                    + column_name_registry.as_str()
                    + ") = ROW("
                    + column_value_registry.as_str()
                    + ") \
                    WHERE aulit.application_user_id = $" + counter_u8_value.to_string().as_str();

                prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

                match counter_u8.get_next() {
                    Ok(counter_) => {
                        counter_u8_value = counter_;
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
                
                query = query
                    + " AND aulit.device_id = $" + counter_u8_value.to_string().as_str()
                    + " RETURNING \
                        aulit.application_user_id AS aui;";
                
                prepared_statemant_parameter_convertation_resolver.add_parameter(&devcie_id, Type::TEXT);
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "There are no values to update.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }

        match authorization_connection.prepare_typed(query.as_str(), &prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()[..]).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, &prepared_statemant_parameter_convertation_resolver.get_parameter_registry()[..]).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserLogInToken can not be updated in Postgesql database.") },
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
}