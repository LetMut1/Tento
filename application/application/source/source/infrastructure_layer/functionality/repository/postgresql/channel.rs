use super::by::By6;
use super::by::By7;
use super::insert::Insert7;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_BackgroundImagePath;
use crate::domain_layer::data::entity::channel::Channel_CoverImagePath;
use crate::domain_layer::data::entity::channel::Channel_CreatedAt;
use crate::domain_layer::data::entity::channel::Channel_Description;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_MarksQuantity;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_Orientation;
use crate::domain_layer::data::entity::channel::Channel_SubscribersQuantity;
use crate::domain_layer::data::entity::channel::Channel_ViewingQuantity;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
pub use action_processor_incoming_outcoming::Channel1;
use std::borrow::Cow;
use crate::infrastructure_layer::data::auditor::Converter;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<Channel<'_>> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_7: Insert7,
    ) -> Result<Channel<'static>, Auditor<Error>> {
        let channel_name = insert_7.channel_name.0.as_str();

        let channel_linked_name = insert_7.channel_linked_name.0.as_str();

        let channel_description = match insert_7.channel_description {
            Some(ref channel_description_) => Some(channel_description_.0.as_str()),
            None => None,
        };

        let channel_orientation = insert_7.channel_orientation.0.as_slice();

        let channel_cover_image_path = match insert_7.channel_cover_image_path {
            Some(ref channel_cover_image_path_) => Some(channel_cover_image_path_.0.as_str()),
            None => None,
        };

        let channel_background_image_path = match insert_7.channel_background_image_path {
            Some(ref channel_background_image_path_) => Some(channel_background_image_path_.0.as_str()),
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
            .add_parameter(
                &insert_7.channel_owner.0,
                Type::INT8,
            )
            .add_parameter(
                &channel_name,
                Type::TEXT,
            )
            .add_parameter(
                &channel_linked_name,
                Type::TEXT,
            )
            .add_parameter(
                &channel_description,
                Type::TEXT,
            )
            .add_parameter(
                &insert_7.channel_access_modifier.0,
                Type::INT2,
            )
            .add_parameter(
                &insert_7.channel_visability_modifier.0,
                Type::INT2,
            )
            .add_parameter(
                &channel_orientation,
                Type::INT2_ARRAY,
            )
            .add_parameter(
                &channel_cover_image_path,
                Type::TEXT,
            )
            .add_parameter(
                &channel_background_image_path,
                Type::TEXT,
            )
            .add_parameter(
                &insert_7.channel_subscribers_quantity.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_7.channel_marks_quantity.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_7.channel_viewing_quantity.0,
                Type::INT8,
            );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(BacktracePart::new(line!(), file!()))?;

        return Ok(
            Channel {
                id: Channel_Id(row_registry[0].try_get::<'_, usize, i64>(0).convert(BacktracePart::new(line!(), file!()))?),
                owner: insert_7.channel_owner,
                name: Cow::Owned(insert_7.channel_name),
                linked_name: insert_7.channel_linked_name,
                description: insert_7.channel_description,
                access_modifier: insert_7.channel_access_modifier,
                visability_modifier: insert_7.channel_visability_modifier,
                orientation: insert_7.channel_orientation,
                cover_image_path: insert_7.channel_cover_image_path,
                background_image_path: insert_7.channel_background_image_path,
                subscribers_quantity: insert_7.channel_subscribers_quantity,
                marks_quantity: insert_7.channel_marks_quantity,
                viewing_quantity: insert_7.channel_viewing_quantity,
                created_at: Channel_CreatedAt(row_registry[0].try_get::<'_, usize, String>(1).convert(BacktracePart::new(line!(), file!()))?),
            },
        );
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_6: &'a By6,
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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_6.channel_id.0,
            Type::INT8,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(BacktracePart::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }


        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_decription_) => Some(Channel_Description(channel_decription_)),
            None => None,
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_cover_image_path_) => Some(Channel_CoverImagePath(channel_cover_image_path_)),
            None => None,
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_background_image_path_) => Some(Channel_BackgroundImagePath(channel_background_image_path_)),
            None => None,
        };

        return Ok(
            Some(
                Channel {
                    id: by_6.channel_id,
                    owner: ApplicationUser_Id(row_registry[0].try_get::<'_, usize, i64>(0).convert(BacktracePart::new(line!(), file!()))?),
                    name: Cow::Owned(Channel_Name(row_registry[0].try_get::<'_, usize, String>(1).convert(BacktracePart::new(line!(), file!()))?)),
                    linked_name: Channel_LinkedName(row_registry[0].try_get::<'_, usize, String>(2).convert(BacktracePart::new(line!(), file!()))?),
                    description: channel_description,
                    access_modifier: Channel_AccessModifier(row_registry[0].try_get::<'_, usize, i16>(4).convert(BacktracePart::new(line!(), file!()))?),
                    visability_modifier: Channel_VisabilityModifier(row_registry[0].try_get::<'_, usize, i16>(5).convert(BacktracePart::new(line!(), file!()))?),
                    orientation: Channel_Orientation(row_registry[0].try_get::<'_, usize, Vec<i16>>(6).convert(BacktracePart::new(line!(), file!()))?),
                    cover_image_path: channel_cover_image_path,
                    background_image_path: channel_background_image_path,
                    subscribers_quantity: Channel_SubscribersQuantity(row_registry[0].try_get::<'_, usize, i64>(9).convert(BacktracePart::new(line!(), file!()))?),
                    marks_quantity: Channel_MarksQuantity(row_registry[0].try_get::<'_, usize, i64>(10).convert(BacktracePart::new(line!(), file!()))?),
                    viewing_quantity: Channel_ViewingQuantity(row_registry[0].try_get::<'_, usize, i64>(11).convert(BacktracePart::new(line!(), file!()))?),
                    created_at: Channel_CreatedAt(row_registry[0].try_get::<'_, usize, String>(12).convert(BacktracePart::new(line!(), file!()))?),
                },
            ),
        );
    }

    pub async fn find_2<'a, 'b>(
        database_1_connection: &'a Connection,
        by_7: &'a By7<'b>,
    ) -> Result<Option<Channel<'b>>, Auditor<Error>> {
        let channel_name = by_7.channel_name.0.as_str();

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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &channel_name,
            Type::TEXT,
        );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(BacktracePart::new(line!(), file!()))?;

        if row_registry.is_empty() {
            return Ok(None);
        }

        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_description_) => Some(Channel_Description(channel_description_)),
            None => None,
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_cover_image_path_) => Some(Channel_CoverImagePath(channel_cover_image_path_)),
            None => None,
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8).convert(BacktracePart::new(line!(), file!()))? {
            Some(channel_background_image_path_) => Some(Channel_BackgroundImagePath(channel_background_image_path_)),
            None => None,
        };

        return Ok(
            Some(
                Channel {
                    id: Channel_Id(row_registry[0].try_get::<'_, usize, i64>(0).convert(BacktracePart::new(line!(), file!()))?),
                    owner: ApplicationUser_Id(row_registry[0].try_get::<'_, usize, i64>(1).convert(BacktracePart::new(line!(), file!()))?),
                    name: Cow::Borrowed(by_7.channel_name),
                    linked_name: Channel_LinkedName(row_registry[0].try_get::<'_, usize, String>(2).convert(BacktracePart::new(line!(), file!()))?),
                    description: channel_description,
                    access_modifier: Channel_AccessModifier(row_registry[0].try_get::<'_, usize, i16>(4).convert(BacktracePart::new(line!(), file!()))?),
                    visability_modifier: Channel_VisabilityModifier(row_registry[0].try_get::<'_, usize, i16>(5).convert(BacktracePart::new(line!(), file!()))?),
                    orientation: Channel_Orientation(row_registry[0].try_get::<'_, usize, Vec<i16>>(6).convert(BacktracePart::new(line!(), file!()))?),
                    cover_image_path: channel_cover_image_path,
                    background_image_path: channel_background_image_path,
                    subscribers_quantity: Channel_SubscribersQuantity(row_registry[0].try_get::<'_, usize, i64>(9).convert(BacktracePart::new(line!(), file!()))?),
                    marks_quantity: Channel_MarksQuantity(row_registry[0].try_get::<'_, usize, i64>(10).convert(BacktracePart::new(line!(), file!()))?),
                    viewing_quantity: Channel_ViewingQuantity(row_registry[0].try_get::<'_, usize, i64>(11).convert(BacktracePart::new(line!(), file!()))?),
                    created_at: Channel_CreatedAt(row_registry[0].try_get::<'_, usize, String>(12).convert(BacktracePart::new(line!(), file!()))?),
                },
            ),
        );
    }
}
