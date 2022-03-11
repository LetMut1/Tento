use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crate::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization as RequestHandlerApplicationUserAuthorization;
use crate::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::Base as RequestHandlerChannelBase;
use hyper::Body;
use hyper::Error as HyperError;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::StatusCode;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::Appender;
use log4rs::config::Config as LogConfig;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use redis::ConnectionInfo;
use std::clone::Clone;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use tokio_postgres::Config as PostgresqlConfig;
use tokio_postgres::NoTls;
use tokio::signal;

pub struct Base;

impl Base {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";
    const DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    pub fn handle(
        binary_file_path: String
    ) -> Result<(), BaseError> {
        Self::load_and_check_environment_variables(binary_file_path.as_str())?;
        Self::configure_log()?;
        Self::run_http_server()?;

        return Ok(());
    }

    fn load_and_check_environment_variables<'a>(
        binary_file_path: &'a str
    ) -> Result<(), BaseError> {
        match Path::new(binary_file_path).parent() {
            Some(file_path) => {
                let production_environment_file_path_buffer = file_path.join(&Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));
                if production_environment_file_path_buffer.exists() {
                    dotenv::from_path(production_environment_file_path_buffer.as_path())?;

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_TRUE)
                } else {
                    let development_local_environment_file_path_buffer = file_path.join(&Path::new(Self::DEVELOPMENT_LOCAL_ENVIRONMENT_FILE_NAME));
                    if development_local_environment_file_path_buffer.exists() {
                        dotenv::from_path(development_local_environment_file_path_buffer.as_path())?;
                    } else {
                        let development_environment_file_path_buffer = file_path.join(&Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));
                        if development_environment_file_path_buffer.exists() {
                            dotenv::from_path(development_environment_file_path_buffer.as_path())?;
                        } else {
                            return Err(BaseError::LogicError {logic_error: LogicError::new(false, "Any ....env files does not exist.")});
                        }
                    }

                    env::set_var(EnvironmentVariableResolver::IS_PRODUCTION_KEY, EnvironmentVariableResolver::IS_PRODUCTION_VALUE_FALSE);
                }

                Self::check_environment_variables()?;

                return Ok(());
            },
            None => {
                return Err(BaseError::LogicError {logic_error: LogicError::new(false, "The directory does not exist.")});
            }
        }
    }

    fn check_environment_variables(
    ) -> Result<(), BaseError> {
        EnvironmentVariableResolver::is_production()?;
        EnvironmentVariableResolver::get_server_socket_address()?;
        EnvironmentVariableResolver::get_logger_roller_log_file_name()?;
        EnvironmentVariableResolver::get_logger_log_file_name()?;
        EnvironmentVariableResolver::get_logger_encoder_pattern()?;
        EnvironmentVariableResolver::get_security_jrwt_encoding_private_key()?;
        EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key()?;
        EnvironmentVariableResolver::get_resource_postgresql_url()?;
        EnvironmentVariableResolver::get_resource_redis_url()?;
        EnvironmentVariableResolver::get_resource_email_server_socket_address()?;

        return Ok(());
    }

    fn configure_log(
    ) -> Result<(), BaseError> {
        let fixed_window_roller = FixedWindowRoller::builder()
            .base(1)
            .build(EnvironmentVariableResolver::get_logger_roller_log_file_name()?.as_str(), 10)?;

        let rolling_file_appender = RollingFileAppender::builder()
            .append(true)
            .encoder(Box::new(PatternEncoder::new(EnvironmentVariableResolver::get_logger_encoder_pattern()?.as_str())))
            .build(
                EnvironmentVariableResolver::get_logger_log_file_name()?,
                Box::new(CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024 * 1024)), Box::new(fixed_window_roller)))
            )?;

        let rolling_file_appender_name = "rfa";

        let appender = Appender::builder().build(rolling_file_appender_name.to_string(), Box::new(rolling_file_appender));

        let root = Root::builder().appender(rolling_file_appender_name.to_string()).build(LevelFilter::Trace);

        let config = LogConfig::builder().appender(appender).build(root)?;

        log4rs::init_config(config)?;

        return Ok(());
    }

     // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
    #[tokio::main]
    async fn run_http_server(
    ) -> Result<(), BaseError> {
        let postgresql_connection_pool = Pool::builder()    // TODO Для девелопмента ТЛС не нужен (НО можно подключить, как вариант), для Продакша - обязательно. Здесь Пул, который содержит только для Дев. Можно Пулы выделить в Оптион для дев и прод окруженияю. Либо через Дженерик, создавать и отдавать в зависимости от от ИзПродакшн значения. Либо Base it on a feature? Probably having NoTls be the feature, since it makes more sense to have TLS by default
        .build(                                                      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
            PostgresqlConnectionManager::new(
                PostgresqlConfig::from_str(EnvironmentVariableResolver::get_resource_postgresql_url()?.as_str())?,
                NoTls
            )
        ).await?;

        let redis_connection_pool = Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        .build(
            RedisConnectionManager::new(
                ConnectionInfo::from_str(EnvironmentVariableResolver::get_resource_redis_url()?.as_str())?
            )?
        ).await?;

        let socket_addres = SocketAddr::from_str(EnvironmentVariableResolver::get_server_socket_address()?.as_str())?;







        // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь). Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
        let service = make_service_fn(move |_: &AddrStream| {
            let postgresql_connection_pool = postgresql_connection_pool.clone();

            let redis_connection_pool = redis_connection_pool.clone();

            async move {
                return Ok::<_, HyperError>(service_fn(move |requset| {
                    let postgresql_connection_pool = postgresql_connection_pool.clone();

                    let redis_connection_pool = redis_connection_pool.clone();

                    return Self::resolve(requset, postgresql_connection_pool, redis_connection_pool);
                }));
            }
        });
         // TODO  TODO  TODO ------------------------------------------------------------------------------------------------------------------








        Server::bind(&socket_addres)       // TODO TODO TODO TODO TODO Настроить сервер для продакшна
            .serve(service)
            .with_graceful_shutdown(Self::create_shutdown_signal())
            .await?;

        return Ok(());
    }

    // TODO  TODO  TODO  УБрать expect. Перерегистрировать с помощью ТОКИО (без использования .with_graceful_shutdown()) ----------
    async fn create_shutdown_signal(
    ) -> () {
        signal::ctrl_c()
            .await
            .expect("Failed to install gracefully shutdown signal");

        return ();
    }

    async fn resolve(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>, 
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Result<Response<Body>, HyperError> {
        match (request.uri().path(), request.method()) {                      // TODO Пути через константы?
            ("v1/m/au/cnfe", &Method::GET) => {
                return Ok(RequestHandlerApplicationUserAuthorization::check_nickname_for_existing(request, postgresql_connection_pool).await);
            },
            ("v1/m/au/cefe", &Method::GET) => {
                return Ok(RequestHandlerApplicationUserAuthorization::check_email_for_existing(request, postgresql_connection_pool).await);
            },
            ("v1/m/au/rbfs", &Method::POST) => {
                return Ok(RequestHandlerApplicationUserAuthorization::register_by_first_step(request, postgresql_connection_pool, redis_connection_pool).await);
            },
            ("v1/m/au/rbls", &Method::POST) => {
            },
            ("v1/m/au/sefr", &Method::POST) => {
            },
            ("v1/m/au/libfs", &Method::POST) => {
            },
            ("v1/m/au/libls", &Method::POST) => {
            },
            ("v1/m/au/sefli", &Method::POST) => {
            },
            ("v1/m/au/rpbfs", &Method::POST) => {
            },
            ("v1/m/au/rpbls", &Method::POST) => {
            },
            ("v1/m/au/sefrp", &Method::POST) => {
            },
            ("v1/m/au/rjawt", &Method::POST) => {
            },
            ("v1/m/au/lo", &Method::POST) => {
            },
            ("v1/m/au/lofad", &Method::POST) => {
            },
            ("v1/m/c/gmbn", &Method::GET) => {
            },
            ("v1/m/c/gmbca", &Method::GET) => {
            },
            ("v1/m/c/gmbsq", &Method::GET) => {
            },
            ("v1/m/c/gmbir", &Method::GET) => {
            },
            _ => {
            }
        }


        //  TODO DELETE --------------------------------------------------------------------------
        match (request.method(), request.uri().path()) {

            // Serve some instructions at /
            (&Method::GET, "/") => Ok(Response::new(Body::from(
                "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
            ))),
    
            // Return the 404 Not Found for other routes.
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
        //  TODO DELETE --------------------------------------------------------------------------
    }

    // TODO DELETE after creating Self::resolve() method!!!!!!!!!!!!!!!!!!!!!!!!!!
    // fn configure_http_server<'a>(
    //     service_config: &'a mut ServiceConfig
    // ) -> () {
    //     service_config     // TODO default_service 
    //     .service(
    //         web::scope("/v1")
    //         .service(
    //             web::scope("/m")
    //             .service(
    //                 web::scope("/na")   // TODO NotAuthorized. Можно ли в новой версии АкстикаВеба убрать этоу чать пути 
    //                 .service( 
    //                     web::scope("/au")
    //                     .route("/cnfe", web::get().to(RequestHandlerApplicationUserAuthorization::check_nickname_for_existingXXXxDelete))
    //                     .route("/cefe", web::get().to(RequestHandlerApplicationUserAuthorization::check_email_for_existingXXXxDelete))
    //                     .route("/rbfs", web::post().to(RequestHandlerApplicationUserAuthorization::register_by_first_stepXXXxDelete))
    //                     .route("/rbls", web::post().to(RequestHandlerApplicationUserAuthorization::register_by_last_step))
    //                     .route("/sefr", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_register))
    //                     .route("/libfs", web::post().to(RequestHandlerApplicationUserAuthorization::log_in_by_first_step))
    //                     .route("/libls", web::post().to(RequestHandlerApplicationUserAuthorization::log_in_by_last_step))
    //                     .route("/sefli", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_log_in))
    //                     .route("/rpbfs", web::post().to(RequestHandlerApplicationUserAuthorization::reset_password_by_first_step))
    //                     .route("/rpbls", web::post().to(RequestHandlerApplicationUserAuthorization::reset_password_by_last_step))
    //                     .route("/sefrp", web::post().to(RequestHandlerApplicationUserAuthorization::send_email_for_reset_password))
    //                     .route("/rjawt", web::post().to(RequestHandlerApplicationUserAuthorization::refresh_json_access_web_token))
    //                 )
    //             )
    //             .service(
    //                 web::scope("/a")
    //                 // .wrap(AuthenticationResolverFactory)             // TODO Делать все в Рекуест Хендлерах
    //                 .service( 
    //                     web::scope("/au")
    //                     .route("/lo", web::post().to(RequestHandlerApplicationUserAuthorization::log_out))
    //                     .route("/lofad", web::post().to(RequestHandlerApplicationUserAuthorization::log_out_from_all_devices))
    //                 )
    //                 .service( 
    //                     web::scope("/c")
    //                     .route("/gmbn", web::get().to(RequestHandlerChannelBase::get_many_by_name))
    //                     .route("/gmbca", web::get().to(RequestHandlerChannelBase::get_many_by_created_at))
    //                     .route("/gmbsq", web::get().to(RequestHandlerChannelBase::get_many_by_subscribers_quantity))
    //                     .route("/gmbir", web::get().to(RequestHandlerChannelBase::get_many_by_id_registry))
    //                 )
    //             )
    //         )
    //     );

    //     return ();
    // }
}