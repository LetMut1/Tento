use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::insert::Insert;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a, 'b>(
        authorization_connection: &'a Connection,
        insert: Insert<'b>
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'b>, ErrorAuditor> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity,
            is_approved
        ) = insert.into_inner();

        let wrong_enter_tries_quantity_ = wrong_enter_tries_quantity as i16;

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
            ON CONFLICT DO NOTHING \
            RETURNING \
                aurct.expires_at::TEXT AS ea;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_email, Type::VARCHAR)
            .add_parameter(&value, Type::VARCHAR)
            .add_parameter(&wrong_enter_tries_quantity_, Type::INT2)
            .add_parameter(&is_approved, Type::BOOL)
            .add_parameter(&quantity_of_minute_for_expiration, Type::INT2);

        match authorization_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match authorization_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let expires_at = match row_registry[0].try_get::<'_, usize, String>(0) {
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

                            let application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(
                                application_user_email,
                                value,
                                wrong_enter_tries_quantity,
                                is_approved,
                                expires_at
                            );

                            return Ok(application_user_registration_confirmation_token);
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

    pub async fn delete<'a>(
        authorization_connection: &'a Connection,
        application_user_email: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_registration_confirmation_token AS aurct \
            WHERE aurct.application_user_email = $1 \
            RETURNING \
                aurct.application_user_email AS aue;";

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
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = application_user_registration_confirmation_token.get_application_user_email();

        let value = application_user_registration_confirmation_token.get_value();

        let wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity() as i16;

        let is_approved = application_user_registration_confirmation_token.get_is_approved();

        let expires_at = application_user_registration_confirmation_token.get_expires_at();

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
            current_timestamp(6) + INTERVAL '$5 MINUTE' \
        ) \
        ON CONFLICT ON CONSTRAINT application_user_registration_confirmation_token3 DO \
        UPDATE SET ( \
            value, \
            wrong_enter_tries_quantity, \
            is_approved, \
            expires_at \
        ) = ROW( \
            $6, \
            $7, \
            $8, \
            $9::TIMESTAMP(6) WITH TIME ZONE \
        ) \
        WHERE aurct.application_user_email = $10 \
        RETURNING \
            aurct.application_user_email AS aue;";

        prepared_statemant_parameter_convertation_resolver
        .add_parameter(&application_user_email, Type::VARCHAR)
        .add_parameter(&value, Type::VARCHAR)
        .add_parameter(&wrong_enter_tries_quantity, Type::INT2)
        .add_parameter(&is_approved, Type::BOOL)
        .add_parameter(&quantity_of_minute_for_expiration, Type::INT2)
        .add_parameter(&value, Type::VARCHAR)
        .add_parameter(&wrong_enter_tries_quantity, Type::INT2)
        .add_parameter(&is_approved, Type::BOOL)
        .add_parameter(&expires_at, Type::VARCHAR)
        .add_parameter(&application_user_email, Type::VARCHAR);

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
}