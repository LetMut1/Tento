use diesel::connection::AnsiTransactionManager;
use super::connection_manager::ConnectionManager;

pub struct TransactionManager<'outer> {
    ansi_transaction_manager: &'outer AnsiTransactionManager
}

// impl<'outer> TransactionManager<'outer> {
//     pub fn new()
// }