use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::counter_u8::CounterU8;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::base::Base as UpdateResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        core_connection: &'a Connection,
        application_user: &'a ApplicationUser
    ) -> Result<i64, ErrorAuditor> {
        let email = application_user.get_email();

        let nickanme = application_user.get_nickname();

        let password_hash = application_user.get_password_hash();

        let created_at = application_user.get_created_at();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "INSERT INTO public.application_user AS au ( \
                id, \
                email, \
                nickname, \
                password_hash, \
                created_at \
            ) VALUES ( \
                nextval('public.application_user1'), \
                $1, \
                $2, \
                $3, \
                $4::TIMESTAMP(6) WITH TIME ZONE \
            ) \
            ON CONFLICT DO NOTHING \
            RETURNING \
                au.id AS i;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&email, Type::VARCHAR)
            .add_parameter(&nickanme, Type::VARCHAR)
            .add_parameter(&password_hash, Type::TEXT)
            .add_parameter(&created_at, Type::TEXT);

        match core_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be inserted into Postgresql database.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }

                        let id = match row_registry[0].try_get::<'_, usize, i64>(0) {
                            Ok(id_) => id_,
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        };

                        return Ok(id);
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
        core_connection: &'a Connection,
        application_user: &'a ApplicationUser,
        update_resolver: UpdateResolver
    ) -> Result<(), ErrorAuditor> {
        let application_user_id = match application_user.get_id() {
            Some(application_user_id_) => application_user_id_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "The application_user_id should exist.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let email = application_user.get_email();

        let nickanme = application_user.get_nickname();

        let password_hash = application_user.get_password_hash();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

        let mut column_name_for_value_registry: Option<(String, String)> = None;
        if update_resolver.get_update_email() {
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
                    "email".to_string(),
                    "$".to_string() + counter_u8_value.to_string().as_str()
                )
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::VARCHAR);
        }
        if update_resolver.get_update_nickname() {
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
                    column_name_registry += ", nickname";
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
                            "nickname".to_string(),
                            "$".to_string() + counter_u8_value.to_string().as_str()
                        )
                    );
                }
            }

            prepared_statemant_parameter_convertation_resolver.add_parameter(&nickanme, Type::VARCHAR);
        }
        if update_resolver.get_update_password_hash() {
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
                    column_name_registry += ", password_hash";
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
                            "password_hash".to_string(),
                            "$".to_string() + counter_u8_value.to_string().as_str()
                        )
                    );
                }
            }

            prepared_statemant_parameter_convertation_resolver.add_parameter(&password_hash, Type::TEXT);
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
                    "UPDATE ONLY public.application_user AS au \
                    SET ("
                    .to_string()
                    + column_name_registry.as_str()
                    + ") = ROW("
                    + column_value_registry.as_str()
                    + ") \
                    WHERE au.id = $" + counter_u8_value.to_string().as_str()
                    + " RETURNING \
                        au.id AS i;";
                        
                    prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);
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

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be updated in Postgresql database.") },
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