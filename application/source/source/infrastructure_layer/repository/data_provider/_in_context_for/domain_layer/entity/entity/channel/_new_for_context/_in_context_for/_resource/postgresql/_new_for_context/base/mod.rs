use crate::domain_layer::entity::entity::channel::_component::name::Name;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_component::channel::Channel;
use postgres::Client as Connection;
use postgres::fallible_iterator::FallibleIterator;
use postgres::RowIter;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;    // TODO Имена ПрепСТейтентов, их отмена - нужно ли это все?

impl Base {
    pub fn get_many_by_name<'outer_a>(
        connection: &'outer_a mut Connection, name: &'outer_a Name, requery_channel_name: &'outer_a Option<Name>, limit: u8
    ) -> Result<Option<Vec<Channel>>, BaseError> {
        let mut query_parameter_type_registry: Vec<Type> = Vec::new();

        let mut query_parameter_registry: Vec<String> = Vec::new();

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
            WHERE c.is_private = FALSE AND c.name LIKE $1 "
            .to_string();

        query_parameter_type_registry.push(Type::TEXT);

        query_parameter_registry.push(name.get_value().to_string() + "%");

        if let Some(subquery_channel_name) = requery_channel_name {
            query.push_str("AND c.name > $2 ");

            query_parameter_type_registry.push(Type::TEXT);

            query_parameter_registry.push(subquery_channel_name.get_value().to_string());
        }

        query.push_str("ORDER BY c.name ASC LIMIT $3;");

        query_parameter_type_registry.push(Type::INT2);

        query_parameter_registry.push(limit.to_string());

        let mut channel_registry: Vec<Channel> = Vec::new();

        let statement: Statement = connection.prepare_typed(query.as_str(), &query_parameter_type_registry)?;

        let mut raw_iter: RowIter<'_> = connection.query_raw(&statement, query_parameter_registry)?;
        while let Some(row) = raw_iter.next()? {
            let channel: Channel = Channel::new(
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

        if !channel_registry.is_empty() {
            return Ok(Some(channel_registry));
        }

        return Ok(None);
    }
}