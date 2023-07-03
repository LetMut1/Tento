use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Method;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Route;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_StatusCode;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;

impl PostgresqlRepository<ActionRoundRegister<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert: Insert<'a>,
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let action_round_register_route = insert.action_round_register_route.get();

        let action_round_register_method = insert.action_round_register_method.get();

        let action_round_register_status_code = insert.action_round_register_status_code.get();

        let action_round_register_context = match insert.action_round_register_context {
            Some(ref action_round_register_context_) => Some(action_round_register_context_.get()),
            None => None,
        };

        let query = "INSERT INTO public.action_round_register AS arr ( \
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
                &action_round_register_status_code,
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

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
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
        };

        return Ok(());
    }
}

pub struct Insert<'a> {
    pub action_round_register_route: ActionRoundRegister_Route<'a>,
    pub action_round_register_method: ActionRoundRegister_Method<'a>,
    pub action_round_register_status_code: ActionRoundRegister_StatusCode,
    pub action_round_register_context: Option<ActionRoundRegister_Context>,
}
