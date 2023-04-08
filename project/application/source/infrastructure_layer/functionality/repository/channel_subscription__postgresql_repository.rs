use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use super::postgresql_repository::PostgresqlRepository;

impl PostgresqlRepository<ChannelSubscription> {
    pub async fn create<'a>(database_1_connection: &'a Connection, insert: Insert) -> Result<ChannelSubscription, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.channel_subscription AS cs ( \
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
            .add_parameter(&insert.application_user_id, Type::INT8)
            .add_parameter(&insert.channel_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
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

        let channel_subscription_created_at = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(channel_subscription_created_at_) => channel_subscription_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let channel_subscription = ChannelSubscription::new(
            insert.application_user_id,
            insert.channel_id,
            channel_subscription_created_at
        );

        return Ok(channel_subscription);
    }

    pub async fn is_exist<'a>(
        database_1_connection: &'a Connection,
        application_user_id: i64,
        channel_id: i64
    ) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                cs.application_user_id AS aui \
            FROM public.channel_subscription cs \
            WHERE cs.application_user_id = $1 AND cs.channel_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&channel_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
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
            return Ok(false);
        }

        return Ok(true);
    }
}

pub struct Insert {
    pub application_user_id: i64,
    pub channel_id: i64
}