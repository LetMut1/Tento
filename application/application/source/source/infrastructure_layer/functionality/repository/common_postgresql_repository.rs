use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_BackgroundImagePath;
use crate::domain_layer::data::entity::channel::Channel_CoverImagePath;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use super::postgresql_repository::by::By11;
use super::postgresql_repository::by::By12;
use super::postgresql_repository::by::By13;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::channel___postgresql_repository::Channel1;
use crate::infrastructure_layer::functionality::service::counter::Counter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;

#[cfg(feature = "manual_testing")]
use extern_crate::serde::Deserialize;

impl PostgresqlRepository<Common1> {
    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_11: &'a By11<'_>,
        limit: i16,
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let application_user_id = by_11.application_user_id.get();

        let channel_name = by_11.channel_name.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let counter_value_1 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let counter_value_2 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let channel_visability_modifier = by_11.channel_visability_modifier.get();

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
            counter_value_1, counter_value_2, counter_value
        );

        let wildcard = format!(
            "{}%",
            channel_name
        );

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &channel_visability_modifier,
                Type::INT2,
            )
            .add_parameter(
                &wildcard,
                Type::TEXT,
            );

        let requery_channel_name: &'_ str;

        if let Some(requery_channel_name_) = by_11.requery_channel_name {
            requery_channel_name = requery_channel_name_.get();

            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            query = format!(
                "{} AND c.name > ${}",
                query.as_str(),
                counter_value
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_name,
                Type::TEXT,
            );
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = match database_1_connection
            .prepare_typed(
                query.as_str(),
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name_ = match row.try_get::<'_, usize, String>(1) {
                Ok(channel_name__) => Channel_Name::new(channel_name__),
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

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_name__) => Channel_LinkedName::new(channel_name__),
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

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(4) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_id_ = match row.try_get::<'_, usize, Option<i64>>(6) {
                Ok(channel_id_) => channel_id_,
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

            let channel = Channel1 {
                channel_id,
                channel_name: channel_name_,
                channel_linked_name,
                channel_access_modifier,
                channel_visability_modifier: by_11.channel_visability_modifier,
                channel_cover_image_path,
                channel_background_image_path,
            };

            let is_application_user_subscribed = match channel_id_ {
                Some(_) => true,
                None => false,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        by_12: &'a By12<'_>,
        limit: i16,
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let application_user_id = by_12.application_user_id.get();

        let channel_name = by_12.channel_name.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let counter_value_1 = counter_value;

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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
            counter_value_1, counter_value
        );

        let wildcard = format!(
            "{}%",
            channel_name
        );

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &wildcard,
                Type::TEXT,
            );

        let requery_channel_name: &'_ str;

        if let Some(requery_channel_name_) = by_12.requery_channel_name {
            requery_channel_name = requery_channel_name_.get();

            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            query = format!(
                "{} AND c.name > ${}",
                query.as_str(),
                counter_value
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_name,
                Type::TEXT,
            );
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = match database_1_connection
            .prepare_typed(
                query.as_str(),
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name_ = match row.try_get::<'_, usize, String>(1) {
                Ok(channel_name__) => Channel_Name::new(channel_name__),
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

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_name__) => Channel_LinkedName::new(channel_name__),
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

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_visability_modifier = match row.try_get::<'_, usize, i16>(4) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(6) {
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
                is_application_user_subscribed: true,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        by_13: &'a By13,
        limit: i16,
    ) -> Result<Vec<Common1>, ErrorAuditor> {
        let application_user_id = by_13.application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &application_user_id,
            Type::INT8,
        );

        let requery_channel_id: i64;

        if let Some(requery_channel_id_) = by_13.requery_channel_id {
            requery_channel_id = requery_channel_id_.get();

            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            query = format!(
                "{} \
                WHERE cs.channel_id > ${}",
                query.as_str(),
                counter_value
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_id,
                Type::INT8,
            );
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = match database_1_connection
            .prepare_typed(
                query.as_str(),
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

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_id = match row.try_get::<'_, usize, i64>(0) {
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

            let channel_name = match row.try_get::<'_, usize, String>(1) {
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

            let channel_linked_name = match row.try_get::<'_, usize, String>(2) {
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

            let channel_access_modifier = match row.try_get::<'_, usize, i16>(3) {
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

            let channel_visability_modifier = match row.try_get::<'_, usize, i16>(4) {
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

            let channel_cover_image_path = match row.try_get::<'_, usize, Option<String>>(5) {
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

            let channel_background_image_path = match row.try_get::<'_, usize, Option<String>>(6) {
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
                is_application_user_subscribed: true,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool,
}
