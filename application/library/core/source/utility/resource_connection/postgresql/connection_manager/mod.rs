use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::error::main_error_kind::core::_in_context_for::utility::resource_connection::_new_for_context::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::_in_context_for::utility::resource_connection::_new_for_context::connection_error_kind::core::_in_context_for::postgresql::_new_for_context::postgresql_connection_error::PostgresqlConnectionError;
use diesel::Connection;
use diesel::connection::TransactionManager;
use diesel::pg::PgConnection as PostgresqlConnection;
use std::ops::Drop;

pub struct ConnectionManager {
    postgresql_connection: Option<PostgresqlConnection>,
}

impl<'this> ConnectionManager {
    pub fn new() -> Self {
        return Self {
            postgresql_connection: None
        };
    }

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
        if let None = self.postgresql_connection {
            match PostgresqlConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                Ok(postgresql_connection) => {
                    self.postgresql_connection = Some(postgresql_connection);

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
        if let Some(_) = self.postgresql_connection {
            self.postgresql_connection = None;

            return ();
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn get_connection(&'this self) -> &'this PostgresqlConnection {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            return postgresql_connection; 
        }

        panic!("Logic error, PgConnection does not exist");  // TODO 
    }

    pub fn  begin_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            postgresql_connection.transaction_manager().begin_transaction(postgresql_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  commit_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            postgresql_connection.transaction_manager().commit_transaction(postgresql_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  rollback_transaction(&'this self) -> Result<(), DieselError> {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            postgresql_connection.transaction_manager().rollback_transaction(postgresql_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    fn close_connection_on_drop(&'this mut self) -> () {
        self.postgresql_connection = None;

        return ();
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) -> () {
        self.close_connection_on_drop();

        return ();
    }
}