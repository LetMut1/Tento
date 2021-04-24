use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::error::main_error_kind::core::resource_error_kind::core::postgresql::postgresql_error_kind::PostgresqlErrorKind;
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

    pub fn establish_connection(&'this mut self) -> Result<(), ResourceErrorKind> {
        if let None = self.postgresql_connection {
            match PostgresqlConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                Ok(postgresql_connection) => {
                    self.postgresql_connection = Some(postgresql_connection);

                    return Ok(());
                },
                Err(connection_error) => {
                    return Err(ResourceErrorKind::PostgresqlErrorKind(PostgresqlErrorKind::ConnectionError(connection_error)));
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

    pub fn  begin_transaction(&'this self) -> Result<(), ResourceErrorKind> {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            postgresql_connection.transaction_manager().begin_transaction(postgresql_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  commit_transaction(&'this self) -> Result<(), ResourceErrorKind> {
        if let Some(ref postgresql_connection) = self.postgresql_connection {
            postgresql_connection.transaction_manager().commit_transaction(postgresql_connection)?;

            return Ok(());
        }

        panic!("Logic error, PgConnection does not exist"); // TODO
    }

    pub fn  rollback_transaction(&'this self) -> Result<(), ResourceErrorKind> {
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