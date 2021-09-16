use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPreConfirmedPostgesqlTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use postgres::Client as Connection;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;

impl StateManagerApplicationUserPreConfirmedPostgesqlTrait for Base {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let query: &'static str = 
            "INSERT INTO public.application_user_pre_confirmed AS aupc ( \
                id, \
                application_user_email, \
                created_at \
            ) VALUES ( \
                nextval('public.application_user_pre_confirmed1'), \
                $1, \
                $2::TIMESTAMP(6) WITH TIME ZONE \
            ) \
            ON CONFLICT DO NOTHING \
            RETURNING \
                aupc.id AS i;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_pre_confirmed.get_application_user_email(), Type::TEXT);
        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_pre_confirmed.get_created_at(), Type::TEXT);

        let statement: Statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if row_registry.is_empty() {
            return Err(BaseError::LogicError {message: "ApplicationUserPreConfirmed can not be inserted into Postgesql database."});
        }

        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error> {
        todo!();
        return Ok(());
    }
}