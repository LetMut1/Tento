use crate::domain_layer::data::entity::system_registry::Level;
use crate::domain_layer::functionality::service::system_registry__level_matcher::SystemRegistry_LevelMatcher;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::system_registry__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::system_registry__postgresql_repository::SystemRegistry_PostgresqlRepository;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[allow(non_camel_case_types)]
pub struct SystemRegistry_Creator;

impl SystemRegistry_Creator {
    pub async fn create<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        message: String,
        level: Level
    ) -> Result<(), ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let system_registry_level = SystemRegistry_LevelMatcher::r#match(level);

        let insert = Insert {
            message,
            level: system_registry_level
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(mut error) = SystemRegistry_PostgresqlRepository::create(&*database_2_postgresql_pooled_connection, insert).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}