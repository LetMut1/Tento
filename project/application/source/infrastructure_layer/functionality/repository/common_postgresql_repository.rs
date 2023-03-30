use crate::domain_layer::data::entity::channel::VisabilityModifier;
use crate::domain_layer::functionality::service::channel__visability_modifier_resolver::Channel_VisabilityModifierResolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel1;
use crate::infrastructure_layer::functionality::service::counter::Counter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

pub struct CommonPostgresqlRepository;

impl CommonPostgresqlRepository {
    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        application_user_id: i64,
        channel_name: &'a str,
        requery_channel_name: &'a Option<String>,
        limit: i16
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let counter_value_1 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let counter_value_2 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let channel_visability_modifier = Channel_VisabilityModifierResolver::from_representation(VisabilityModifier::Public);

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip, \
                cs.channel_id AS ca \
            FROM public.channel c LEFT OUTER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id \
            WHERE c.visability_modifier = ${} AND c.name LIKE ${}",
            counter_value_1,
            counter_value_2,
            counter_value
        );

        let wildcard = format!("{}%", channel_name);

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&channel_visability_modifier, Type::INT2)
            .add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_channel_name_) = requery_channel_name {
            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            query = format!("{} AND c.name > ${}", query.as_str(), counter_value);

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_channel_name_, Type::TEXT);
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        query = format!(
            "{} \
            ORDER BY c.name ASC \
            LIMIT ${};",
            query.as_str(),
            counter_value
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name_ = match row.try_get::<'_, usize, String>(1) {
                Ok(channel_name__) => channel_name__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_name__) => channel_name__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(4) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_id_ = match row.try_get::<'_, usize, Option<i64>>(6) {
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

            let channel = Channel1 {
                channel_id,
                channel_name: channel_name_,
                channel_linked_name,
                channel_access_modifier,
                channel_visability_modifier,
                channel_cover_image_path,
                channel_background_image_path,
            };

            let is_application_user_subscribed = match channel_id_ {
                Some(_) => true,
                None => false
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        application_user_id: i64,
        channel_name: &'a str,
        requery_channel_name: &'a Option<String>,
        limit: i16
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let counter_value_1 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip \
            FROM public.channel c INNER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id \
            WHERE c.name LIKE ${}",
            counter_value_1,
            counter_value
        );

        let wildcard = format!("{}%", channel_name);

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_channel_name_) = requery_channel_name {
            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            query = format!("{} AND c.name > ${}", query.as_str(), counter_value);

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_channel_name_, Type::TEXT);
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        query = format!(
            "{} \
            ORDER BY c.name ASC \
            LIMIT ${};",
            query.as_str(),
            counter_value
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name_ = match row.try_get::<'_, usize, String>(1) {
                Ok(channel_name__) => channel_name__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_name__) => channel_name__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_visability_modifier = match row.try_get::<'_, usize, i16>(4) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(6) {
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

            let channel = Channel1 {
                channel_id,
                channel_name: channel_name_,
                channel_linked_name,
                channel_access_modifier,
                channel_visability_modifier,
                channel_cover_image_path,
                channel_background_image_path,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed: true
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        application_user_id: i64,
        requery_channel_id: Option<i64>,
        limit: i16
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip \
            FROM public.channel c INNER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id",
            counter_value
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

        if let Some(ref requery_channel_id_) = requery_channel_id {
            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            query = format!(
                "{} \
                WHERE cs.channel_id > ${}",
                query.as_str(),
                counter_value
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_channel_id_, Type::INT8);
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        query = format!(
            "{} \
            ORDER BY cs.channel_id ASC \
            LIMIT ${};",
            query.as_str(),
            counter_value
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name = match row.try_get::<'_, usize, String>(1) {
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

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_name__) => channel_name__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_visability_modifier = match row.try_get::<'_, usize, i16>(4) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(6) {
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

            let channel = Channel1 {
                channel_id,
                channel_name,
                channel_linked_name,
                channel_access_modifier,
                channel_visability_modifier,
                channel_cover_image_path,
                channel_background_image_path,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed: true
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    // pub async fn request_find_x<'a>(
    //     database_1_connection: &'a Connection,
    //     channel_created_at: &'a Option<String>,
    //     sort_order: SortOrder,
    //     limit: i16
    // ) -> Result<Option<Vec<GetManyByCreatedAtChannel>>, ErrorAuditor> {
    //     todo!();
    //     // let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

    //     // let mut counter = Counter::<i16>::new_classic();

    //     // let mut counter_value: i16;

    //     // let mut query =
    //     //     "SELECT \
    //     //         c.id AS i, \
    //     //         c.name AS n, \
    //     //         c.description AS d, \
    //     //         c.orientation AS o, \
    //     //         c.personalization_image_path AS pip, \
    //     //         c.subscribers_quantity AS sq, \
    //     //         c.marks_quantity AS mq, \
    //     //         c.

    //     //         c.created_at::TEXT AS ca \
    //     //     FROM public.channel c \
    //     //     WHERE c.is_private = FALSE AND c.created_at <= current_timestamp(6)"
    //     //     .to_string();

    //     // if let Some(created_at_) = channel_created_at {
    //     //     match sort_order {
    //     //         SortOrder::Asc => {
    //     //             query += " AND c.created_at > $";
    //     //         }
    //     //         SortOrder::Desc => {
    //     //             query += " AND c.created_at < $";
    //     //         }
    //     //     }

    //     //     counter_value = match counter.get_next_value() {
    //     //         Ok(counter_value_) => counter_value_,
    //     //         Err(mut error) => {
    //     //             error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

    //     //             return Err(error);
    //     //         }
    //     //     };

    //     //     query = query + counter_value.to_string().as_str() + "::TIMESTAMP(6) WITH TIME ZONE";

    //     //     prepared_statemant_parameter_convertation_resolver.add_parameter(created_at_, Type::TEXT);
    //     // }

    //     // counter_value = match counter.get_next_value() {
    //     //     Ok(counter_value_) => counter_value_,
    //     //     Err(mut error) => {
    //     //         error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

    //     //         return Err(error);
    //     //     }
    //     // };
    //     // query = query + " ORDER BY c.created_at " + sort_order.convert() + " LIMIT $" + counter_value.to_string().as_str() + ";";

    //     // prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

    //     // let mut channel_registry: Vec<GetManyByCreatedAtChannel> = vec![];

    //     // let statement = match database_1_connection.prepare_typed(
    //     //     query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
    //     // ).await {
    //     //     Ok(statement_) => statement_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // let row_registry = match database_1_connection.query(
    //     //     &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
    //     // ).await {
    //     //     Ok(row_registry_) => row_registry_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // if row_registry.is_empty() {
    //     //     return Ok(None);
    //     // }

    //     // '_a: for row in row_registry.iter() {
    //     //     let channel_id = match row.try_get::<'_, usize, i64>(0) {
    //     //         Ok(channel_id_) => channel_id_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_name = match row.try_get::<'_, usize, String>(1) {
    //     //         Ok(channel_name_) => channel_name_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_personalization_image_path = match row.try_get::<'_, usize, String>(2) {
    //     //         Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_subscribers_quantity = match row.try_get::<'_, usize, i64>(3) {
    //     //         Ok(channel_subscribers_quantity_) => channel_subscribers_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_public_marks_quantity = match row.try_get::<'_, usize, i64>(4) {
    //     //         Ok(channel_public_marks_quantity_) => channel_public_marks_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_hidden_marks_quantity = match row.try_get::<'_, usize, i64>(5) {
    //     //         Ok(channel_hidden_marks_quantity_) => channel_hidden_marks_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_reactions_quantity = match row.try_get::<'_, usize, i64>(6) {
    //     //         Ok(channel_reactions_quantity_) => channel_reactions_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_viewing_quantity = match row.try_get::<'_, usize, i64>(7) {
    //     //         Ok(channel_viewing_quantity_) => channel_viewing_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_created_at_ = match row.try_get::<'_, usize, String>(8) {
    //     //         Ok(channel_created_at__) => channel_created_at__,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel = GetManyByCreatedAtChannel {
    //     //         channel_id,
    //     //         channel_name,
    //     //         channel_personalization_image_path,
    //     //         channel_subscribers_quantity,
    //     //         channel_public_marks_quantity,
    //     //         channel_hidden_marks_quantity,
    //     //         channel_reactions_quantity,
    //     //         channel_viewing_quantity,
    //     //         channel_created_at: channel_created_at_
    //     //     };

    //     //     channel_registry.push(channel);
    //     // }

    //     // return Ok(Some(channel_registry));
    // }

    // pub async fn request_find_xx<'a>(
    //     database_1_connection: &'a Connection,
    //     channel_subscribers_quantity: Option<i64>,
    //     sort_order: SortOrder,
    //     limit: i16
    // ) -> Result<Option<Vec<GetManyBySubscribersQuantityChannel>>, ErrorAuditor> {
    //     todo!();
    //     // let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

    //     // let mut counter = Counter::<i16>::new_classic();

    //     // let mut counter_value: i16;

    //     // let mut query =
    //     //     "SELECT \
    //     //         c.id AS i, \
    //     //         c.subscribers_quantity AS sq \
    //     //     FROM public.channel c \
    //     //     WHERE c.is_private = FALSE"
    //     //     .to_string();

    //     // if let Some(ref subscribers_quantity_) = channel_subscribers_quantity {
    //     //     match sort_order {
    //     //         SortOrder::Asc => {
    //     //             query += " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) > public.limit_channel_subscribers_quantity($";
    //     //         }
    //     //         SortOrder::Desc => {
    //     //             query += " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) < public.limit_channel_subscribers_quantity($";
    //     //         }
    //     //     }

    //     //     counter_value = match counter.get_next_value() {
    //     //         Ok(counter_value_) => counter_value_,
    //     //         Err(mut error) => {
    //     //             error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

    //     //             return Err(error);
    //     //         }
    //     //     };

    //     //     query = query + counter_value.to_string().as_str() + ")";

    //     //     prepared_statemant_parameter_convertation_resolver.add_parameter(subscribers_quantity_, Type::INT8);
    //     // }

    //     // counter_value = match counter.get_next_value() {
    //     //     Ok(counter_value_) => counter_value_,
    //     //     Err(mut error) => {
    //     //         error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

    //     //         return Err(error);
    //     //     }
    //     // };
    //     // query = query + " ORDER BY public.limit_channel_subscribers_quantity(c.subscribers_quantity) " + sort_order.convert() +" LIMIT $" + counter_value.to_string().as_str() + ";";

    //     // prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

    //     // let mut channel_registry: Vec<GetManyBySubscribersQuantityChannel> = vec![];

    //     // let statement = match database_1_connection.prepare_typed(
    //     //     query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
    //     // ).await {
    //     //     Ok(statement_) => statement_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // let row_registry = match database_1_connection.query(
    //     //     &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
    //     // ).await {
    //     //     Ok(row_registry_) => row_registry_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // if row_registry.is_empty() {
    //     //     return Ok(None);
    //     // }

    //     // '_a: for row in row_registry.iter() {
    //     //     let channel_id = match row.try_get::<'_, usize, i64>(0) {
    //     //         Ok(channel_id_) => channel_id_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_subscribers_quantity_ = match row.try_get::<'_, usize, i64>(1) {
    //     //         Ok(channel_subscribers_quantity__) => channel_subscribers_quantity__,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel = GetManyBySubscribersQuantityChannel {
    //     //         channel_id,
    //     //         channel_subscribers_quantity: channel_subscribers_quantity_
    //     //     };

    //     //     channel_registry.push(channel);
    //     // }

    //     // return Ok(Some(channel_registry));
    // }

    // pub async fn request_find_xxx<'a>(
    //     database_1_connection: &'a Connection,
    //     id_registry: &'a Vec<i64>
    // ) -> Result<Option<Vec<GetManyByIdRegistryChannel>>, ErrorAuditor> {
    //     todo!();
    //     // if id_registry.is_empty() {
    //     //     return Ok(None)
    //     // }

    //     // let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

    //     // let query =
    //     //     "SELECT \
    //     //         c.id AS i, \
    //     //         c.name AS n, \
    //     //         c.personalization_image_path AS pip, \
    //     //         c.subscribers_quantity AS sq, \
    //     //         c.public_marks_quantity AS pmq, \
    //     //         c.hidden_marks_quantity AS hmq, \
    //     //         c.reactions_quantity AS rq, \
    //     //         c.viewing_quantity AS vq, \
    //     //         c.created_at::TEXT AS ca \
    //     //     FROM public.channel c \
    //     //     WHERE c.is_private = FALSE AND c.id = ANY($1);";

    //     // prepared_statemant_parameter_convertation_resolver.add_parameter(&id_registry, Type::INT8_ARRAY);

    //     // let mut channel_registry: Vec<GetManyByIdRegistryChannel> = vec![];

    //     // let statement = match database_1_connection.prepare_typed(
    //     //     query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
    //     // ).await {
    //     //     Ok(statement_) => statement_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // let row_registry = match database_1_connection.query(
    //     //     &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
    //     // ).await {
    //     //     Ok(row_registry_) => row_registry_,
    //     //     Err(error) => {
    //     //         return Err(
    //     //             ErrorAuditor::new(
    //     //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                 BacktracePart::new(line!(), file!(), None)
    //     //             )
    //     //         );
    //     //     }
    //     // };

    //     // if row_registry.is_empty() {
    //     //     return Ok(None);
    //     // }

    //     // '_a: for row in row_registry.iter() {
    //     //     let channel_id = match row.try_get::<'_, usize, i64>(0) {
    //     //         Ok(channel_id_) => channel_id_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_name = match row.try_get::<'_, usize, String>(1) {
    //     //         Ok(channel_name_) => channel_name_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_personalization_image_path =  match row.try_get::<'_, usize, String>(2) {
    //     //         Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_subscribers_quantity = match row.try_get::<'_, usize, i64>(3) {
    //     //         Ok(channel_subscribers_quantity_) => channel_subscribers_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_public_marks_quantity = match row.try_get::<'_, usize, i64>(4) {
    //     //         Ok(channel_public_marks_quantity_) => channel_public_marks_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_hidden_marks_quantity = match row.try_get::<'_, usize, i64>(5) {
    //     //         Ok(channel_hidden_marks_quantity_) => channel_hidden_marks_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_reactions_quantity = match row.try_get::<'_, usize, i64>(6) {
    //     //         Ok(channel_reactions_quantity_) => channel_reactions_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_viewing_quantity = match row.try_get::<'_, usize, i64>(7) {
    //     //         Ok(channel_viewing_quantity_) => channel_viewing_quantity_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel_created_at: String = match row.try_get::<'_, usize, String>(8) {
    //     //         Ok(channel_created_at_) => channel_created_at_,
    //     //         Err(error) => {
    //     //             return Err(
    //     //                 ErrorAuditor::new(
    //     //                     BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
    //     //                     BacktracePart::new(line!(), file!(), None)
    //     //                 )
    //     //             );
    //     //         }
    //     //     };

    //     //     let channel = GetManyByIdRegistryChannel {
    //     //         channel_id,
    //     //         channel_name,
    //     //         channel_personalization_image_path,
    //     //         channel_subscribers_quantity,
    //     //         channel_public_marks_quantity,
    //     //         channel_hidden_marks_quantity,
    //     //         channel_reactions_quantity,
    //     //         channel_viewing_quantity,
    //     //         channel_created_at
    //     //     };

    //     //     channel_registry.push(channel);
    //     // }

    //     // return Ok(Some(channel_registry));
    // }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool
}