// use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
// use crate::error::main_error_kind::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
// use crate::error::main_error_kind::core::connection_error_kind::core::postgresql::postgresql_connection_error::PostgresqlConnectionError;
// use diesel::Connection;
// use diesel::connection::TransactionManager;
// use diesel::pg::PgConnection;
// use std::ops::Drop;

// pub struct ConnectionManager {
//     pg_connection: Option<PgConnection>,
// }

// impl<'this> ConnectionManager {
//     pub fn new() -> Self {
//         return Self {
//             pg_connection: None
//         };
//     }

//     pub fn establish_connection(&'this mut self) -> Result<(), ConnectionErrorKind> {
//         if let None = self.pg_connection {
//             match PgConnection::establish("postgres://root:password@postgresql/mem_is") {  // TODO from env
//                 Ok(pg_connection) => {
//                     self.pg_connection = Some(pg_connection);

//                     return Ok(());
//                 },
//                 Err(connection_error) => {
//                     return Err(ConnectionErrorKind::PostgresqlConnectionError(PostgresqlConnectionError::new(connection_error)));
//                 }
//             }
//         }

//         panic!("Logic error, PgConnection is already exist"); // TODO 
//     }

//     pub fn close_connection(&'this mut self) -> () {
//         if let Some(_) = self.pg_connection {
//             self.pg_connection = None;

//             return ();
//         }

//         panic!("Logic error, PgConnection does not exist"); // TODO
//     }

//     pub fn get_connection(&'this self) -> &'this PgConnection {
//         if let Some(ref pg_connection) = self.pg_connection {
//             return pg_connection; 
//         }

//         panic!("Logic error, PgConnection does not exist");  // TODO 
//     }

//     fn close_connection_on_drop(&'this mut self) -> () {
//         self.pg_connection = None;

//         return ();
//     }
// }

// impl Drop for ConnectionManager {
//     fn drop(&mut self) -> () {
//         self.close_connection_on_drop();

//         return ();
//     }
// }

// // pub fn re() -> HttpResponse<Body> {

// //     use redis::Commands;
// //     match redis::Client::open("redis://redis") {
// //         Ok(client) => {
// //             match client.get_connection() {
// //                 Ok(mut con) => {
// //                     // throw away the result, just make sure it does not fail
// //                 con.set::<&'static str, u8, ()>("t", 10).unwrap();
// //                 // read back the key and return it.  Because the return value
// //                 // from the function is a result for integer this will automatically
// //                 // convert into one.
// //                 match con.get::<&'static str, String>("t") {
// //                     Ok(value) => {
// //                     println!("{:?}", value);
// //                     },
// //                     Err(error) => {
// //                         println!("error3");
// //                         println!("{:?}", error);
// //                     }
// //                 }
// //                 },
// //                 Err(error) => {
// //                     println!("error2");
// //                     println!("{:?}", error);
// //                 }
// //             }
// //         },
// //         Err(error) => {
// //             println!("error1");
// //             println!("{:?}", error);
// //         }
// //     }

// //     return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
// // }