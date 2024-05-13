use super::Creator;
use crate::infrastructure_layer::data::environment_configurationxxx::Environment;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::ConnectionInfo;
use std::clone::Clone;
use crate::infrastructure_layer::data::auditor::ErrorConverter;

pub use crate::infrastructure_layer::data::control_type::RedisConnectonPool;

impl Creator<RedisConnectonPool> {
    pub async fn create<'a>(
        environment: &'a Environment,
        connection_info: &'a ConnectionInfo,
    ) -> Result<Pool<RedisConnectionManager>, Auditor<Error>> {
        let redis_connection_pool = match *environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development | Environment::LocalDevelopment => {
                let redis_connection_manager = RedisConnectionManager::new(connection_info.clone()).convert(Backtrace::new(line!(), file!()))?;

                Pool::builder() // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                    .build(redis_connection_manager)
                    .await
                    .convert(Backtrace::new(line!(), file!()))?
            }
        };

        return Ok(redis_connection_pool);
    }
}
