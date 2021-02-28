use crate::error::error::core::common::connection::connection_error::ConnectionError;
use crate::error::error::core::common::connection::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;
use crate::error::context::Context;
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

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionError> {
        match self.pg_connection {
            Some(ref _value) => { panic!("Logic error, PgConnection is already exist"); }, // TODO error Класть в Состояние фрэймворка (чтобы брать при запуске один раз и хранить в памяти)
            None => { 
                match PgConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
                    Ok(value) => {
                        self.pg_connection = Some(value);

                        return Ok(());
                    },
                    Err(value) => {
                        return Err(ConnectionError::Postgresql(PostgresqlConnectionErrorKind::CanNotEstablish(Context::new(Some(value), None))));
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