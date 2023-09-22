use super::postgresql_repository::by::By6;
use super::postgresql_repository::by::By7;
use super::postgresql_repository::insert::Insert7;
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
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
pub use action_processor_incoming_outcoming::Channel1;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<Channel<'_>> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_7: Insert7,
    ) -> Result<Channel<'static>, ErrorAuditor> {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_id_) => Channel_Id(channel_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_created_at_) => Channel_CreatedAt(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Channel {
                id: channel_id,
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
                created_at: channel_created_at,
            },
        );
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_6: &'a By6,
    ) -> Result<Option<Channel<'static>>, ErrorAuditor> {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_owner) => ApplicationUser_Id(channel_owner),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_name_) => Channel_Name(channel_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_linked_name_) => Channel_LinkedName(channel_linked_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_description_) => {
                let channel_description__ = match channel_description_ {
                    Some(channel_description___) => Some(Channel_Description(channel_description___)),
                    None => None,
                };

                channel_description__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_access_modifier_) => Channel_AccessModifier(channel_access_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_visability_modifier_) => Channel_VisabilityModifier(channel_visability_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_orientation_) => Channel_Orientation(channel_orientation_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_cover_image_path_) => {
                let channel_cover_image_path__ = match channel_cover_image_path_ {
                    Some(channel_cover_image_path___) => Some(Channel_CoverImagePath(channel_cover_image_path___)),
                    None => None,
                };

                channel_cover_image_path__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_background_image_path_) => {
                let channel_background_image_path__ = match channel_background_image_path_ {
                    Some(channel_background_image_path___) => Some(Channel_BackgroundImagePath(channel_background_image_path___)),
                    None => None,
                };

                channel_background_image_path__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_subscribers_quantity_) => Channel_SubscribersQuantity(channel_subscribers_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_marks_quantity_) => Channel_MarksQuantity(channel_marks_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_viewing_quantity_) => Channel_ViewingQuantity(channel_viewing_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_created_at_) => Channel_CreatedAt(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                Channel {
                    id: by_6.channel_id,
                    owner: channel_owner,
                    name: Cow::Owned(channel_name),
                    linked_name: channel_linked_name,
                    description: channel_description,
                    access_modifier: channel_access_modifier,
                    visability_modifier: channel_visability_modifier,
                    orientation: channel_orientation,
                    cover_image_path: channel_cover_image_path,
                    background_image_path: channel_background_image_path,
                    subscribers_quantity: channel_subscribers_quantity,
                    marks_quantity: channel_marks_quantity,
                    viewing_quantity: channel_viewing_quantity,
                    created_at: channel_created_at,
                },
            ),
        );
    }

    pub async fn find_2<'a, 'b>(
        database_1_connection: &'a Connection,
        by_7: &'a By7<'b>,
    ) -> Result<Option<Channel<'b>>, ErrorAuditor> {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_id_) => Channel_Id(channel_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_owner_) => ApplicationUser_Id(channel_owner_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_linked_name_) => Channel_LinkedName(channel_linked_name_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_description_) => {
                let channel_description__ = match channel_description_ {
                    Some(channel_description___) => Some(Channel_Description(channel_description___)),
                    None => None,
                };

                channel_description__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_access_modifier_) => Channel_AccessModifier(channel_access_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_visability_modifier_) => Channel_VisabilityModifier(channel_visability_modifier_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_orientation_) => Channel_Orientation(channel_orientation_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_cover_image_path_) => {
                let channel_cover_image_path__ = match channel_cover_image_path_ {
                    Some(channel_cover_image_path___) => Some(Channel_CoverImagePath(channel_cover_image_path___)),
                    None => None,
                };

                channel_cover_image_path__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_background_image_path_) => {
                let channel_background_image_path__ = match channel_background_image_path_ {
                    Some(channel_background_image_path___) => Some(Channel_BackgroundImagePath(channel_background_image_path___)),
                    None => None,
                };

                channel_background_image_path__
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_subscribers_quantity_) => Channel_SubscribersQuantity(channel_subscribers_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_marks_quantity_) => Channel_MarksQuantity(channel_marks_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_viewing_quantity_) => Channel_ViewingQuantity(channel_viewing_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_created_at_) => Channel_CreatedAt(channel_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                Channel {
                    id: channel_id,
                    owner: channel_owner,
                    name: Cow::Borrowed(by_7.channel_name),
                    linked_name: channel_linked_name,
                    description: channel_description,
                    access_modifier: channel_access_modifier,
                    visability_modifier: channel_visability_modifier,
                    orientation: channel_orientation,
                    cover_image_path: channel_cover_image_path,
                    background_image_path: channel_background_image_path,
                    subscribers_quantity: channel_subscribers_quantity,
                    marks_quantity: channel_marks_quantity,
                    viewing_quantity: channel_viewing_quantity,
                    created_at: channel_created_at,
                },
            ),
        );
    }
}
