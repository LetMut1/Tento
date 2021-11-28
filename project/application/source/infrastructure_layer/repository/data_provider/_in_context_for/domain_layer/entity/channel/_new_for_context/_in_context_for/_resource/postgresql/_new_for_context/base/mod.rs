use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_counter::PreparedStatementParameterCounter;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_component::channel::Channel as ResponseGetManyByCreatedAtChannel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::_component::channel::Channel as ResponseGetManyByIdRegistryChannel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_component::channel::Channel as ResponseGetManyByNameChannel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_component::channel::Channel as ResponseGetManyBySubscribersQuantityChannel;
use postgres::Client as Connection;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO 
                    // TODO !!!!!!!1  TODO  TODO  TODO  TODO  Если извне оборачивать в транзакцию, что будет с декларирование подготовленного запроса? То есть: Бегин- создать препэрэд стэйстмент - иполнить пр ст- коммит/роллбэу
impl Base {
    pub fn find_many_by_name<'a>(
        connection: &'a mut Connection,
        name: &'a str,
        requery_name: &'a Option<String>,
        limit: &'a i16
    ) -> Result<Option<Vec<ResponseGetManyByNameChannel>>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();
        
        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.personalization_image_path AS pip, \
                c.subscribers_quantity AS sq, \
                c.public_marks_quantity AS pmq, \
                c.hidden_marks_quantity AS hmq, \
                c.reactions_quantity AS rq, \
                c.viewing_quantity AS vq, \
                c.created_at::TEXT AS ca \
            FROM public.channel c \
            WHERE c.is_private = FALSE AND c.name LIKE $"
            .to_string()
            + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

        let wildcard: String = name.to_string() + "%";
        prepared_statemant_parameter_convertation_resolver.add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_name) = requery_name {
            query = query + " AND c.name > $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_name, Type::TEXT);
        }

        query = query + " ORDER BY c.name ASC LIMIT $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(limit, Type::INT2);

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

    pub fn find_many_by_created_at<'a>(
        connection: &'a mut Connection,
        created_at: &'a Option<String>,
        order: &'a i8,
        limit: &'a i16
    ) -> Result<Option<Vec<ResponseGetManyByCreatedAtChannel>>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.personalization_image_path AS pip, \
                c.subscribers_quantity AS sq, \
                c.public_marks_quantity AS pmq, \
                c.hidden_marks_quantity AS hmq, \
                c.reactions_quantity AS rq, \
                c.viewing_quantity AS vq, \
                c.created_at::TEXT AS ca \
            FROM public.channel c \
            WHERE c.is_private = FALSE AND c.created_at <= current_timestamp(6)"
            .to_string();

        if let Some(created_at) = created_at {
            if OrderConventionResolver::is_asc(order) {
                query = query + " AND c.created_at > $";
            }
            if OrderConventionResolver::is_desc(order) {
                query = query + " AND c.created_at < $";
            }
            query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + "::TIMESTAMP(6) WITH TIME ZONE";

            prepared_statemant_parameter_convertation_resolver.add_parameter(created_at, Type::TEXT);
        }

        query = query + " ORDER BY c.created_at " + OrderConventionResolver::convert(order)? +
        " LIMIT $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(limit, Type::INT2);

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

    pub fn find_many_by_subscribers_quantity<'a>(
        connection: &'a mut Connection,
        subscribers_quantity: &'a Option<i64>,
        order: &'a i8,
        limit: &'a i16
    ) -> Result<Option<Vec<ResponseGetManyBySubscribersQuantityChannel>>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let mut prepared_statemant_parameter_counter: PreparedStatementParameterCounter = PreparedStatementParameterCounter::new();

        let mut query: String = 
            "SELECT \
                c.id AS i, \
                c.subscribers_quantity AS sq \
            FROM public.channel c \
            WHERE c.is_private = FALSE"
            .to_string();

        if let Some(subscribers_quantity) = subscribers_quantity {
            if OrderConventionResolver::is_asc(order) {
                query = query + " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) > public.limit_channel_subscribers_quantity($";
            }
            if OrderConventionResolver::is_desc(order) {
                query = query + " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) < public.limit_channel_subscribers_quantity($";
            }
            query = query + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ")";

            prepared_statemant_parameter_convertation_resolver.add_parameter(subscribers_quantity, Type::INT8);
        }

        query = query + " ORDER BY public.limit_channel_subscribers_quantity(c.subscribers_quantity) " + OrderConventionResolver::convert(order)? +
        " LIMIT $" + prepared_statemant_parameter_counter.get_next()?.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(limit, Type::INT2);

        let mut channel_registry: Vec<ResponseGetManyBySubscribersQuantityChannel> = Vec::new();

        let statement: Statement = connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            for row in row_registry.iter() {
                let channel: ResponseGetManyBySubscribersQuantityChannel = ResponseGetManyBySubscribersQuantityChannel::new(
                    row.try_get::<'_, usize, i64>(0)?,
                    row.try_get::<'_, usize, i64>(1)?
                );

                channel_registry.push(channel);
            }

            return Ok(Some(channel_registry));
        }

        return Ok(None);
    }

    pub fn find_many_by_id_registry<'a>(
        connection: &'a mut Connection,
        id_registry: &'a Vec<i64>
    ) -> Result<Option<Vec<ResponseGetManyByIdRegistryChannel>>, BaseError> {
        if id_registry.is_empty() {
            return Ok(None)
        }

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let query: &'static str = 
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.personalization_image_path AS pip, \
                c.subscribers_quantity AS sq, \
                c.public_marks_quantity AS pmq, \
                c.hidden_marks_quantity AS hmq, \
                c.reactions_quantity AS rq, \
                c.viewing_quantity AS vq, \
                c.created_at::TEXT AS ca \
            FROM public.channel c \
            WHERE c.is_private = FALSE AND c.id = ANY($1);";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&id_registry, Type::INT8_ARRAY);

        let mut channel_registry: Vec<ResponseGetManyByIdRegistryChannel> = Vec::new();

        let statement: Statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if !row_registry.is_empty() {
            for row in row_registry.iter() {
                let channel: ResponseGetManyByIdRegistryChannel = ResponseGetManyByIdRegistryChannel::new(
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

