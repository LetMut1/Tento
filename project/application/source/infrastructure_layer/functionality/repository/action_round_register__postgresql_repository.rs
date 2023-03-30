use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

pub struct ActionRoundRegister_PostgresqlRepository;

impl ActionRoundRegister_PostgresqlRepository {
    pub async fn create<'a, 'b>(database_2_connection: &'a Connection, insert: Insert<'b>) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.action_round_register AS arr ( \
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
            .add_parameter(&insert.action_round_register_route, Type::TEXT)
            .add_parameter(&insert.action_round_register_method, Type::TEXT)
            .add_parameter(&insert.action_round_register_status_code, Type::INT2)
            .add_parameter(&insert.action_round_register_context, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
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

        if let Err(error) = database_2_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }
}

pub struct Insert<'a> {
    pub action_round_register_route: &'a str,
    pub action_round_register_method: &'a str,
    pub action_round_register_status_code: i16,
    pub action_round_register_context: Option<String>
}