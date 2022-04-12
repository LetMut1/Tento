use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use super::_component::transaction_isolation_level::TransactionIsolationLevel;
use tokio_postgres::Client as Connection;

pub struct TransactionManager;

impl TransactionManager {
    pub async fn start_transaction<'a>(
        connection: &'a mut Connection,
        transaction_isolation_level: TransactionIsolationLevel
    ) -> Result<Self, ErrorAggregator> {
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

        connection.execute(query.as_str(), &[]).await?;

        return Ok(Self);
    }

    pub async fn commit_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), ErrorAggregator> {
        let query = "COMMIT;";

        connection.execute(query, &[]).await?;

        return Ok(());
    }

    pub async fn rollback_transaction<'a>(
        self,
        connection: &'a mut Connection
    ) -> Result<(), ErrorAggregator> {
        let query = "ROLLBACK;";

        connection.execute(query, &[]).await?;

        return Ok(());
    }
}