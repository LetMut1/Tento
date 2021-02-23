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
        self.pg_connection = Some(PgConnection::establish("postgres://root:password@postgresql/mem_is").unwrap()); // TODO всплывание ошибок // TODO from env
    }

    pub fn get_pg_connection(&'a self) -> &'a PgConnection {        // TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        match self.pg_connection {
            Some(ref pg_connection) => {
                return pg_connection;
            },
            None => {
                panic!("Logic error, please, initilize 'self.pg_connection' with not-None value");
            }
        }
    }
}