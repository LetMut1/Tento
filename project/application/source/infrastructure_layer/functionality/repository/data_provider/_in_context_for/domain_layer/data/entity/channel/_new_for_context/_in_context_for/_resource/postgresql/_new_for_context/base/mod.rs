use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_new_for_context::base::_component::channel::Channel as ActionHandlerOutcomingDataGetManyByCreatedAtChannel;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::_new_for_context::base::_component::channel::Channel as ActionHandlerOutcomingDataGetManyByIdRegistryChannel;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_new_for_context::base::_component::channel::Channel as ActionHandlerOutcomingDataGetManyByNameChannel;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_new_for_context::base::_component::channel::Channel as ActionHandlerOutcomingDataGetManyBySubscribersQuantityChannel;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use crate::infrastructure_layer::functionality::service::counter_u8::CounterU8;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;    // TODO  TODO  TODO  TODO  TODO  Имена ПрепСТейтентов, их отмена - нужно ли это все? TODO  TODO  TODO 
                    // TODO !!!!!!!1  TODO  TODO  TODO  TODO  Если извне оборачивать в транзакцию, что будет с декларирование подготовленного запроса? То есть: Бегин- создать препэрэд стэйстмент - иполнить пр ст- коммит/роллбэу
impl Base {
    pub async fn per_request_1<'a>(
        core_connection: &'a Connection,
        name: &'a str,
        requery_name: &'a Option<String>,
        limit: i16
    ) -> Result<Option<Vec<ActionHandlerOutcomingDataGetManyByNameChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
        
        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;
        match counter_u8.get_next() {
            Ok(counter_) => {
                counter_u8_value = counter_;
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
            + counter_u8_value.to_string().as_str();

        let wildcard = name.to_string() + "%";
        prepared_statemant_parameter_convertation_resolver.add_parameter(&wildcard, Type::TEXT);

        if let Some(requery_name_) = requery_name {
            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }
            query = query + " AND c.name > $" + counter_u8_value.to_string().as_str();

            prepared_statemant_parameter_convertation_resolver.add_parameter(requery_name_, Type::TEXT);
        }

        match counter_u8.get_next() {
            Ok(counter_) => {
                counter_u8_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY c.name ASC LIMIT $" + counter_u8_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<ActionHandlerOutcomingDataGetManyByNameChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id: i64;
                                match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => {
                                        channel_id = channel_id_;
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

                                let channel_name: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_name_) => {
                                        channel_name = channel_name_;
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

                                let channel_personalization_image_path: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => {
                                        channel_personalization_image_path = channel_personalization_image_path_;
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

                                let channel_subscribers_quantity: i64;
                                match row.try_get::<'_, usize, i64>(3) {
                                    Ok(channel_subscribers_quantity_) => {
                                        channel_subscribers_quantity = channel_subscribers_quantity_;
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

                                let channel_public_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(4) {
                                    Ok(channel_public_marks_quantity_) => {
                                        channel_public_marks_quantity = channel_public_marks_quantity_;
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

                                let channel_hidden_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(5) {
                                    Ok(channel_hidden_marks_quantity_) => {
                                        channel_hidden_marks_quantity = channel_hidden_marks_quantity_;
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

                                let channel_reactions_quantity: i64;
                                match row.try_get::<'_, usize, i64>(6) {
                                    Ok(channel_reactions_quantity_) => {
                                        channel_reactions_quantity = channel_reactions_quantity_;
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

                                let channel_viewing_quantity: i64;
                                match row.try_get::<'_, usize, i64>(7) {
                                    Ok(channel_viewing_quantity_) => {
                                        channel_viewing_quantity = channel_viewing_quantity_;
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

                                let channel_created_at: String;
                                match row.try_get::<'_, usize, String>(8) {
                                    Ok(channel_created_at_) => {
                                        channel_created_at = channel_created_at_;
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

                                let channel = ActionHandlerOutcomingDataGetManyByNameChannel::new(
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

    pub async fn per_request_2<'a>(
        core_connection: &'a Connection,
        created_at: &'a Option<String>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<ActionHandlerOutcomingDataGetManyByCreatedAtChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

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

        if let Some(created_at_) = created_at {
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
            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }
            query = query + counter_u8_value.to_string().as_str() + "::TIMESTAMP(6) WITH TIME ZONE";

            prepared_statemant_parameter_convertation_resolver.add_parameter(created_at_, Type::TEXT);
        }

        let order_: &'static str;
        match counter_u8.get_next() {
            Ok(counter_) => {
                counter_u8_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        match OrderConventionResolver::convert(order) {
            Ok(order__) => {
                order_ = order__;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY c.created_at " + order_ + " LIMIT $" + counter_u8_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<ActionHandlerOutcomingDataGetManyByCreatedAtChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id: i64;
                                match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => {
                                        channel_id = channel_id_;
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

                                let channel_name: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_name_) => {
                                        channel_name = channel_name_;
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

                                let channel_personalization_image_path: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => {
                                        channel_personalization_image_path = channel_personalization_image_path_;
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

                                let channel_subscribers_quantity: i64;
                                match row.try_get::<'_, usize, i64>(3) {
                                    Ok(channel_subscribers_quantity_) => {
                                        channel_subscribers_quantity = channel_subscribers_quantity_;
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

                                let channel_public_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(4) {
                                    Ok(channel_public_marks_quantity_) => {
                                        channel_public_marks_quantity = channel_public_marks_quantity_;
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

                                let channel_hidden_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(5) {
                                    Ok(channel_hidden_marks_quantity_) => {
                                        channel_hidden_marks_quantity = channel_hidden_marks_quantity_;
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

                                let channel_reactions_quantity: i64;
                                match row.try_get::<'_, usize, i64>(6) {
                                    Ok(channel_reactions_quantity_) => {
                                        channel_reactions_quantity = channel_reactions_quantity_;
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

                                let channel_viewing_quantity: i64;
                                match row.try_get::<'_, usize, i64>(7) {
                                    Ok(channel_viewing_quantity_) => {
                                        channel_viewing_quantity = channel_viewing_quantity_;
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

                                let channel_created_at: String;
                                match row.try_get::<'_, usize, String>(8) {
                                    Ok(channel_created_at_) => {
                                        channel_created_at = channel_created_at_;
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

                                let channel = ActionHandlerOutcomingDataGetManyByCreatedAtChannel::new(
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

    pub async fn per_request_3<'a>(
        core_connection: &'a Connection,
        subscribers_quantity: Option<i64>,
        order: i8,
        limit: i16
    ) -> Result<Option<Vec<ActionHandlerOutcomingDataGetManyBySubscribersQuantityChannel>>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter_u8 = CounterU8::new();

        let mut counter_u8_value: u8;

        let mut query = 
            "SELECT \
                c.id AS i, \
                c.subscribers_quantity AS sq \
            FROM public.channel c \
            WHERE c.is_private = FALSE"
            .to_string();

        if let Some(ref subscribers_quantity_) = subscribers_quantity {
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
            match counter_u8.get_next() {
                Ok(counter_) => {
                    counter_u8_value = counter_;
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }
            query = query + counter_u8_value.to_string().as_str() + ")";

            prepared_statemant_parameter_convertation_resolver.add_parameter(subscribers_quantity_, Type::INT8);
        }

        let order_: &'static str;
        match counter_u8.get_next() {
            Ok(counter_) => {
                counter_u8_value = counter_;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        match OrderConventionResolver::convert(order) {
            Ok(order__) => {
                order_ = order__;
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
        query = query + " ORDER BY public.limit_channel_subscribers_quantity(c.subscribers_quantity) " + order_ +" LIMIT $" + counter_u8_value.to_string().as_str() + ";";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&limit, Type::INT2);

        let mut channel_registry: Vec<ActionHandlerOutcomingDataGetManyBySubscribersQuantityChannel> = vec![];

        match core_connection.prepare_typed(query.as_str(), prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id: i64;
                                match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => {
                                        channel_id = channel_id_;
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

                                let channel_subscribers_quantity: i64;
                                match row.try_get::<'_, usize, i64>(1) {
                                    Ok(channel_subscribers_quantity_) => {
                                        channel_subscribers_quantity = channel_subscribers_quantity_;
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

                                let channel = ActionHandlerOutcomingDataGetManyBySubscribersQuantityChannel::new(
                                    channel_id,
                                    channel_subscribers_quantity
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
    ) -> Result<Option<Vec<ActionHandlerOutcomingDataGetManyByIdRegistryChannel>>, ErrorAuditor> {
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

        let mut channel_registry: Vec<ActionHandlerOutcomingDataGetManyByIdRegistryChannel> = vec![];

        match core_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            '_a: for row in row_registry.iter() {
                                let channel_id: i64;
                                match row.try_get::<'_, usize, i64>(0) {
                                    Ok(channel_id_) => {
                                        channel_id = channel_id_;
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

                                let channel_name: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_name_) => {
                                        channel_name = channel_name_;
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

                                let channel_personalization_image_path: String;
                                match row.try_get::<'_, usize, String>(1) {
                                    Ok(channel_personalization_image_path_) => {
                                        channel_personalization_image_path = channel_personalization_image_path_;
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

                                let channel_subscribers_quantity: i64;
                                match row.try_get::<'_, usize, i64>(3) {
                                    Ok(channel_subscribers_quantity_) => {
                                        channel_subscribers_quantity = channel_subscribers_quantity_;
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

                                let channel_public_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(4) {
                                    Ok(channel_public_marks_quantity_) => {
                                        channel_public_marks_quantity = channel_public_marks_quantity_;
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

                                let channel_hidden_marks_quantity: i64;
                                match row.try_get::<'_, usize, i64>(5) {
                                    Ok(channel_hidden_marks_quantity_) => {
                                        channel_hidden_marks_quantity = channel_hidden_marks_quantity_;
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

                                let channel_reactions_quantity: i64;
                                match row.try_get::<'_, usize, i64>(6) {
                                    Ok(channel_reactions_quantity_) => {
                                        channel_reactions_quantity = channel_reactions_quantity_;
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

                                let channel_viewing_quantity: i64;
                                match row.try_get::<'_, usize, i64>(7) {
                                    Ok(channel_viewing_quantity_) => {
                                        channel_viewing_quantity = channel_viewing_quantity_;
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

                                let channel_created_at: String;
                                match row.try_get::<'_, usize, String>(8) {
                                    Ok(channel_created_at_) => {
                                        channel_created_at = channel_created_at_;
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

                                let channel = ActionHandlerOutcomingDataGetManyByIdRegistryChannel::new(
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

