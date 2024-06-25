use super::Resolver;
use crate::infrastructure_layer::data::{
    auditor::{
        Backtrace,
    },
    control_type::PostgresqlTransaction,
    error::AlternativeWorkflow,
    error::ResultConverter,
};
use tokio_postgres::Client as Connection;
impl Resolver<PostgresqlTransaction> {
    pub async fn start<'a>(connection: &'a Connection, transaction_isolation_level: TransactionIsolationLevel) -> Result<Self, AlternativeWorkflow> {
        let mut query = "START TRANSACTION ISOLATION LEVEL".to_string();
        match transaction_isolation_level {
            TransactionIsolationLevel::ReadCommitted => {
                query += " READ COMMITTED, READ WRITE, NOT DEFERRABLE;";
            }
            TransactionIsolationLevel::RepeatableRead => {
                query += " REPEATABLE READ, READ WRITE, NOT DEFERRABLE;";
            }
            TransactionIsolationLevel::Serializable {
                read_only,
                deferrable,
            } => {
                query += " SERIALIZABLE,";
                if read_only {
                    query += " READ ONLY,";
                } else {
                    query += " READ WRITE,";
                }
                if deferrable {
                    query += " DEFERRABLE;";
                } else {
                    query += " NOT DEFERRABLE;";
                }
            }
        }
        connection
            .execute(
                query.as_str(),
                &[],
            )
            .await
            .convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        return Ok(Self::new());
    }
    pub async fn commit<'a>(self, connection: &'a Connection) -> Result<(), AlternativeWorkflow> {
        let query = "COMMIT;";
        connection
            .execute(
                query,
                &[],
            )
            .await
            .convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        return Ok(());
    }
    pub async fn rollback<'a>(self, connection: &'a Connection) -> Result<(), AlternativeWorkflow> {
        let query = "ROLLBACK;";
        connection
            .execute(
                query,
                &[],
            )
            .await
            .convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        return Ok(());
    }
}
pub enum TransactionIsolationLevel {
    ReadCommitted,
    RepeatableRead,
    Serializable {
        read_only: bool,
        deferrable: bool,
    },
}
