use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Channel as OutcomingGetManyByCreatedAtChannel;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Channel as OutcomingGetManyByIdRegistryChannel;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Channel as OutcomingGetManyByNameChannel;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Channel as OutcomingGetManyBySubscribersQuantityChannel;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::counter::Counter;
use crate::infrastructure_layer::functionality::service::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct Channel_PostgresqlRepository;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO
                    // TODO !!!!!!!1  TODO  TODO  TODO  TODO  Если извне оборачивать в транзакцию, что будет с декларирование подготовленного запроса? То есть: Бегин- создать препэрэд стэйстмент - иполнить пр ст- коммит/роллбэу
impl Channel_PostgresqlRepository {
    pub async fn per_request_1<'a>(                     //  TODO подумать, как это называть. Когда возвращается сущность, это называется find_number
        core_connection: &'a Connection,
        channel_name: &'a str,
        requery_name: &'a Option<String>,
        limit: i16
    ) -> Result<Option<Vec<OutcomingGetManyByNameChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new();

        let mut counter_value: usize;
        match counter.get_next() {
            Ok(counter_) => {
                counter_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

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
            match counter.get_next() {
                Ok(counter_) => {
                    counter_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }
            query = query + " AND c.name > $" + counter_value.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_name_, Type::TEXT);
        }

        match counter.get_next() {
            Ok(counter_) => {
                counter_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY c.name ASC LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<OutcomingGetManyByNameChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id = match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => channel_id_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel_personalization_image_path = match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel = OutcomingGetManyByNameChannel::new(
                                    channel_id,
                                    channel_name_,
                                    channel_personalization_image_path,
                                    channel_subscribers_quantity,
                                    channel_public_marks_quantity,
                                    channel_hidden_marks_quantity,
                                    channel_reactions_quantity,
                                    channel_viewing_quantity,
                                    channel_created_at
                                );

                                channel_registry.push(channel);
                            }

                            return Ok(Some(channel_registry));
                        }

                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn per_request_2<'a>(
        core_connection: &'a Connection,
        channel_created_at: &'a Option<String>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<OutcomingGetManyByCreatedAtChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new();

        let mut counter_value: usize;

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
                if OrderConventionResolver::is_desc(order) {
                    query += " AND c.created_at < $";
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::InvalidArgumentError,
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
            match counter.get_next() {
                Ok(counter_) => {
                    counter_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }
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
        match counter.get_next() {
            Ok(counter_) => {
                counter_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY c.created_at " + order_ + " LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<OutcomingGetManyByCreatedAtChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id = match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => channel_id_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel_personalization_image_path = match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel = OutcomingGetManyByCreatedAtChannel::new(
                                    channel_id,
                                    channel_name,
                                    channel_personalization_image_path,
                                    channel_subscribers_quantity,
                                    channel_public_marks_quantity,
                                    channel_hidden_marks_quantity,
                                    channel_reactions_quantity,
                                    channel_viewing_quantity,
                                    channel_created_at_
                                );

                                channel_registry.push(channel);
                            }

                            return Ok(Some(channel_registry));
                        }

                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn per_request_3<'a>(
        core_connection: &'a Connection,
        channel_subscribers_quantity: Option<i64>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<OutcomingGetManyBySubscribersQuantityChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::new();

        let mut counter_value: usize;

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
                if OrderConventionResolver::is_desc(order) {
                    query += " AND public.limit_channel_subscribers_quantity(c.subscribers_quantity) < public.limit_channel_subscribers_quantity($";
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::InvalidArgumentError,
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
            match counter.get_next() {
                Ok(counter_) => {
                    counter_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }
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
        match counter.get_next() {
            Ok(counter_) => {
                counter_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY public.limit_channel_subscribers_quantity(c.subscribers_quantity) " + order_ +" LIMIT $" + counter_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<OutcomingGetManyBySubscribersQuantityChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id = match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => channel_id_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel = OutcomingGetManyBySubscribersQuantityChannel::new(
                                    channel_id,
                                    channel_subscribers_quantity_
                                );

                                channel_registry.push(channel);
                            }

                            return Ok(Some(channel_registry));
                        }

                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn per_request_4<'a>(
        core_connection: &'a Connection,
        id_registry: &'a Vec<i64>
    ) -> Result<Option<Vec<OutcomingGetManyByIdRegistryChannel>>, ErrorAuditor> {
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

        let mut channel_registry: Vec<OutcomingGetManyByIdRegistryChannel> = vec![];

        match core_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id = match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => channel_id_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel_personalization_image_path =  match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => channel_personalization_image_path_,
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                };

                                let channel = OutcomingGetManyByIdRegistryChannel::new(
                                    channel_id,
                                    channel_name,
                                    channel_personalization_image_path,
                                    channel_subscribers_quantity,
                                    channel_public_marks_quantity,
                                    channel_hidden_marks_quantity,
                                    channel_reactions_quantity,
                                    channel_viewing_quantity,
                                    channel_created_at
                                );

                                channel_registry.push(channel);
                            }

                            return Ok(Some(channel_registry));
                        }

                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}
