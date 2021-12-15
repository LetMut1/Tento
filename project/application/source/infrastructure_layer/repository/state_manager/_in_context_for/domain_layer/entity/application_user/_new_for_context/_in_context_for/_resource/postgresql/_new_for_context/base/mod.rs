use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserStateManagerPostgresqlTrait;
use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
use postgres::Client as Connection;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;

impl ApplicationUserStateManagerPostgresqlTrait for Base {
    type Error = BaseError;
    type UpdateResolverApplicationUser = UpdateResolverApplicationUser;

    fn create<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser
    ) -> Result<(), Self::Error> {
        let email: &'_ str = application_user.get_email();

        let nickanme: &'_ str = application_user.get_nickname();

        let password_hash: &'_ str = application_user.get_password_hash();

        let created_at: &'_ str = application_user.get_created_at();

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let query: &'static str = 
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

        let statement: Statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if row_registry.is_empty() {
            return Err(BaseError::LogicError {unreachable: false, message: "ApplicationUser can not be inserted into Postgesql database."});
        }

        return Ok(());
    }

    fn update<'a>(
        connection: &'a mut Connection,
        application_user: &'a ApplicationUser,
        update_resolver: Self::UpdateResolverApplicationUser
    ) -> Result<(), Self::Error> {
        if !update_resolver.should_update() {
            return Err(BaseError::LogicError {unreachable: false, message: "The columns allowing update should exist for ApplicationUser."})
        }

        let email: &'_ str = application_user.get_email();

        let nickanme: &'_ str = application_user.get_nickname();

        let password_hash: &'_ str = application_user.get_password_hash();

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

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
                        },
                        None => {
                            return Err(BaseError::LogicError {unreachable: true, message: "The columns value description should exist for ApplicationUser update."});
                        }
                    }
                },
                None => {
                    column_name_registry_description = Some("nickname".to_string());

                    match column_value_registry_description {
                        Some (_) => {
                            return Err(BaseError::LogicError {unreachable: true, message: "The columns value description should not exist for ApplicationUser."});
                        },
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
                        },
                        None => {
                            return Err(BaseError::LogicError {unreachable: true, message: "The columns value description should exist for ApplicationUser update."});
                        }
                    }
                },
                None => {
                    column_name_registry_description = Some("password_hash".to_string());

                    match column_value_registry_description {
                        Some (_) => {
                            return Err(BaseError::LogicError {unreachable: true, message: "The columns value description should not exist for ApplicationUser."});
                        },
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
                            + ") = ("
                            + column_value_registry_description_.as_str()
                            + ") \
                            WHERE au.id = $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str()
                            + " RETURNING \
                                au.id AS i;";
                        
                        prepared_statemant_parameter_convertation_resolver.add_parameter(application_user.get_id()?, Type::INT8);
                    },
                    None => {
                        return Err(BaseError::LogicError {unreachable: true, message: "The columns value description should exist for ApplicationUser update."})
                    }
                }
            },
            None => {
                return Err(BaseError::LogicError {unreachable: true, message: "The columns name description should exist for ApplicationUser update."})
            }
        }

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if row_registry.is_empty() {
            return Err(BaseError::LogicError {unreachable: false, message: "ApplicationUser can not be updated in Postgesql database."});
        }

        return Ok(());
    }
}