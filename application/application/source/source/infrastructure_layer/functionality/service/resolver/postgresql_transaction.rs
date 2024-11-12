use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    },
    capture::Capture,
};
use dedicated_crate::void::Void;
use std::future::Future;
use deadpool_postgres::Client;
use super::Resolver;
pub struct PostgresqlTransaction<'a> {
    // Should be &'_ mut for outer requirement.
    client: &'a mut Client,
}
impl<'a> PostgresqlTransaction<'a> {
    pub fn get_client<'b>(&'b self) -> &'b Client {
        return &*self.client;
    }
}
impl Resolver<PostgresqlTransaction<'_>> {
    pub fn start<'a>(
        client: &'a mut Client,
        transaction_isolation_level: TransactionIsolationLevel,
    ) -> impl Future<Output = Result<PostgresqlTransaction<'a>, AggregateError>> + Send {
        return async move {
            let mut query = "START TRANSACTION ISOLATION LEVEL".to_string();
            match transaction_isolation_level {
                TransactionIsolationLevel::ReadCommitted => {
                    query = format!(
                        "{} READ COMMITTED,READ WRITE,NOT DEFERRABLE;",
                        query.as_str(),
                    );
                }
                TransactionIsolationLevel::RepeatableRead => {
                    query = format!(
                        "{} REPEATABLE READ,READ WRITE,NOT DEFERRABLE;",
                        query.as_str(),
                    );
                }
                TransactionIsolationLevel::Serializable {
                    read_only,
                    deferrable,
                } => {
                    if read_only && deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ ONLY,DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if read_only && !deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ ONLY,NOT DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if !read_only && deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ WRITE,DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if !read_only && !deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ WRITE,NOT DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                }
            }
            if let Result::Err(aggregate_error) = client
            .simple_query(query.as_str())
            .await
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )
            {
                let _ = client
                .simple_query("ROLLBACK;")
                .await;
                return Result::Err(aggregate_error);
            }
            return Result::Ok(
                PostgresqlTransaction {
                    client,
                },
            );
        };
    }
    pub fn commit<'a>(postgresql_transaction: PostgresqlTransaction<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if let Result::Err(aggregate_error) = postgresql_transaction
            .client
            .simple_query("COMMIT;")
            .await
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )
            {
                let _ = postgresql_transaction.client
                .simple_query("ROLLBACK;")
                .await;
                return Result::Err(aggregate_error);
            }
            return Result::Ok(());
        };
    }
    pub fn rollback<'a>(postgresql_transaction: PostgresqlTransaction<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            postgresql_transaction
            .client
            .simple_query("ROLLBACK;")
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
