use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;
use diesel::Connection;
use diesel::connection::TransactionManager;
use diesel::pg::PgConnection;

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
                    Ok(value) => {
                        self.pg_connection = Some(value);

                        return Ok(());
                    },
                    Err(value) => {
                        return Err(ConnectionErrorKind::Postgresql(PostgresqlConnectionErrorKind::new_can_not_establish(value, None)));
                    }
                };
             }
        };
    }

    pub fn close_connection(&'this mut self) -> () {
        match self.pg_connection {
            Some(_) => { self.pg_connection = None; },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO error
        };
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        match self.pg_connection {
            Some(ref value) => { return value; },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  begin_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref value) => { 
                match value.transaction_manager().begin_transaction(value) {
                    Ok(_) => { return Ok(()); },
                    Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  commit_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref value) => { 
                match value.transaction_manager().commit_transaction(value) {
                    Ok(_) => { return Ok(()); },
                    Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    pub fn  rollback_transaction(&'this self) -> Result<(), DieselErrorKind> {
        match self.pg_connection {
            Some(ref value) => { 
                match value.transaction_manager().rollback_transaction(value) {
                    Ok(_) => { return Ok(()); },
                    Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
                };
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }
}