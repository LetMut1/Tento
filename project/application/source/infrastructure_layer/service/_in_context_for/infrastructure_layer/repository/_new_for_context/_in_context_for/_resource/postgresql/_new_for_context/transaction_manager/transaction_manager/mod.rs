use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use super::_component::transaction_isolation_level::TransactionIsolationLevel;
use tokio_postgres::Client as Connection;

pub struct TransactionManager;

impl TransactionManager {
    pub async fn start_transaction<'a>(
        connection: &'a mut Connection,
        transaction_isolation_level: TransactionIsolationLevel
    ) -> Result<Self, ErrorAuditor> {
        let mut query = "START TRANSACTION ISOLATION LEVEL".to_string();
        match transaction_isolation_level {
            TransactionIsolationLevel::ReadCommitted => {
                query = query + " READ COMMITTED, READ WRITE, NOT DEFERRABLE;";
            }
            TransactionIsolationLevel::RepeatableRead => {
                query = query + " REPEATABLE READ, READ WRITE, NOT DEFERRABLE;";
            }
            TransactionIsolationLevel::Serializable { read_only, deferrable } => {
                query = query + " SERIALIZABLE,";
                if read_only {
                    query = query + " READ ONLY,";
                } else {
                    query = query + " READ WRITE,";
                }
                if deferrable {
                    query = query + " DEFERRABLE;";
                } else {
                    query = query + " NOT DEFERRABLE;";
                }
            }
        }

        if let Err(error) = connection.execute(query.as_str(), &[]).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(Self);
    }

    pub async fn commit_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), ErrorAuditor> {
        let query = "COMMIT;";

        if let Err(error) = connection.execute(query, &[]).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    pub async fn rollback_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), ErrorAuditor> {
        let query = "ROLLBACK;";

        if let Err(error) = connection.execute(query, &[]).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }
}