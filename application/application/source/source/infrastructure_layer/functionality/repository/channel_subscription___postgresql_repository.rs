use super::postgresql_repository::by::By10;
use super::postgresql_repository::insert::Insert10;
use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription_CreatedAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ChannelSubscription> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_10: Insert10,
    ) -> Result<ChannelSubscription, ErrorAuditor_> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.channel_subscription AS cs ( \
                application_user_id, \
                channel_id, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                current_timestamp(6) \
            ) \
            RETURNING \
                cs.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &insert_10.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_10.channel_id.0,
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
                    ErrorAuditor_::new(
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
                    ErrorAuditor_::new(
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

        let channel_subscription_created_at = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(channel_subscription_created_at_) => ChannelSubscription_CreatedAt(channel_subscription_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
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

        let channel_subscription = ChannelSubscription {
            application_user_id: insert_10.application_user_id,
            channel_id: insert_10.channel_id,
            created_at: channel_subscription_created_at,
        };

        return Ok(channel_subscription);
    }

    pub async fn is_exist_1<'a>(
        database_1_connection: &'a Connection,
        by_10: &'a By10,
    ) -> Result<bool, ErrorAuditor_> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                cs.application_user_id AS aui \
            FROM public.channel_subscription cs \
            WHERE cs.application_user_id = $1 AND cs.channel_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_10.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &by_10.channel_id.0,
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
                    ErrorAuditor_::new(
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
                    ErrorAuditor_::new(
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
            return Ok(false);
        }

        return Ok(true);
    }
}
