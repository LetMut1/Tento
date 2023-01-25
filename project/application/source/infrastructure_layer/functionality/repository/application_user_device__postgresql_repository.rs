use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct ApplicationUserDevice_PostgresqlRepository;

impl ApplicationUserDevice_PostgresqlRepository {
    pub async fn create<'a>(core_connection: &'a Connection, insert: Insert) -> Result<ApplicationUserDevice, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user_device AS aud ( \
                id, \
                application_user_id \
            ) VALUES ( \
                $1, \
                $2 \
            ) \
            ON CONFLICT ON CONSTRAINT application_user_device2 DO NOTHING;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_device_id, Type::TEXT)
            .add_parameter(&insert.application_user_id, Type::INT8);

        let statement = match core_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = core_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(
            ApplicationUserDevice::new(
                insert.application_user_device_id,
                insert.application_user_id
            )
        );
    }
}

pub struct Insert {
    pub application_user_device_id: String,
    pub application_user_id: i64
}