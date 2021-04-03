use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error::PostgresqlConnectionError;
use diesel::Connection;
use diesel::connection::TransactionManager;
use diesel::pg::PgConnection;
use std::ops::Drop;

pub struct ConnectionManager {
    pg_connection: Option<PgConnection>,
}

impl<'this> ConnectionManager {
    pub fn new() -> Self {
        return Self {
            pg_connection: None
        };
    }

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
        if let None = self.pg_connection {
            match PgConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                Ok(pg_connection) => {
                    self.pg_connection = Some(pg_connection);

                    return Ok(());
                },
                Err(connection_error) => {
                    return Err(ConnectionErrorKind::PostgresqlConnectionError(PostgresqlConnectionError::new(connection_error)));
                }
            }
        }

        panic!("Logic error, PgConnection is already exist"); // TODO 
    }

    pub fn close_connection(&'this mut self) -> () {
        if let Some(_) = self.pg_connection {
            self.pg_connection = None;

            return ();
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        if let Some(ref pg_connection) = self.pg_connection {
            return pg_connection; 
        }

        panic!("Logic error, PgConnection does not exist");  // TODO 
    }

    pub fn  begin_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref pg_connection) = self.pg_connection {
            pg_connection.transaction_manager().begin_transaction(pg_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  commit_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref pg_connection) = self.pg_connection {
            pg_connection.transaction_manager().commit_transaction(pg_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  rollback_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref pg_connection) = self.pg_connection {
            pg_connection.transaction_manager().rollback_transaction(pg_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    fn close_connection_on_drop(&'this mut self) -> () {
        self.pg_connection = None;

        return ();
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) -> () {
        self.close_connection_on_drop();

        return ();
    }
}