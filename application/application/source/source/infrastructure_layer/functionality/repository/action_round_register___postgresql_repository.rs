use super::postgresql_repository::insert::Insert11;
use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ActionRoundRegister<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_11: Insert11<'a>,
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let action_round_register_route = insert_11.action_round_register_route.0.as_ref();

        let action_round_register_method = insert_11.action_round_register_method.0.as_ref();

        let action_round_register_context = match insert_11.action_round_register_context {
            Some(ref action_round_register_context_) => Some(action_round_register_context_.0.as_str()),
            None => None,
        };

        let query = "\
            INSERT INTO public.action_round_register AS arr ( \
                route, \
                method, \
                status_code, \
                context, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                current_timestamp(6) \
            );";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &action_round_register_route,
                Type::TEXT,
            )
            .add_parameter(
                &action_round_register_method,
                Type::TEXT,
            )
            .add_parameter(
                &insert_11.action_round_register_status_code.0,
                Type::INT2,
            )
            .add_parameter(
                &action_round_register_context,
                Type::TEXT,
            );

        let statement = match database_2_connection
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

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
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
        };

        return Ok(());
    }
}
