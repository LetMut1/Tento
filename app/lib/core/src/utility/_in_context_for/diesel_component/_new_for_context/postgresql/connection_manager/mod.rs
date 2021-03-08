use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;
use diesel::Connection;
use diesel::connection::AnsiTransactionManager;
use diesel::pg::PgConnection;

pub struct ConnectionManager {
    pg_connection: Option<PgConnection>,
    // ansi_transaction_manager: Option<&'inner AnsiTransactionManager>,
}

impl<'this> ConnectionManager {
    pub fn new() -> Self {
        return Self {
            pg_connection: None,
            // ansi_transaction_manager: None
        };
    }

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
        match self.pg_connection {
            Some(ref _value) => { panic!("Logic error, PgConnection is already exist"); }, // TODO error
            None => { 
                match PgConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                    Ok(value) => {
                        self.pg_connection = Some(value);
                        // self.ansi_transaction_manager = Some(self.pg_connection.unwrap(). transaction_manager());

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
            Some(ref _value) => { 
                self.pg_connection = None;
                // self.ansi_transaction_manager = None;
             },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO error
        };
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        match self.pg_connection {
            Some(ref value) => { return value; },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }

    // pub fn  begin_transaction(&'this self) -> Result<(), DieselErrorKind> {
    //     match self.ansi_transaction_manager.begin_transaction(self.connection_manager.get_connection()) {
    //         Ok(_value) => { return Ok(()); },
    //         Err(value) => { return Err(DieselErrorKind::new_any(value, None)); }
    //     };
    // }
}