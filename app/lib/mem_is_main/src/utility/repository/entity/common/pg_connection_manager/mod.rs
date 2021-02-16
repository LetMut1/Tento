use diesel::{Connection, pg::PgConnection};

pub struct PGConnectionManager {
    pg_connection: PgConnection
}

impl<'a> PGConnectionManager {
    pub fn new() -> Self {  // TODO всплывание ошибок 
        return Self {
            pg_connection: PgConnection::establish("postgres://root:password@postgresql/mem_is").unwrap()   // TODO всплывание ошибок
        }
    }

    pub fn get_connection(&'a self) -> &'a PgConnection {
        return &self.pg_connection;
    }
}