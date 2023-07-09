use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserDevice> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert: Insert,
    ) -> Result<ApplicationUserDevice, ErrorAuditor> {
        let application_user_id = insert.application_user_id.get();

        let application_user_device_id = insert.application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_device AS aud ( \
                id, \
                application_user_id \
            ) VALUES ( \
                $1, \
                $2 \
            ) \
            ON CONFLICT ON CONSTRAINT application_user_device2 DO NOTHING;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_id,
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

        if let Err(error) = database_1_connection
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

        return Ok(
            ApplicationUserDevice::new(
                insert.application_user_device_id,
                insert.application_user_id,
            ),
        );
    }
}

pub struct Insert {
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_id: ApplicationUser_Id,
}
