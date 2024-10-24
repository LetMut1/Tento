use super::Resolver;
use crate::infrastructure_layer::data::capture::Capture;
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use std::future::Future;
use tokio_postgres::Client as Connection;
use dedicated_crate::void::Void;
pub struct PostgresqlTransaction;
impl Resolver<PostgresqlTransaction> {
    pub fn start<'a>(
        connection: &'a Connection,
        transaction_isolation_level: TransactionIsolationLevel,
    ) -> impl Future<Output = Result<Self, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
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
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(Self::new());
        };
    }
    pub fn commit<'a>(self, connection: &'a Connection) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "COMMIT;";
            connection
                .execute(
                    query,
                    &[],
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(());
        };
    }
    pub fn rollback<'a>(self, connection: &'a Connection) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let query = "ROLLBACK;";
            connection
                .execute(
                    query,
                    &[],
                )
                .await
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            return Result::Ok(());
        };
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
