use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::counter_u8::CounterU8;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        authorization_connection: &'a mut Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let applicaion_user_email = application_user_registration_confirmation_token.get_application_user_email();

        let value = application_user_registration_confirmation_token.get_value();

        let wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() as i16;

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "INSERT INTO public.application_user_registration_confirmation_token AS aurct ( \
                applicaion_user_email, \
                value, \
                wrong_enter_tries_quantity, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                DEFAULT \
            ) \
            ON CONFLICT DO NOTHING \
            RETURNING \
                1::SMALLINT;";
        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&applicaion_user_email, Type::TEXT)
            .add_parameter(&value, Type::TEXT)
            .add_parameter(&wrong_enter_tries_quantity, Type::INT2);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be inserted into Postgesql database.") },
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

    pub async fn update_created_at<'a>(
        authorization_connection: &'a mut Connection,
        application_user_registration_confrimation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_confrimation_token.get_application_user_email();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let counter_u8_value: u8;

        let query: String;

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
            SET ( \
                created_at \
            ) = ROW( \
                DEFAULT \
            ) \
            WHERE aurct.application_user_email = $"
            .to_string()
            + counter_u8_value.to_string().as_str()
            + " RETURNING \
                1::SMALLINT;";
        
        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        match authorization_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUserRegistrationConfirmationToken can not be updated in Postgesql database.") },
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