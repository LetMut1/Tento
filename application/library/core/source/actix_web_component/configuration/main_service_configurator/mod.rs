use core::panic;

use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::actix_web_component::middleware::authentication_resolver::authentication_resolver_factory::AuthenticationResolverFactory;
use crate::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::Authorization;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;

pub struct MainServiceConfigurator;

impl<'outer_a> MainServiceConfigurator {
    pub fn safely_configure(service_config: &mut ServiceConfig) -> () {
        match AggregateConnectionPool::new() {
            Ok(aggregate_connection_pool) => {
                Self::configure(service_config, aggregate_connection_pool);

                return ();
            },
            Err(resource_error_kind) => {
                panic!(format!("{:?}", resource_error_kind).as_str());      // TODO TEst it // TODO правилен ли тот, факто что здест Паника, и остановит ли это дальнейшую инициализацию приложения
            }
        }
    }

    fn configure(service_config: &mut ServiceConfig, aggregate_connection_pool: AggregateConnectionPool) -> () {
        service_config                                                   // TODO default_service 
        .data::<AggregateConnectionPool>(aggregate_connection_pool)
        .service(
            web::scope("/api")
            .service(
                web::scope("/v1")
                .service(
                    web::scope("/m")
                    .service(
                        web::scope("/na")     // TODO можно ли объединить в Скопе без роута. ( Если есть Скоуп с пустым Роуом, то другие Скоцпы не воспринимаютмя)
                        .service( 
                            web::scope("/au")
                            .route("/pr", web::post().to(Authorization::pre_register))
                            .route("/r", web::post().to(Authorization::register))
                            .route("/refr", web::post().to(Authorization::resend_email_for_register))
                            .route("/pli", web::post().to(Authorization::pre_log_in))
                            .route("/refl", web::post().to(Authorization::resend_email_for_log_in))
                            .route("/li", web::post().to(Authorization::log_in))
                            .route("/cnfe", web::get().to(Authorization::check_nickname_for_existing))
                            .route("/cefe", web::get().to(Authorization::check_email_for_existing))
                            .route("/rjawt", web::post().to(Authorization::refresh_json_access_web_token))
                            .route("/prp", web::post().to(Authorization::pre_reset_password))
                            .route("/rp", web::post().to(Authorization::reset_password))
                            .route("/refrp", web::post().to(Authorization::resend_email_for_reset_password))
                        )
                    )
                    .service(
                        web::scope("/a")
                        .wrap(AuthenticationResolverFactory)
                        .service( 
                            web::scope("/au")
                            .route("/lo", web::post().to(Authorization::log_out))
                            .route("/lofad", web::post().to(Authorization::log_out_from_all_devices))
                        )
                    )
                )
            )
        );

        return ();
    }
}