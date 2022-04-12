use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use crate::infrastructure_layer::error::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser
    ) -> Result<i64, ErrorAggregator> {
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

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if row_registry.is_empty() {
            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(false, "ApplicationUser can not be inserted into Postgesql database.")});
        }

        return Ok(row_registry[0].try_get::<'_, usize, i64>(0)?);
    }

    pub async fn update<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser,
        update_resolver: UpdateResolverApplicationUser
    ) -> Result<(), ErrorAggregator> {
        let application_user_id: &'_ i64;
        match application_user.get_id() {
            Some(application_user_id_) => {
                application_user_id = application_user_id_;
            }
            None => {
                return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Application_user_id should exist")});
            }
        }

        let email = application_user.get_email();

        let nickanme = application_user.get_nickname();

        let password_hash = application_user.get_password_hash();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut prepared_statemant_parameter_counter = PreparedStatementParameterCounter::new();

        let mut column_name_registry_description: Option<String> = None;
        let mut column_value_registry_description: Option<String> = None;
        if update_resolver.is_update_email() {
            column_name_registry_description = Some("email".to_string());

            column_value_registry_description = Some(
                "$".to_string() + prepared_statemant_parameter_counter.get_next()?.to_string().as_str()
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);
        }
        if update_resolver.is_update_nickname() {
            match column_name_registry_description {
                Some(mut column_name_registry_description_) => {
                    column_name_registry_description_ = column_name_registry_description_ + ", nickname";
                    
                    column_name_registry_description = Some(column_name_registry_description_);

                    match column_value_registry_description {
                        Some (mut column_value_registry_description_) => {
                            column_value_registry_description_ = column_value_registry_description_+ ", $"
                                + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

                            column_value_registry_description = Some(column_value_registry_description_);

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&nickanme, Type::TEXT);
                        }
                        None => {
                            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.")});
                        }
                    }
                }
                None => {
                    column_name_registry_description = Some("nickname".to_string());

                    match column_value_registry_description {
                        Some (_) => {
                            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns value description should not exist for ApplicationUser.")});
                        }
                        None => {
                            column_value_registry_description = Some(
                                "$".to_string() + prepared_statemant_parameter_counter.get_next()?.to_string().as_str()
                            );

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
                        Some (mut column_value_registry_description_) => {
                            column_value_registry_description_ = column_value_registry_description_+ ", $"
                                + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

                            column_value_registry_description = Some(column_value_registry_description_);

                            prepared_statemant_parameter_convertation_resolver.add_parameter(&password_hash, Type::TEXT);
                        }
                        None => {
                            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.")});
                        }
                    }
                }
                None => {
                    column_name_registry_description = Some("password_hash".to_string());

                    match column_value_registry_description {
                        Some (_) => {
                            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns value description should not exist for ApplicationUser.")});
                        }
                        None => {
                            column_value_registry_description = Some(
                                "$".to_string() + prepared_statemant_parameter_counter.get_next()?.to_string().as_str()
                            );

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
                        query = 
                            "UPDATE ONLY public.application_user AS au \
                            SET ("
                            .to_string()
                            + column_name_registry_description_.as_str()
                            + ") = ROW("
                            + column_value_registry_description_.as_str()
                            + ") \
                            WHERE au.id = $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str()
                            + " RETURNING \
                                au.id AS i;";
                        
                        prepared_statemant_parameter_convertation_resolver.add_parameter(application_user_id, Type::INT8);
                    }
                    None => {
                        return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns value description should exist for ApplicationUser update.")})
                    }
                }
            }
            None => {
                return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(true, "The columns name description should exist for ApplicationUser update.")})
            }
        }

        let statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if row_registry.is_empty() {
            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(false, "ApplicationUser can not be updated in Postgesql database.")});
        }

        return Ok(());
    }
}