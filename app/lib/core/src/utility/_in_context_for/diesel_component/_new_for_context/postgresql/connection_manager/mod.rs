use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;
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
        match self.pg_connection {
            Some(_) => { panic!("Logic error, PgConnection is already exist"); }, // TODO error
            None => { 
                match PgConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                    Ok(pg_connection) => {
                        self.pg_connection = Some(pg_connection);

                        return Ok(());
                    },
                    Err(connection_error) => {
                        return Err(ConnectionErrorKind::Postgresql(PostgresqlConnectionErrorKind::new_can_not_establish(connection_error, None)));
                    }
                };
             }
        };
    }

    pub fn close_connection(&'this mut self) -> () {
        match self.pg_connection {
            Some(_) => { 
                self.pg_connection = None;

                return ();
            },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO error
        };
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        match self.pg_connection {
            Some(ref pg_connection) => { 
                return pg_connection; 
            },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  begin_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref pg_connection) => { 
                match pg_connection.transaction_manager().begin_transaction(pg_connection) {
                    Ok(_) => { 
                        return Ok(()); 
                    },
                    Err(error) => { 
                        return Err(DieselErrorKind::new_any(error, None)); 
                    }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  commit_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref pg_connection) => { 
                match pg_connection.transaction_manager().commit_transaction(pg_connection) {
                    Ok(_) => { 
                        return Ok(()); 
                    },
                    Err(error) => { 
                        return Err(DieselErrorKind::new_any(error, None)); 
                    }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  rollback_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref pg_connection) => { 
                match pg_connection.transaction_manager().rollback_transaction(pg_connection) {
                    Ok(_) => { 
                        return Ok(()); 
                    },
                    Err(error) => { 
                        return Err(DieselErrorKind::new_any(error, None)); 
                    }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
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