use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;
impl PostgresqlRepository<ApplicationUserDevice> {
    pub async fn create_1<'a>(database_1_connection: &'a Connection, insert_1: Insert1) -> Result<ApplicationUserDevice, Auditor<Error>> {
        let application_user_device_id = insert_1.application_user_device_id.as_str();
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
            .add_parameter(&application_user_device_id, Type::TEXT)
            .add_parameter(&insert_1.application_user_id, Type::INT8);
        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;
        database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert(Backtrace::new(line!(), file!()))?;
        return Ok(ApplicationUserDevice::new(
            insert_1.application_user_device_id,
            insert_1.application_user_id,
        ));
    }
}
pub struct Insert1 {
    pub application_user_device_id: String,
    pub application_user_id: i64,
}
