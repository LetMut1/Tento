use diesel::Connection;
use diesel::pg::PgConnection;

pub struct PGConnectionManager {
    pg_connection: Option<PgConnection>
}

impl<'this> PGConnectionManager {
    pub fn new() -> Self {
        return Self {
            pg_connection: None
        }
    }

    pub fn establish_connection(&'this mut self) -> () {
        match self.pg_connection {
            Some(ref _pg_connection) => {
                panic!("Logic error, PgConnection is already exist");   // TODO error 
            },
            None => {
                self.pg_connection = Some(PgConnection::establish("postgres://root:password@postgresql/mem_is").unwrap()); // TODO всплывание ошибок // TODO from env
            }
        }
    }

    pub fn close_connection(&'this mut self) -> () {
        match self.pg_connection {
            Some(ref _pg_connection) => {
                self.pg_connection = None;
            },
            None => {
                 panic!("Logic error, PgConnection does not exist"); // TODO error
            }
        }
    }

    pub fn get_connection(&'this self) -> &'this PgConnection {
        match self.pg_connection {
            Some(ref pg_connection) => {
                return pg_connection;
            },
            None => {
                 panic!("Logic error, PgConnection does not exist");    // TODO error
            }
        }
    }
}