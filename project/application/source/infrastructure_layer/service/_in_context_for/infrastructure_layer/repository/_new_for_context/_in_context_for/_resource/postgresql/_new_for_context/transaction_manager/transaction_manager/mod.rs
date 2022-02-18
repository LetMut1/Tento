use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use postgres::Client as Connection;
use super::_component::transaction_isolation_level::TransactionIsolationLevel;

pub struct TransactionManager;

impl TransactionManager {
    pub fn start_transaction<'a>(
        connection: &'a mut Connection,
        transaction_isolation_level: TransactionIsolationLevel
    ) -> Result<Self, BaseError> {
        let mut query = "START TRANSACTION ISOLATION LEVEL".to_string();
        match transaction_isolation_level {
            TransactionIsolationLevel::ReadCommitted => {
                query = query + " READ COMMITTED, READ WRITE, NOT DEFERRABLE;";
            },
            TransactionIsolationLevel::RepeatableRead => {
                query = query + " REPEATABLE READ, READ WRITE, NOT DEFERRABLE;";
            },
            TransactionIsolationLevel::Serializable {read_only, deferrable} => {
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

        connection.execute(query.as_str(), &[])?;

        return Ok(Self);
    }

    pub fn commit_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), BaseError> {
        let query = "COMMIT;";

        connection.execute(query, &[])?;

        return Ok(());
    }

    pub fn rollback_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), BaseError> {
        let query = "ROLLBACK;";

        connection.execute(query, &[])?;

        return Ok(());
    }
}