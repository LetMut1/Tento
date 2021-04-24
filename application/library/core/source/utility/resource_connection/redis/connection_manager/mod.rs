use crate::error::main_error_kind::core::_in_context_for::utility::resource_connection::_new_for_context::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use crate::error::main_error_kind::core::_in_context_for::utility::resource_connection::_new_for_context::connection_error_kind::core::_in_context_for::redis::_new_for_context::redis_connection_error::RedisConnectionError;
use redis::Client;
use redis::Connection as RedisConnection;
use std::ops::Drop;

pub struct ConnectionManager {
    redis_connection: Option<RedisConnection>,
}

impl<'this> ConnectionManager {
    pub fn new() -> Self {
        return Self {
            redis_connection: None
        };
    }

    pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
        if let None = self.redis_connection {
            match Client::open("redis://redis") {  // TODO from env
                Ok(client) => {
                    match client.get_connection() {
                        Ok(redis_connection) => {
                            self.redis_connection = Some(redis_connection);

                            return Ok(());
                        },
                        Err(redis_error) => {
                            return Err(ConnectionErrorKind::RedisConnectionError(RedisConnectionError::new(redis_error)));
                        }
                    }
                },
                Err(redis_error) => {
                    return Err(ConnectionErrorKind::RedisConnectionError(RedisConnectionError::new(redis_error)));
                }
            }
        }

        panic!("Logic error, Connection is already exist"); // TODO 
    }

    pub fn close_connection(&'this mut self) -> () {
        if let Some(_) = self.redis_connection {
            self.redis_connection = None;

            return ();
        }

        panic!("Logic error, Connection does not exist"); // TODO
    }

    pub fn get_connection(&'this mut self) -> &'this mut RedisConnection {
        if let Some(ref mut redis_connection) = self.redis_connection {
            return redis_connection; 
        }

        panic!("Logic error, Connection does not exist");  // TODO 
    }

    fn close_connection_on_drop(&'this mut self) -> () {
        self.redis_connection = None;

        return ();
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) -> () {
        self.close_connection_on_drop();

        return ();
    }
}