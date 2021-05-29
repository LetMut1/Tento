use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use diesel::Connection as DieselConnection;
use diesel::connection::TransactionManager as DieselTransactionManager;
use diesel::PgConnection as Connection;

pub struct TransactionManager;

impl TransactionManager {
    pub fn  begin_transaction<'outer_a>(connection: &'outer_a Connection) -> Result<(), ResourceErrorKind> {
        connection.transaction_manager().begin_transaction(connection)?;  // TODO все ли тут определяется, или нужн обловфиш

        return Ok(());
    }

    pub fn  commit_transaction<'outer_a>(connection: &'outer_a Connection) -> Result<(), ResourceErrorKind> {
        connection.transaction_manager().commit_transaction(connection)?;

        return Ok(());
    }

    pub fn  rollback_transaction<'outer_a>(connection: &'outer_a Connection) -> Result<(), ResourceErrorKind> {
        connection.transaction_manager().rollback_transaction(connection)?;

        return Ok(());
    }
}