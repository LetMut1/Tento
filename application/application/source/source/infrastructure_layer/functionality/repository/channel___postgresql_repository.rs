use super::postgresql_repository::PostgresqlRepository;
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
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;
use std::borrow::Cow;

#[cfg(feature = "manual_testing")]
use extern_crate::serde::Deserialize;

impl PostgresqlRepository<Channel<'_>> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert: Insert,
    ) -> Result<Channel<'static>, ErrorAuditor> {
        let channel_owner = insert.channel_owner.get();

        let channel_name = insert.channel_name.get();

        let channel_linked_name = insert.channel_linked_name.get();

        let channel_description = match insert.channel_description {
            Some(ref channel_description_) => Some(channel_description_.get()),
            None => None,
        };

        let channel_access_modifier = insert.channel_access_modifier.get();

        let channel_visability_modifier = insert.channel_visability_modifier.get();

        let channel_orientation = insert.channel_orientation.get();

        let channel_cover_image_path = match insert.channel_cover_image_path {
            Some(ref channel_cover_image_path_) => Some(channel_cover_image_path_.get()),
            None => None,
        };

        let channel_background_image_path = match insert.channel_background_image_path {
            Some(ref channel_background_image_path_) => Some(channel_background_image_path_.get()),
            None => None,
        };

        let channel_subscribers_quantity = insert.channel_subscribers_quantity.get();

        let channel_marks_quantity = insert.channel_marks_quantity.get();

        let channel_viewing_quantity = insert.channel_viewing_quantity.get();

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
                &channel_owner,
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
                &channel_access_modifier,
                Type::INT2,
            )
            .add_parameter(
                &channel_visability_modifier,
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
                &channel_subscribers_quantity,
                Type::INT8,
            )
            .add_parameter(
                &channel_marks_quantity,
                Type::INT8,
            )
            .add_parameter(
                &channel_viewing_quantity,
                Type::INT8,
            );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_id_) => Channel_Id::new(channel_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(channel_created_at_) => Channel_CreatedAt::new(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
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
                Channel_AccessModifier::new(channel_access_modifier),
                Channel_VisabilityModifier::new(channel_visability_modifier),
                insert.channel_orientation,
                insert.channel_cover_image_path,
                insert.channel_background_image_path,
                insert.channel_subscribers_quantity,
                insert.channel_marks_quantity,
                insert.channel_viewing_quantity,
                channel_created_at,
            ),
        );
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        channel_id: Channel_Id,
    ) -> Result<Option<Channel<'static>>, ErrorAuditor> {
        let channel_id_ = channel_id.get();

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
            &channel_id_,
            Type::INT8,
        );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let channel_owner = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_owner) => ApplicationUser_Id::new(channel_owner),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_name = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(channel_name_) => Channel_Name::new(channel_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_linked_name = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(channel_linked_name_) => Channel_LinkedName::new(channel_linked_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3) {
            Ok(channel_description_) => channel_description_.map(Channel_Description::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_access_modifier = match row_registry[0].try_get::<'_, usize, i16>(4) {
            Ok(channel_access_modifier_) => Channel_AccessModifier::new(channel_access_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_visability_modifier = match row_registry[0].try_get::<'_, usize, i16>(5) {
            Ok(channel_visability_modifier_) => Channel_VisabilityModifier::new(channel_visability_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_orientation = match row_registry[0].try_get::<'_, usize, Vec<i16>>(6) {
            Ok(channel_orientation_) => Channel_Orientation::new(channel_orientation_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7) {
            Ok(channel_cover_image_path_) => channel_cover_image_path_.map(Channel_CoverImagePath::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8) {
            Ok(channel_background_image_path_) => channel_background_image_path_.map(Channel_BackgroundImagePath::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_subscribers_quantity = match row_registry[0].try_get::<'_, usize, i64>(9) {
            Ok(channel_subscribers_quantity_) => Channel_SubscribersQuantity::new(channel_subscribers_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_marks_quantity = match row_registry[0].try_get::<'_, usize, i64>(10) {
            Ok(channel_marks_quantity_) => Channel_MarksQuantity::new(channel_marks_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_viewing_quantity = match row_registry[0].try_get::<'_, usize, i64>(11) {
            Ok(channel_viewing_quantity_) => Channel_ViewingQuantity::new(channel_viewing_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(12) {
            Ok(channel_created_at_) => Channel_CreatedAt::new(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
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
                    channel_created_at,
                ),
            ),
        );
    }

    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        channel_name: &'a Channel_Name,
    ) -> Result<Option<Channel<'a>>, ErrorAuditor> {
        let channel_name_ = channel_name.get();

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
            &channel_name_,
            Type::TEXT,
        );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let channel_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(channel_id_) => Channel_Id::new(channel_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_owner = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(channel_owner_) => ApplicationUser_Id::new(channel_owner_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_linked_name = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(channel_linked_name_) => Channel_LinkedName::new(channel_linked_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_description = match row_registry[0].try_get::<'_, usize, Option<String>>(3) {
            Ok(channel_description_) => channel_description_.map(Channel_Description::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_access_modifier = match row_registry[0].try_get::<'_, usize, i16>(4) {
            Ok(channel_access_modifier_) => Channel_AccessModifier::new(channel_access_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_visability_modifier = match row_registry[0].try_get::<'_, usize, i16>(5) {
            Ok(channel_visability_modifier_) => Channel_VisabilityModifier::new(channel_visability_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_orientation = match row_registry[0].try_get::<'_, usize, Vec<i16>>(6) {
            Ok(channel_orientation_) => Channel_Orientation::new(channel_orientation_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_cover_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(7) {
            Ok(channel_cover_image_path_) => channel_cover_image_path_.map(Channel_CoverImagePath::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_background_image_path = match row_registry[0].try_get::<'_, usize, Option<String>>(8) {
            Ok(channel_background_image_path_) => channel_background_image_path_.map(Channel_BackgroundImagePath::new),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_subscribers_quantity = match row_registry[0].try_get::<'_, usize, i64>(9) {
            Ok(channel_subscribers_quantity_) => Channel_SubscribersQuantity::new(channel_subscribers_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_marks_quantity = match row_registry[0].try_get::<'_, usize, i64>(10) {
            Ok(channel_marks_quantity_) => Channel_MarksQuantity::new(channel_marks_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_viewing_quantity = match row_registry[0].try_get::<'_, usize, i64>(11) {
            Ok(channel_viewing_quantity_) => Channel_ViewingQuantity::new(channel_viewing_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_created_at = match row_registry[0].try_get::<'_, usize, String>(12) {
            Ok(channel_created_at_) => Channel_CreatedAt::new(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
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
                    channel_created_at,
                ),
            ),
        );
    }
}

pub struct Insert {
    pub channel_owner: ApplicationUser_Id,
    pub channel_name: Channel_Name,
    pub channel_linked_name: Channel_LinkedName,
    pub channel_description: Option<Channel_Description>,
    pub channel_access_modifier: Channel_AccessModifier,
    pub channel_visability_modifier: Channel_VisabilityModifier,
    pub channel_orientation: Channel_Orientation,
    pub channel_cover_image_path: Option<Channel_CoverImagePath>,
    pub channel_background_image_path: Option<Channel_BackgroundImagePath>,
    pub channel_subscribers_quantity: Channel_SubscribersQuantity,
    pub channel_marks_quantity: Channel_MarksQuantity,
    pub channel_viewing_quantity: Channel_ViewingQuantity,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel1 {
    pub channel_id: Channel_Id,
    pub channel_name: Channel_Name,
    pub channel_linked_name: Channel_LinkedName,
    pub channel_access_modifier: Channel_AccessModifier,
    pub channel_visability_modifier: Channel_VisabilityModifier,
    pub channel_cover_image_path: Option<Channel_CoverImagePath>,
    pub channel_background_image_path: Option<Channel_BackgroundImagePath>,
}
