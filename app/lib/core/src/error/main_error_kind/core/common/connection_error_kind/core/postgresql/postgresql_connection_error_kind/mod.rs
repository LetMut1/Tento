use crate::error::context::Context;
use diesel::result::ConnectionError;

#[derive(Debug)]
pub enum PostgresqlConnectionErrorKind {
    CanNotEstablish(Context<ConnectionError>)
}