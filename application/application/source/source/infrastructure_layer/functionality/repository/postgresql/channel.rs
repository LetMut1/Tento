use super::PostgresqlRepository;
use crate::domain_layer::data::entity::channel::Channel;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
pub use action_processor_incoming_outcoming::Channel1;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<Channel<'_>> {
    pub async fn create_1<'a>(
        database_1_connection: &'a Connection,
        insert_1: Insert1,
    ) -> Result<Channel<'static>, Auditor<Error>> {
        let channel_name = insert_1.channel_name.as_str();

        let channel_linked_name = insert_1.channel_linked_name.as_str();

        let channel_description = match insert_1.channel_description {
            Some(ref channel_description_) => Some(channel_description_.as_str()),
            None => None,
        };

        let channel_orientation = insert_1.channel_orientation.as_slice();

        let channel_cover_image_path = match insert_1.channel_cover_image_path {
            Some(ref channel_cover_image_path_) => Some(channel_cover_image_path_.as_str()),
            None => None,
        };

        let channel_background_image_path = match insert_1.channel_background_image_path {
            Some(ref channel_background_image_path_) => Some(channel_background_image_path_.as_str()),
            None => None,
        };

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.channel AS c ( \
                id, \
                owner, \
                name, \
                linked_name, \
                description, \
                access_modifier, \
                visability_modifier, \
                orientation, \
                cover_image_path, \
                background_image_path, \
                subscribers_quantity, \
                marks_quantity, \
                viewing_quantity, \
                created_at \
            ) VALUES ( \
                nextval('public.channel1'), \
                $1, \
                $2, \
                $3, \
                $4, \
                $5, \
                $6, \
                $7, \
                $8, \
                $9, \
                $10, \
                $11, \
                $12, \
                current_timestamp(6) \
            ) \
            RETURNING \
                c.id AS i,
                c.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert_1.channel_owner, Type::INT8)
            .add_parameter(&channel_name, Type::TEXT)
            .add_parameter(&channel_linked_name, Type::TEXT)
            .add_parameter(&channel_description, Type::TEXT)
            .add_parameter(&insert_1.channel_access_modifier, Type::INT2)
            .add_parameter(&insert_1.channel_visability_modifier, Type::INT2)
            .add_parameter(&channel_orientation, Type::INT2_ARRAY)
            .add_parameter(&channel_cover_image_path, Type::TEXT)
            .add_parameter(&channel_background_image_path, Type::TEXT)
            .add_parameter(&insert_1.channel_subscribers_quantity, Type::INT8)
            .add_parameter(&insert_1.channel_marks_quantity, Type::INT8)
            .add_parameter(&insert_1.channel_viewing_quantity, Type::INT8);

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        return Ok(Channel::new(
            row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
            insert_1.channel_owner,
            Cow::Owned(insert_1.channel_name),
            insert_1.channel_linked_name,
            insert_1.channel_description,
            insert_1.channel_access_modifier,
            insert_1.channel_visability_modifier,
            insert_1.channel_orientation,
            insert_1.channel_cover_image_path,
            insert_1.channel_background_image_path,
            insert_1.channel_subscribers_quantity,
            insert_1.channel_marks_quantity,
            insert_1.channel_viewing_quantity,
            row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
        ));
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_1: By1,
    ) -> Result<Option<Channel<'static>>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                c.owner AS ow, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.description AS d, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.orientation AS or, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip, \
                c.subscribers_quantity, \
                c.marks_quantity AS mq, \
                c.viewing_quantity AS vq, \
                c.created_at::TEXT AS ca \
            FROM public.channel c \
            WHERE c.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&by_1.channel_id, Type::INT8);

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(Some(Channel::new(
            by_1.channel_id,
            row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
            Cow::Owned(row_registry[0].try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?),
            row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(3).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i16>(4).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i16>(5).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Vec<i16>>(6).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(7).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(8).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(9).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(10).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(11).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, String>(12).convert(Backtrace::new(line!(), file!()))?,
        )));
    }

    pub async fn find_2<'a, 'b>(
        database_1_connection: &'a Connection,
        by_2: By2<'b>,
    ) -> Result<Option<Channel<'b>>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                c.id AS i, \
                c.owner AS ow, \
                c.linked_name AS ln, \
                c.description AS d, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.orientation AS or, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip, \
                c.subscribers_quantity, \
                c.marks_quantity AS mq, \
                c.viewing_quantity AS vq, \
                c.created_at::TEXT AS ca \
            FROM public.channel c \
            WHERE c.name = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&by_2.channel_name, Type::TEXT);

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        return Ok(Some(Channel::new(
            row_registry[0].try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(1).convert(Backtrace::new(line!(), file!()))?,
            Cow::Borrowed(by_2.channel_name),
            row_registry[0].try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(3).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i16>(4).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i16>(5).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Vec<i16>>(6).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(7).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, Option<String>>(8).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(9).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(10).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, i64>(11).convert(Backtrace::new(line!(), file!()))?,
            row_registry[0].try_get::<'_, usize, String>(12).convert(Backtrace::new(line!(), file!()))?,
        )));
    }
}

pub struct Insert1 {
    pub channel_owner: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_description: Option<String>,
    pub channel_access_modifier: i16,
    pub channel_visability_modifier: i16,
    pub channel_orientation: Vec<i16>,
    pub channel_cover_image_path: Option<String>,
    pub channel_background_image_path: Option<String>,
    pub channel_subscribers_quantity: i64,
    pub channel_marks_quantity: i64,
    pub channel_viewing_quantity: i64,
}

pub struct By1 {
    pub channel_id: i64,
}

pub struct By2<'a> {
    pub channel_name: &'a str,
}
