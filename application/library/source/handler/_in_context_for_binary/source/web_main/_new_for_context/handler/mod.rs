use actix_web::App;
use actix_web::HttpServer;
use crate::actix_web_component::configuration::main_service_configurator::MainServiceConfigurator;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use std::io::Error;
use std::io::ErrorKind;

pub struct Handler;

impl Handler {
    pub async fn handle() -> Result<(), Error> {
        match AggregateConnectionPool::new() {
            Ok(aggregate_connection_pool) => {
                return HttpServer::new(move || {
                    return App::new()
                    .data::<AggregateConnectionPool>(aggregate_connection_pool.clone())
                    .configure(MainServiceConfigurator::configure);
                })
                .bind("0.0.0.0:80")?.run().await; // TODO env or method 
            },
            Err(resource_error_kind) => {
                return Err(Error::new::<String>(ErrorKind::Other, format!("{:?}", resource_error_kind)));
            }
        }
    }
}