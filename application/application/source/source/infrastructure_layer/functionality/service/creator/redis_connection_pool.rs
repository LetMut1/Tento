use super::Creator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::ConnectionInfo;
use std::clone::Clone;
use crate::infrastructure_layer::data::error::Other;

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
                let redis_connection_manager = match RedisConnectionManager::new(connection_info.clone()) {
                    Ok(redis_connection_manager_) => redis_connection_manager_,
                    Err(error) => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Runtime {
                                    runtime: Runtime::Other {
                                        other: Other::new(error),
                                    },
                                },
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };

                let redis_connection_pool_ = match Pool::builder() // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                    .build(redis_connection_manager)
                    .await
                {
                    Ok(redis_connection_pool__) => redis_connection_pool__,
                    Err(error) => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Runtime {
                                    runtime: Runtime::Other {
                                        other: Other::new(error),
                                    },
                                },
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };

                redis_connection_pool_
            }
        };

        return Ok(redis_connection_pool);
    }
}
