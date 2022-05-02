use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::counter_u8::CounterU8;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection,
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
            .add_parameter(&email, Type::TEXT)
            .add_parameter(&nickanme, Type::TEXT)
            .add_parameter(&password_hash, Type::TEXT)
            .add_parameter(&created_at, Type::TEXT);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be inserted into Postgesql database.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }

                        let id: i64;
                        match row_registry[0].try_get::<'_, usize, i64>(0) {
                            Ok(id_) => {
                                id = id_;
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }

                
                        return Ok(id);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn update<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser,
        update_resolver: UpdateResolverApplicationUser
    ) -> Result<(), ErrorAuditor> {
        let application_user_id: i64;
        match application_user.get_id() {
            Some(application_user_id_) => {
                application_user_id = application_user_id_;
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::LogicError { logic_error: LogicError::new(false, "The application_user_id should exist.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }

        let email = application_user.get_email();

        let nickanme = application_user.get_nickname();

        let password_hash = application_user.get_password_hash();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

        let mut column_name_registry_description: Option<String> = None;
        let mut column_value_registry_description: Option<String> = None;
        if update_resolver.is_update_email() {
            column_name_registry_description = Some("email".to_string());

            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }
            column_value_registry_description = Some(
                "$".to_string() + counter_u8_value.to_string().as_str()
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);
        }
        if update_resolver.is_update_nickname() {
            match column_name_registry_description {
                Some(mut column_name_registry_description_) => {
                    column_name_registry_description_ = column_name_registry_description_ + ", nickname";
                    
                    column_name_registry_description = Some(column_name_registry_description_);

                    match column_value_registry_description {
                        Some(mut column_value_registry_description_) => {
                            match counter_u8.get_next() {
                                Ok(counter_) => {
                                    counter_u8_value = counter_;
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
                            }
                            column_value_registry_description_ = column_value_registry_description_+ ", $" + counter_u8_value.to_string().as_str();

                            column_value_registry_description = Some(column_value_registry_description_);

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&nickanme, Type::TEXT);
                        }
                        None => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                None => {
                    column_name_registry_description = Some("nickname".to_string());

                    match column_value_registry_description {
                        Some(_) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns value description should not exist for ApplicationUser update.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                        None => {
                            match counter_u8.get_next() {
                                Ok(counter_) => {
                                    counter_u8_value = counter_;
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
                            }
                            column_value_registry_description = Some("$".to_string() + counter_u8_value.to_string().as_str());

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&nickanme, Type::TEXT);
                        }
                    }
                }
            }
        }
        if update_resolver.is_update_password_hash() {
            match column_name_registry_description {
                Some(mut column_name_registry_description_) => {
                    column_name_registry_description_ = column_name_registry_description_ + ", password_hash";

                    column_name_registry_description = Some(column_name_registry_description_);

                    match column_value_registry_description {
                        Some(mut column_value_registry_description_) => {
                            match counter_u8.get_next() {
                                Ok(counter_) => {
                                    counter_u8_value = counter_;
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
                            }
                            column_value_registry_description_ = column_value_registry_description_+ ", $" + counter_u8_value.to_string().as_str();

                            column_value_registry_description = Some(column_value_registry_description_);

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&password_hash, Type::TEXT);
                        }
                        None => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                None => {
                    column_name_registry_description = Some("password_hash".to_string());

                    match column_value_registry_description {
                        Some(_) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns value description should not exist for ApplicationUser update.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                        None => {
                            match counter_u8.get_next() {
                                Ok(counter_) => {
                                    counter_u8_value = counter_;
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
                            }
                            column_value_registry_description = Some("$".to_string() + counter_u8_value.to_string().as_str());

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&password_hash, Type::TEXT);
                        }
                    }
                }
            }
        }

        let query: String;
        match column_name_registry_description {
            Some(column_name_registry_description_) => {
                match column_value_registry_description {
                    Some(column_value_registry_description_) => {
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
                            + column_name_registry_description_.as_str()
                            + ") = ROW("
                            + column_value_registry_description_.as_str()
                            + ") \
                            WHERE au.id = $" + counter_u8_value.to_string().as_str()
                            + " RETURNING \
                                au.id AS i;";
                        
                        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);
                    }
                    None => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.") },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::LogicError { logic_error: LogicError::new(true, "The columns name description should exist for ApplicationUser update.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }

        match connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be updated in Postgesql database.") },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                
                        return Ok(());
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}