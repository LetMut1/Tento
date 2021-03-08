use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;
use diesel::Connection;
use diesel::pg::PgConnection;

pub struct PGConnectionManager {
    pg_connection: Option<PgConnection>
}

impl<'this> PGConnectionManager {
    pub fn new() -> Self {
        return Self {
            pg_connection: None
        };                          // TODO везде ли проставлны 
    }

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
        match self.pg_connection {
            Some(ref _value) => { panic!("Logic error, PgConnection is already exist"); }, // TODO error Класть в Состояние фрэймворка (чтобы брать при запуске один раз и хранить в памяти)
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
            Some(ref _value) => { self.pg_connection = None; },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO error
        };
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        match self.pg_connection {
            Some(ref value) => { return value; },
            None => { panic!("Logic error, PgConnection does not exist"); } // TODO Error
        };
    }
}