use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_created_at::Channel as GetManyByCreatedAtChannel;
use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_id_registry::Channel as GetManyByIdRegistryChannel;
use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_name::Channel as GetManyByNameChannel;
use crate::application_layer::functionality::service::action_processor::channel__base::get_many_by_subscribers_quantity::Channel as GetManyBySubscribersQuantityChannel;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::counter::Counter;
use crate::infrastructure_layer::functionality::service::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

pub struct Channel_PostgresqlRepository;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO
                    // TODO !!!!!!!1  TODO  TODO  TODO  TODO  Если извне оборачивать в транзакцию, что будет с декларирование подготовленного запроса? То есть: Бегин- создать препэрэд стэйстмент - иполнить пр ст- коммит/роллбэу
impl Channel_PostgresqlRepository {
    pub async fn per_request_1<'a>(                     //  TODO подумать, как это называть. Когда возвращается сущность, это называется find_number
        database_1_connection: &'a Connection,
        channel_name: &'a str,
        requery_name: &'a Option<String>,
        limit: i16
    ) -> Result<Option<Vec<GetManyByNameChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new_classic();

        let mut counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut query =
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
            + counter_value.to_string().as_str();

        let wildcard = channel_name.to_string() + "%";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_name_) = requery_name {
            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            query = query + " AND c.name > $" + counter_value.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_name_, Type::TEXT);
        }

        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        query = query + " ORDER BY c.name ASC LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<GetManyByNameChannel> = vec![];

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
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
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
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

            let channel_personalization_image_path = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_subscribers_quantity = match row.try_get::<'_, usize, i64>(3) {
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

            let channel_public_marks_quantity = match row.try_get::<'_, usize, i64>(4) {
                Ok(channel_public_marks_quantity_) => channel_public_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_hidden_marks_quantity = match row.try_get::<'_, usize, i64>(5) {
                Ok(channel_hidden_marks_quantity_) => channel_hidden_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_reactions_quantity = match row.try_get::<'_, usize, i64>(6) {
                Ok(channel_reactions_quantity_) => channel_reactions_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_viewing_quantity = match row.try_get::<'_, usize, i64>(7) {
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

            let channel_created_at = match row.try_get::<'_, usize, String>(8) {
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

            let channel = GetManyByNameChannel {
                channel_id,
                channel_name: channel_name_,
                channel_personalization_image_path,
                channel_subscribers_quantity,
                channel_public_marks_quantity,
                channel_hidden_marks_quantity,
                channel_reactions_quantity,
                channel_viewing_quantity,
                channel_created_at
            };

            channel_registry.push(channel);
        }

        return Ok(Some(channel_registry));
    }

    pub async fn per_request_2<'a>(
        database_1_connection: &'a Connection,
        channel_created_at: &'a Option<String>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<GetManyByCreatedAtChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new_classic();

        let mut counter_value: i16;

        let mut query =
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

        if let Some(created_at_) = channel_created_at {
            if OrderConventionResolver::is_asc(order) {
                query += " AND c.created_at > $";
            } else {
                query += " AND c.created_at < $";
            }

            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            query = query + counter_value.to_string().as_str() + "::TIMESTAMP(6) WITH TIME ZONE";

            prepared_statemant_parameter_convertation_resolver.add_parameter(created_at_, Type::TEXT);
        }

        let order_ = match OrderConventionResolver::convert(order) {
            Ok(order__) => order__,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        query = query + " ORDER BY c.created_at " + order_ + " LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<GetManyByCreatedAtChannel> = vec![];

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
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
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
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

            let channel_personalization_image_path = match row.try_get::<'_, usize, String>(2) {
                Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_subscribers_quantity = match row.try_get::<'_, usize, i64>(3) {
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

            let channel_public_marks_quantity = match row.try_get::<'_, usize, i64>(4) {
                Ok(channel_public_marks_quantity_) => channel_public_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_hidden_marks_quantity = match row.try_get::<'_, usize, i64>(5) {
                Ok(channel_hidden_marks_quantity_) => channel_hidden_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_reactions_quantity = match row.try_get::<'_, usize, i64>(6) {
                Ok(channel_reactions_quantity_) => channel_reactions_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_viewing_quantity = match row.try_get::<'_, usize, i64>(7) {
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

            let channel_created_at_ = match row.try_get::<'_, usize, String>(8) {
                Ok(channel_created_at__) => channel_created_at__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel = GetManyByCreatedAtChannel {
                channel_id,
                channel_name,
                channel_personalization_image_path,
                channel_subscribers_quantity,
                channel_public_marks_quantity,
                channel_hidden_marks_quantity,
                channel_reactions_quantity,
                channel_viewing_quantity,
                channel_created_at: channel_created_at_
            };

            channel_registry.push(channel);
        }

        return Ok(Some(channel_registry));
    }

    pub async fn per_request_3<'a>(
        database_1_connection: &'a Connection,
        channel_subscribers_quantity: Option<i64>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<GetManyBySubscribersQuantityChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new_classic();

        let mut counter_value: i16;

        let mut query =
            "SELECT \
                c.id AS i, \
                c.subscribers_quantity AS sq \
            FROM public.channel c \
            WHERE c.is_private = FALSE"
            .to_string();

        if let Some(ref subscribers_quantity_) = channel_subscribers_quantity {
            if OrderConventionResolver::is_asc(order) {
                query += " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) > public.limit_channel_subscribers_quantity($";
            } else {
                query += " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) < public.limit_channel_subscribers_quantity($";
            }

            counter_value = match counter.get_next_value() {
                Ok(counter_value_) => counter_value_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            query = query + counter_value.to_string().as_str() + ")";

            prepared_statemant_parameter_convertation_resolver.add_parameter(subscribers_quantity_, Type::INT8);
        }

        let order_ = match OrderConventionResolver::convert(order) {
            Ok(order__) => order__,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        counter_value = match counter.get_next_value() {
            Ok(counter_value_) => counter_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        query = query + " ORDER BY public.limit_channel_subscribers_quantity(c.subscribers_quantity) " + order_ +" LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<GetManyBySubscribersQuantityChannel> = vec![];

        let statement = match database_1_connection.prepare_typed(
            query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
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
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
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

            let channel_subscribers_quantity_ = match row.try_get::<'_, usize, i64>(1) {
                Ok(channel_subscribers_quantity__) => channel_subscribers_quantity__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel = GetManyBySubscribersQuantityChannel {
                channel_id,
                channel_subscribers_quantity: channel_subscribers_quantity_
            };

            channel_registry.push(channel);
        }

        return Ok(Some(channel_registry));
    }

    pub async fn per_request_4<'a>(
        database_1_connection: &'a Connection,
        id_registry: &'a Vec<i64>
    ) -> Result<Option<Vec<GetManyByIdRegistryChannel>>, ErrorAuditor> {
        if id_registry.is_empty() {
            return Ok(None)
        }

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
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

        let mut channel_registry: Vec<GetManyByIdRegistryChannel> = vec![];

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
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
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
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

            let channel_personalization_image_path =  match row.try_get::<'_, usize, String>(2) {
                Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_subscribers_quantity = match row.try_get::<'_, usize, i64>(3) {
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

            let channel_public_marks_quantity = match row.try_get::<'_, usize, i64>(4) {
                Ok(channel_public_marks_quantity_) => channel_public_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_hidden_marks_quantity = match row.try_get::<'_, usize, i64>(5) {
                Ok(channel_hidden_marks_quantity_) => channel_hidden_marks_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_reactions_quantity = match row.try_get::<'_, usize, i64>(6) {
                Ok(channel_reactions_quantity_) => channel_reactions_quantity_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let channel_viewing_quantity = match row.try_get::<'_, usize, i64>(7) {
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

            let channel_created_at: String = match row.try_get::<'_, usize, String>(8) {
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

            let channel = GetManyByIdRegistryChannel {
                channel_id,
                channel_name,
                channel_personalization_image_path,
                channel_subscribers_quantity,
                channel_public_marks_quantity,
                channel_hidden_marks_quantity,
                channel_reactions_quantity,
                channel_viewing_quantity,
                channel_created_at
            };

            channel_registry.push(channel);
        }

        return Ok(Some(channel_registry));
    }
}
