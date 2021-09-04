use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_component::channel::Channel as ResponseGetManyByCreatedAtChannel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_component::channel::Channel as ResponseGetManyByNameChannel;
use postgres::Client as Connection;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO 

impl Base {
    pub fn get_many_by_name<'outer_a>(
        connection: &'outer_a mut Connection, name: &'outer_a str, requery_name: &'outer_a Option<String>, limit: u8
    ) -> Result<Option<Vec<ResponseGetManyByNameChannel>>, BaseError> {
        let limit: i16 = limit as i16;

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver = PreparedStatementParameterConvertationResolver::new();
        
        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                c.id as i, \
                c.name as n, \
                c.personalization_image_path as pip, \
                c.subscribers_quantity as sq, \
                c.public_marks_quantity as pmq, \
                c.hidden_marks_quantity as hmq, \
                c.reactions_quantity as rq, \
                c.viewing_quantity as vq, \
                c.created_at::TEXT as ca \
            FROM public.channel c \
            WHERE c.is_private = FALSE AND c.name LIKE $"
            .to_string();
        query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

        let wildcard: String = name.to_string() + "%";
        prepared_statemant_parameter_convertation_resolver.add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_name) = requery_name {
            query = query + " AND c.name > $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_name, Type::TEXT);
        }

        query = query + " ORDER BY c.name ASC LIMIT $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<ResponseGetManyByNameChannel> = Vec::new();

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            for row in row_registry.iter() {
                let channel: ResponseGetManyByNameChannel = ResponseGetManyByNameChannel::new(
                    row.try_get::<'_, usize, i64>(0)?,
                    row.try_get::<'_, usize, String>(1)?,
                    row.try_get::<'_, usize, String>(2)?,
                    row.try_get::<'_, usize, i64>(3)?,
                    row.try_get::<'_, usize, i64>(4)?,
                    row.try_get::<'_, usize, i64>(5)?,
                    row.try_get::<'_, usize, i64>(6)?,
                    row.try_get::<'_, usize, i64>(7)?,
                    row.try_get::<'_, usize, String>(8)?,
                );

                channel_registry.push(channel);
            }

            return Ok(Some(channel_registry));
        }

        return Ok(None);
    }

    pub fn get_many_by_created_at<'outer_a>(
        connection: &'outer_a mut Connection, created_at: &'outer_a str, requery_created_at: &'outer_a Option<String>, order: u8, limit: u8
    ) -> Result<Option<Vec<ResponseGetManyByCreatedAtChannel>>, BaseError> {
        let limit: i16 = limit as i16;

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver = PreparedStatementParameterConvertationResolver::new();

        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                c.id as i, \
                c.name as n, \
                c.personalization_image_path as pip, \
                c.subscribers_quantity as sq, \
                c.public_marks_quantity as pmq, \
                c.hidden_marks_quantity as hmq, \
                c.reactions_quantity as rq, \
                c.viewing_quantity as vq, \
                c.created_at::TEXT as ca \
            FROM public.channel c \
            WHERE c.is_private = FALSE AND c.created_at < $"
            .to_string();
        query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

        prepared_statemant_parameter_convertation_resolver.add_parameter(&created_at, Type::TEXT);

        if let Some(requery_created_at) = requery_created_at {
            if OrderConventionResolver::is_asc(order) {
                query = query + " AND c.created_at > $";
            }
            if OrderConventionResolver::is_desc(order) {
                query = query + " AND c.created_at < $";
            }
            query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_created_at, Type::TEXT);
        }

        query = query + " ORDER BY " + OrderConventionResolver::convert(order)? +
        " LIMIT $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<ResponseGetManyByCreatedAtChannel> = Vec::new();

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            for row in row_registry.iter() {
                let channel: ResponseGetManyByCreatedAtChannel = ResponseGetManyByCreatedAtChannel::new(
                    row.try_get::<'_, usize, i64>(0)?,
                    row.try_get::<'_, usize, String>(1)?,
                    row.try_get::<'_, usize, String>(2)?,
                    row.try_get::<'_, usize, i64>(3)?,
                    row.try_get::<'_, usize, i64>(4)?,
                    row.try_get::<'_, usize, i64>(5)?,
                    row.try_get::<'_, usize, i64>(6)?,
                    row.try_get::<'_, usize, i64>(7)?,
                    row.try_get::<'_, usize, String>(8)?,
                );

                channel_registry.push(channel);
            }

            return Ok(Some(channel_registry));
        }

        return Ok(None);
    }
}