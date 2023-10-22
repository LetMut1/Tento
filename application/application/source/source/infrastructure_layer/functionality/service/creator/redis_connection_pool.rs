use super::Creator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::ConnectionInfo;
use std::clone::Clone;

pub use crate::infrastructure_layer::data::control_type::RedisConnectonPool;

impl Creator<RedisConnectonPool> {
    pub async fn create<'a>(
        environment: &'a Environment,
        connection_info: &'a ConnectionInfo,
    ) -> Result<Pool<RedisConnectionManager>, ErrorAuditor> {
        let redis_connection_pool = match *environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development | Environment::LocalDevelopment => {
                let redis_connection_manager = match RedisConnectionManager::new(connection_info.clone()) {
                    Ok(redis_connection_manager_) => redis_connection_manager_,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                Error::Runtime {
                                    runtime: Runtime::Resource {
                                        resource: ResourceError::Redis {
                                            redis_error: error,
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

                let redis_connection_pool_ = match Pool::builder() // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                    .build(redis_connection_manager)
                    .await
                {
                    Ok(redis_connection_pool__) => redis_connection_pool__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                Error::Runtime {
                                    runtime: Runtime::Resource {
                                        resource: ResourceError::Redis {
                                            redis_error: error,
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

                redis_connection_pool_
            }
        };

        return Ok(redis_connection_pool);
    }
}
