use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::environment_configuration::ENVIRONMENT_CONFIGURATION_FILE_PATH;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::creator::RedisConnectonPool;
use crate::infrastructure_layer::functionality::service::loader::Loader;
use crate::presentation_layer::data::http_route_registry::HttpRouteRegistry;
use crate::presentation_layer::functionality::action::route_not_found;
use crate::presentation_layer::functionality::action::version_1::application_user__authorization;
use crate::presentation_layer::functionality::action::version_1::channel__base;
use crate::presentation_layer::functionality::action::version_1::channel_subscription__base;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Error as HyperError;
use extern_crate::hyper::Method;
use extern_crate::hyper::Server;
use extern_crate::hyper::server::conn::AddrStream;
use extern_crate::hyper::service::make_service_fn;
use extern_crate::hyper::service::service_fn;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio::runtime::Builder;
use extern_crate::tokio::signal;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use std::sync::Arc;

pub struct RunServerProcessor;

impl RunServerProcessor {
    pub fn process() -> Result<(), ErrorAuditor> {
        let environment_configuration = match Loader::<EnvironmentConfiguration>::load_from_file(ENVIRONMENT_CONFIGURATION_FILE_PATH) {
            Ok(environment_configuration_) => environment_configuration_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let runtime = match Builder::new_multi_thread()
            .enable_all()
            .build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(mut error) = runtime.block_on(
            Self::run_http_server(environment_configuration)
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn run_http_server(environment_configuration: EnvironmentConfiguration) -> Result<(), ErrorAuditor> {     // TODO  TODO  TODO ---- create HTTP2 (h2).   // TODO HTTP3 (QUICK) (h3), когда будет готов.!!!!!!!!!!!
        let environment = environment_configuration.get_pushable_environment_configuration().get_environment();

        let postgresql_connection_pool_aggregator = match *environment {
            Environment::Production => {
                todo!();           // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                let database_1_postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(
                    environment,
                    environment_configuration.get_database_1_postgresql_configuration()
                ).await {
                    Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let database_2_postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(
                    environment,
                    environment_configuration.get_database_2_postgresql_configuration()
                ).await {
                    Ok(database_2_postgresql_connection_pool_) => database_2_postgresql_connection_pool_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                PostgresqlConnectionPoolAggregator::LocalDevelopment { database_1_postgresql_connection_pool, database_2_postgresql_connection_pool }
            }
        };

        let redis_connection_pool = match Creator::<RedisConnectonPool>::create(
            environment,
            environment_configuration.get_database_1_redis_connection_info()
        ).await {
            Ok(redis_connection_pool_) => redis_connection_pool_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let builder = match Server::try_bind(environment_configuration.get_application_server_socket_address()) {
            Ok(builder_) => builder_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let environment_configuration_ = Arc::new(environment_configuration);

        // TODO  TODO  TODO ---------  убрать Замыкания, написав и стипизировав функцию (https://docs.rs/futures/latest/futures/future/type.BoxFuture.html может помочь).
        // Либо так https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs Но здесь сущает future::Ready<>.
        // https://stackoverflow.com/questions/55606450/how-to-share-immutable-configuration-data-with-hyper-request-handlers
        let service = make_service_fn(
            move |_: &'_ AddrStream| {
                let environment_configuration__ = environment_configuration_.clone();

                let postgresql_connection_pool_aggregator_ = postgresql_connection_pool_aggregator.clone();

                let redis_connection_pool_ = redis_connection_pool.clone();

                async move {
                    return Ok::<_, HyperError>(
                        service_fn(
                            move |request| {
                                let environment_configuration___ = environment_configuration__.clone();

                                let postgresql_connection_pool_aggregator__ = postgresql_connection_pool_aggregator_.clone();

                                let redis_connection_pool__ = redis_connection_pool_.clone();

                                return async move {
                                    let (database_1_postgresql_connection_pool_, database_2_postgresql_connection_pool_) = match postgresql_connection_pool_aggregator__ {
                                        PostgresqlConnectionPoolAggregator::LocalDevelopment {
                                            database_1_postgresql_connection_pool, database_2_postgresql_connection_pool
                                        } => (database_1_postgresql_connection_pool, database_2_postgresql_connection_pool)
                                    };

                                    return Ok::<_, HyperError>(
                                        Self::resolve(
                                            environment_configuration___.get_pushable_environment_configuration(),
                                            request,
                                            &database_1_postgresql_connection_pool_,
                                            &database_2_postgresql_connection_pool_,
                                            &redis_connection_pool__
                                        ).await
                                    );
                                };
                            }
                        )
                    );
                }
            }
        );
        // TODO  TODO  TODO ------------------------------------------------------------------------------------------------------------------

        if let Err(error) = builder       // TODO TODO TODO TODO TODO Настроить сервер для продакшна
            .serve(service)
            .with_graceful_shutdown(Self::create_shutdown_signal())
            .await {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

        return Ok(());
    }

    async fn create_shutdown_signal() -> () {    // TODO  TODO  TODO  УБрать expect. Перерегистрировать с помощью ТОКИО (без использования .with_graceful_shutdown()) ----------
        signal::ctrl_c()
            .await
            .expect("Failed to install gracefully shutdown signal");

        return ();
    }

    async fn resolve<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let route = request.uri().path();

        let method = request.method();

        match (route, method) {
            // Area for existing routes with not authorized user.
            // GET functional.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING, &Method::POST) => {
                return application_user__authorization::check_nickname_for_existing::CheckNicknameForExisting::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING, &Method::POST) => {
                return application_user__authorization::check_email_for_existing::CheckEmailForExisting::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::register_by_first_step::RegisterByFirstStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_SECOND_STEP, &Method::POST) => {
                return application_user__authorization::register_by_second_step::RegisterBySecondStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::register_by_last_step::RegisterByLastStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_REGISTER, &Method::POST) => {
                return application_user__authorization::send_email_for_register::SendEmailForRegister::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::authorize_by_first_step::AuthorizeByFirstStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::authorize_by_last_step::AuthorizeByLastStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE, &Method::POST) => {
                return application_user__authorization::send_email_for_authorize::SendEmailForAuthorize::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP, &Method::POST) => {
                return application_user__authorization::reset_password_by_last_step::ResetPasswordByLastStep::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD, &Method::POST) => {
                return application_user__authorization::send_email_for_reset_password::SendEmailForResetPassword::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REFRESH_APPLICATION_USER_ACCESS_TOKEN, &Method::POST) => {
                return application_user__authorization::refresh_application_user_access_token::RefreshApplicationUserAccessToken::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // Area for existing routes with authorized user.
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE, &Method::POST) => {
                return application_user__authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE, &Method::POST) => {
                return application_user__authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_ONE_BY_ID, &Method::POST) => {
                return channel__base::get_one_by_id::GetOneByID::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS, &Method::POST) => {
                return channel__base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_SUBSCRIPTION, &Method::POST) => {
                return channel__base::get_many_by_subscription::GetManyBySubscription::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // GET functional.
            (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_PUBLIC_BY_NAME, &Method::POST) => {
                return channel__base::get_many_public_by_name::GetManyPublicByName::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            (HttpRouteRegistry::VERSION_1__CHANNEL_SUBSCRIPTION__CREATE, &Method::POST) => {
                return channel_subscription__base::create::Create::run(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
            // Area for not existing routes.
            _ => {
                #[cfg(feature = "facilitate_non_automatic_functional_testing")]
                match (route, method) {
                    // Area for existing routes with not authorized user.
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING_, &Method::POST) => {
                        return application_user__authorization::check_nickname_for_existing::CheckNicknameForExisting::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING_, &Method::POST) => {
                        return application_user__authorization::check_email_for_existing::CheckEmailForExisting::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_FIRST_STEP_, &Method::POST) => {
                        return application_user__authorization::register_by_first_step::RegisterByFirstStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_SECOND_STEP_, &Method::POST) => {
                        return application_user__authorization::register_by_second_step::RegisterBySecondStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REGISTER_BY_LAST_STEP_, &Method::POST) => {
                        return application_user__authorization::register_by_last_step::RegisterByLastStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_REGISTER_, &Method::POST) => {
                        return application_user__authorization::send_email_for_register::SendEmailForRegister::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP_, &Method::POST) => {
                        return application_user__authorization::authorize_by_first_step::AuthorizeByFirstStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__AUTHORIZE_BY_LAST_STEP_, &Method::POST) => {
                        return application_user__authorization::authorize_by_last_step::AuthorizeByLastStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE_, &Method::POST) => {
                        return application_user__authorization::send_email_for_authorize::SendEmailForAuthorize::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP_, &Method::POST) => {
                        return application_user__authorization::reset_password_by_first_step::ResetPasswordByFirstStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP_, &Method::POST) => {
                        return application_user__authorization::reset_password_by_second_step::ResetPasswordBySecondStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP_, &Method::POST) => {
                        return application_user__authorization::reset_password_by_last_step::ResetPasswordByLastStep::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD_, &Method::POST) => {
                        return application_user__authorization::send_email_for_reset_password::SendEmailForResetPassword::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__REFRESH_APPLICATION_USER_ACCESS_TOKEN_, &Method::POST) => {
                        return application_user__authorization::refresh_application_user_access_token::RefreshApplicationUserAccessToken::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for existing routes with authorized user.
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE_, &Method::POST) => {
                        return application_user__authorization::deauthorize_from_one_device::DeauthorizeFromOneDevice::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE_, &Method::POST) => {
                        return application_user__authorization::deauthorize_from_all_devices::DeauthorizeFromAllDevices::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__CHANNEL__GET_ONE_BY_ID_, &Method::POST) => {
                        return channel__base::get_one_by_id::GetOneByID::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_, &Method::POST) => {
                        return channel__base::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_BY_SUBSCRIPTION_, &Method::POST) => {
                        return channel__base::get_many_by_subscription::GetManyBySubscription::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // GET functional.
                    (HttpRouteRegistry::VERSION_1__CHANNEL__GET_MANY_PUBLIC_BY_NAME_, &Method::POST) => {
                        return channel__base::get_many_public_by_name::GetManyPublicByName::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    (HttpRouteRegistry::VERSION_1__CHANNEL_SUBSCRIPTION__CREATE_, &Method::POST) => {
                        return channel_subscription__base::create::Create::run_(
                            pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                        ).await;
                    }
                    // Area for not existing routes.
                    _ => {}
                }

                return route_not_found::route_not_found(
                    pushable_environment_configuration, request, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, redis_connection_pool
                ).await;
            }
        }
    }
}

#[derive(Clone)]
enum PostgresqlConnectionPoolAggregator {
    LocalDevelopment {
        database_1_postgresql_connection_pool: PostgresqlConnectionPoolNoTls,
        database_2_postgresql_connection_pool: PostgresqlConnectionPoolNoTls
    }
}