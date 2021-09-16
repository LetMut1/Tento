use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPreConfirmedPostgesqlTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base::Base as ApplicationUserPreConfirmedFactory;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;
use postgres::Client as Connection;

pub struct Base;

impl DataProviderApplicationUserPreConfirmedPostgesqlTrait for Base {
    type Error = BaseError;

    fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_email: &'outer_a str
    ) -> Result<bool, Self::Error> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver = PreparedStatementParameterConvertationResolver::new();
        
        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                aupc.id as i \
            FROM public.application_user_pre_confirmed aupc \
            WHERE aupc.application_user_email = $"
            .to_string();
        query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            return Ok(true);
        }

        return Ok(false);
    }

    fn find_by_application_user_email<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_email: &'outer_a str
    ) -> Result<Option<ApplicationUserPreConfirmed>, Self::Error> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver = PreparedStatementParameterConvertationResolver::new();
        
        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                aupc.id as i, \
                aupc.application_user_email as aue, \
                aupc.created_at::TEXT as ca \
            FROM public.application_user_pre_confirmed aupc \
            WHERE aupc.application_user_email = $"
            .to_string();
        query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            return Ok(Some(
                ApplicationUserPreConfirmed::new(
                    Some(row_registry[0].try_get::<'_, usize, i64>(0)?),
                    row_registry[0].try_get::<'_, usize, String>(1)?,
                    row_registry[0].try_get::<'_, usize, String>(2)?
                )
            ));
        }

        return Ok(None);
    }
}