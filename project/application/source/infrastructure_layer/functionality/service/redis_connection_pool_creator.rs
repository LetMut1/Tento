use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::redis::ConnectionInfo;
use std::clone::Clone;

pub struct RedisConnectionPoolCreator;

impl RedisConnectionPoolCreator {
    pub async fn create<'a>(environment: &'a Environment, connection_info: &'a ConnectionInfo) -> Result<Pool<RedisConnectionManager>, ErrorAuditor> {
        match *environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                let redis_connection_manager = match RedisConnectionManager::new(connection_info.clone()) {
                    Ok(redis_connection_manager_) => redis_connection_manager_,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                return match Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                    .build(redis_connection_manager)
                    .await {
                    Ok(redis_connection_pool_) => Ok(redis_connection_pool_),
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };
            }
        }
    }
}