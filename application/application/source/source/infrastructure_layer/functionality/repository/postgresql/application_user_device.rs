use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::application_user_device::ApplicationUserDevice,
    infrastructure_layer::{
        data::capture::Capture,
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use std::future::Future;
use tokio_postgres::{
    types::Type,
    Client as Connection,
};
use void::Void;
impl PostgresqlRepository<ApplicationUserDevice> {
    pub fn create_1<'a>(
        database_1_connection: &'a Connection,
        insert_1: Insert1,
    ) -> impl Future<Output = Result<ApplicationUserDevice, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let application_user_device__id = insert_1.application_user_device__id.as_str();
            let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
            let query = "\
                INSERT INTO public.application_user_device AS aud ( \
                    id, \
                    application_user__id \
                ) VALUES ( \
                    $1, \
                    $2 \
                ) \
                ON CONFLICT ON CONSTRAINT application_user_device2 DO NOTHING;";
            prepared_statemant_parameter_convertation_resolver
                .add_parameter(
                    &application_user_device__id,
                    Type::TEXT,
                )
                .add_parameter(
                    &insert_1.application_user__id,
                    Type::INT8,
                );
            let statement = database_1_connection
                .prepare_typed(
                    query,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
                )
                .await
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            database_1_connection
                .query(
                    &statement,
                    prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(
                ApplicationUserDevice::new(
                    insert_1.application_user_device__id,
                    insert_1.application_user__id,
                ),
            );
        };
    }
}
pub struct Insert1 {
    pub application_user_device__id: String,
    pub application_user__id: i64,
}
