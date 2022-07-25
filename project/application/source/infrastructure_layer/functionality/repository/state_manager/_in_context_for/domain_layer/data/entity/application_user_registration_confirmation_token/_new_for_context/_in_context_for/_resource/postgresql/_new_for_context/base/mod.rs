use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::counter_u8::CounterU8;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::base::Base as UpdateResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        authorization_connection: &'a Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_confirmation_token.get_application_user_email();

        let value = application_user_registration_confirmation_token.get_value();

        let wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() as i16;

        let is_approved = application_user_registration_confirmation_token.get_is_approved();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "INSERT INTO public.application_user_registration_confirmation_token AS aurct ( \
                application_user_email, \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
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
            .add_parameter(&application_user_email, Type::VARCHAR)
            .add_parameter(&value, Type::VARCHAR)
            .add_parameter(&wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&is_approved, Type::BOOL);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be inserted into Postgresql database.") },
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
            WHERE aurct.application_user_email = $1 \
            RETURNING \
                1::SMALLINT;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::VARCHAR);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be deleted from Postgresql database.") },
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
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>,
        update_resolver: UpdateResolver
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_confirmation_token.get_application_user_email();

        let value = application_user_registration_confirmation_token.get_value();

        let wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() as i16;

        let is_approved = application_user_registration_confirmation_token.get_is_approved();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

        let mut column_name_for_value_registry: Option<(String, String)> = None;
        if update_resolver.get_update_value() {
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
                    "value".to_string(),
                    "$".to_string() + counter_u8_value.to_string().as_str()
                )
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(&value, Type::TEXT);
        }
        if update_resolver.get_update_wrong_enter_tries_quantity() {
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
                    column_name_registry = column_name_registry + ", wrong_enter_tries_quantity";
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
                            "wrong_enter_tries_quantity".to_string(),
                            "$".to_string() + counter_u8_value.to_string().as_str()
                        )
                    );
                }
            }

            prepared_statemant_parameter_convertation_resolver.add_parameter(&wrong_enter_tries_quantity, Type::INT2);
        }
        if update_resolver.get_update_is_approved() {
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
                    column_name_registry = column_name_registry + ", is_approved";
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
                            "is_approved".to_string(),
                            "$".to_string() + counter_u8_value.to_string().as_str()
                        )
                    );
                }
            }

            prepared_statemant_parameter_convertation_resolver.add_parameter(&is_approved, Type::BOOL);
        }
        if update_resolver.get_update_created_at() {
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

        let query: String;
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
                    "UPDATE ONLY public.application_user_registration_confirmation_token AS aurct \
                    SET ("
                    .to_string()
                    + column_name_registry.as_str()
                    + ") = ROW("
                    + column_value_registry.as_str()
                    + ") \
                    WHERE aurct.application_user_email = $" + counter_u8_value.to_string().as_str()
                    + " RETURNING \
                        aurct.application_user_email AS aue;";
                
                prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::VARCHAR);
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

        match authorization_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
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
}