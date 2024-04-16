use super::Resolver;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::Client as Connection;
use crate::infrastructure_layer::data::error::Other;

pub use crate::infrastructure_layer::data::control_type::PostgresqlTransaction;

impl Resolver<PostgresqlTransaction> {
    pub async fn start<'a>(
        connection: &'a Connection,
        transaction_isolation_level: TransactionIsolationLevel,
    ) -> Result<Self, Auditor<Error>> {
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

        if let Err(error) = connection
            .execute(
                query.as_str(),
                &[],
            )
            .await
        {
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

        return Ok(Self::new());
    }

    pub async fn commit<'a>(
        self,
        connection: &'a Connection,
    ) -> Result<(), Auditor<Error>> {
        let query = "COMMIT;";

        if let Err(error) = connection
            .execute(
                query,
                &[],
            )
            .await
        {
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

        return Ok(());
    }

    pub async fn rollback<'a>(
        self,
        connection: &'a Connection,
    ) -> Result<(), Auditor<Error>> {
        let query = "ROLLBACK;";

        if let Err(error) = connection
            .execute(
                query,
                &[],
            )
            .await
        {
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
