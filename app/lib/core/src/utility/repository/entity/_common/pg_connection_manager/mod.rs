use diesel::pg::PgConnection;
use diesel::Connection;

pub struct PGConnectionManager {
    pg_connection: Option<PgConnection>
}

impl<'a> PGConnectionManager {
    pub fn new() -> Self {
        return Self {
            pg_connection: None
        }
    }

    pub fn establish_connection(&'a mut self) -> () {
        match self.pg_connection {
            Some(ref _pg_connection) => {
                // compile_error!("Logic error, PgConnection is already exist");
            },
            None => {
                self.pg_connection = Some(PgConnection::establish("postgres://root:password@postgresql/mem_is").unwrap()); // TODO всплывание ошибок // TODO from env
            }
        }
    }

    pub fn close_connection(&'a mut self) -> () {
        match self.pg_connection {
            Some(ref _pg_connection) => {
                self.pg_connection = None;
            },
            None => {
                // compile_error!("Logic error, PgConnection does not exist");
            }
        }
    }

    pub fn get_connection(&'a self) -> &'a PgConnection {
        match self.pg_connection {
            Some(ref pg_connection) => {
                return pg_connection;
            },
            None => {
                // compile_error!("Logic error, PgConnection does not exist");
                panic!("DELET THIS");// TODO delete
            }
        }
    }
}