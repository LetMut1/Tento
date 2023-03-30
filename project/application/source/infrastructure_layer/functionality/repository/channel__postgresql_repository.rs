use crate::domain_layer::data::entity::channel::AccessModifier;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::data::entity::channel::VisabilityModifier;
use crate::domain_layer::functionality::service::channel__access_modifier_resolver::Channel_AccessModifierResolver;
use crate::domain_layer::functionality::service::channel__visability_modifier_resolver::Channel_VisabilityModifierResolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use std::borrow::Cow;

pub struct Channel_PostgresqlRepository;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO     // TODO !!!!!!!1  TODO  TODO  TODO  TODO  Если извне оборачивать в транзакцию, что будет с декларирование подготовленного запроса? То есть: Бегин- создать препэрэд стэйстмент - иполнить пр ст- коммит/роллбэу

impl Channel_PostgresqlRepository {
    pub async fn create<'a>(database_1_connection: &'a Connection, insert: Insert) -> Result<Channel<'static>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let channel_access_modifier = Channel_AccessModifierResolver::from_representation(insert.channel_access_modifier);

        let channel_visability_modifier = Channel_VisabilityModifierResolver::from_representation(insert.channel_visability_modifier);
        let query =
            "INSERT INTO public.channel AS c ( \
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
            .add_parameter(&insert.channel_owner, Type::INT8)
            .add_parameter(&insert.channel_name, Type::TEXT)
            .add_parameter(&insert.channel_linked_name, Type::TEXT)
            .add_parameter(&insert.channel_description, Type::TEXT)
            .add_parameter(&channel_access_modifier, Type::INT2)
            .add_parameter(&channel_visability_modifier, Type::INT2)
            .add_parameter(&insert.channel_orientation, Type::INT2_ARRAY)
            .add_parameter(&insert.channel_cover_image_path, Type::TEXT)
            .add_parameter(&insert.channel_background_image_path, Type::TEXT)
            .add_parameter(&insert.channel_subscribers_quantity, Type::INT8)
            .add_parameter(&insert.channel_marks_quantity, Type::INT8)
            .add_parameter(&insert.channel_viewing_quantity, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_id_) => channel_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(channel_created_at_) => channel_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Channel::new(
                channel_id,
                insert.channel_owner,
                Cow::Owned(insert.channel_name),
                insert.channel_linked_name,
                insert.channel_description,
                channel_access_modifier,
                channel_visability_modifier,
                insert.channel_orientation,
                insert.channel_cover_image_path,
                insert.channel_background_image_path,
                insert.channel_subscribers_quantity,
                insert.channel_marks_quantity,
                insert.channel_viewing_quantity,
                channel_created_at
            )
        );
    }

    pub async fn find_1<'a>(database_1_connection: &'a Connection, channel_id: i64) -> Result<Option<Channel<'static>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
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

        prepared_statemant_parameter_convertation_resolver.add_parameter(&channel_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let channel_owner = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_owner) => channel_owner,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_name = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(channel_name_) => channel_name_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_linked_name = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(channel_linked_name_) => channel_linked_name_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3) {
            Ok(channel_description_) => channel_description_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_access_modifier = match row_registry[0].try_get::<'_, usize, i16>(4) {
            Ok(channel_access_modifier_) => channel_access_modifier_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_visability_modifier = match row_registry[0].try_get::<'_, usize, i16>(5) {
            Ok(channel_visability_modifier_) => channel_visability_modifier_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_orientation = match row_registry[0].try_get::<'_, usize, Vec<i16>>(6) {
            Ok(channel_orientation_) => channel_orientation_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7) {
            Ok(channel_cover_image_path_) => channel_cover_image_path_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8) {
            Ok(channel_background_image_path_) => channel_background_image_path_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_subscribers_quantity = match row_registry[0].try_get::<'_, usize, i64>(9) {
            Ok(channel_subscribers_quantity_) => channel_subscribers_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_marks_quantity = match row_registry[0].try_get::<'_, usize, i64>(10) {
            Ok(channel_marks_quantity_) => channel_marks_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_viewing_quantity = match row_registry[0].try_get::<'_, usize, i64>(11) {
            Ok(channel_viewing_quantity_) => channel_viewing_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(12) {
            Ok(channel_created_at_) => channel_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                Channel::new(
                    channel_id,
                    channel_owner,
                    Cow::Owned(channel_name),
                    channel_linked_name,
                    channel_description,
                    channel_access_modifier,
                    channel_visability_modifier,
                    channel_orientation,
                    channel_cover_image_path,
                    channel_background_image_path,
                    channel_subscribers_quantity,
                    channel_marks_quantity,
                    channel_viewing_quantity,
                    channel_created_at
                )
            )
        );
    }

    pub async fn find_2<'a>(database_1_connection: &'a Connection, channel_name: &'a str) -> Result<Option<Channel<'a>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
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

        prepared_statemant_parameter_convertation_resolver.add_parameter(&channel_name, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let channel_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_id_) => channel_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_owner = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(channel_owner_) => channel_owner_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_linked_name = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(channel_linked_name_) => channel_linked_name_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3) {
            Ok(channel_description_) => channel_description_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_access_modifier = match row_registry[0].try_get::<'_, usize, i16>(4) {
            Ok(channel_access_modifier_) => channel_access_modifier_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_visability_modifier = match row_registry[0].try_get::<'_, usize, i16>(5) {
            Ok(channel_visability_modifier_) => channel_visability_modifier_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_orientation = match row_registry[0].try_get::<'_, usize, Vec<i16>>(6) {
            Ok(channel_orientation_) => channel_orientation_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7) {
            Ok(channel_cover_image_path_) => channel_cover_image_path_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8) {
            Ok(channel_background_image_path_) => channel_background_image_path_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_subscribers_quantity = match row_registry[0].try_get::<'_, usize, i64>(9) {
            Ok(channel_subscribers_quantity_) => channel_subscribers_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_marks_quantity = match row_registry[0].try_get::<'_, usize, i64>(10) {
            Ok(channel_marks_quantity_) => channel_marks_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_viewing_quantity = match row_registry[0].try_get::<'_, usize, i64>(11) {
            Ok(channel_viewing_quantity_) => channel_viewing_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(12) {
            Ok(channel_created_at_) => channel_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                Channel::new(
                    channel_id,
                    channel_owner,
                    Cow::Borrowed(channel_name),
                    channel_linked_name,
                    channel_description,
                    channel_access_modifier,
                    channel_visability_modifier,
                    channel_orientation,
                    channel_cover_image_path,
                    channel_background_image_path,
                    channel_subscribers_quantity,
                    channel_marks_quantity,
                    channel_viewing_quantity,
                    channel_created_at
                )
            )
        );
    }
}

pub struct Insert {
    pub channel_owner: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_description: Option<String>,
    pub channel_access_modifier: AccessModifier,
    pub channel_visability_modifier: VisabilityModifier,
    pub channel_orientation: Vec<i16>,
    pub channel_cover_image_path: Option<String>,
    pub channel_background_image_path: Option<String>,
    pub channel_subscribers_quantity: i64,
    pub channel_marks_quantity: i64,
    pub channel_viewing_quantity: i64,
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel1 {
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_access_modifier: i16,
    pub channel_visability_modifier: i16,
    pub channel_cover_image_path: Option<String>,
    pub channel_background_image_path: Option<String>
}