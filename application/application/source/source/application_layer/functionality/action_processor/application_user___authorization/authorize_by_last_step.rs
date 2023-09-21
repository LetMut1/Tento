use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken4;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::incrementor::Incrementor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::Resource;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By3;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update6;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Precedent;

pub struct AuthorizeByLastStep;

impl AuthorizeByLastStep {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>, // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Id,
                },
            );
        }

        let is_valid_value = match Validator::<ApplicationUserAuthorizationToken_Value>::is_valid(&incoming.application_user_authorization_token_value) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        if !is_valid_value {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserAuthorizationToken_Value,
                },
            );
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming.application_user_device_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserDevice_Id,
                },
            );
        }

        let by_4 = By4 {
            application_user_id: incoming.application_user_id,
            application_user_device_id: &incoming.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: Resource::ConnectionPoolPostgresql {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken2>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let mut application_user_authorization_token_ = match application_user_authorization_token {
            Some(application_user_authorization_token__) => application_user_authorization_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.expires_at.0) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_4,
            )
            .await
            {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_AlreadyExpired),
                },
            );
        }

        if application_user_authorization_token_.value.0 != incoming.application_user_authorization_token_value.0 {
            if let Err(mut error) = Incrementor::<ApplicationUserAuthorizationToken_WrongEnterTriesQuantity>::increment(&mut application_user_authorization_token_.wrong_enter_tries_quantity) {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }

            if application_user_authorization_token_.wrong_enter_tries_quantity.0 < ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken4>::update(
                    database_2_postgresql_connection,
                    &Update6 {
                        application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                    },
                    &by_4,
                )
                .await
                {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            } else {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    &by_4,
                )
                .await
                {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(
                        Precedent::ApplicationUserAuthorizationToken_WrongValue {
                            application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                        },
                    ),
                },
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: Resource::ConnectionPoolPostgresql {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let is_exist = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_3(
            database_1_postgresql_connection,
            &By3 {
                application_user_id: incoming.application_user_id,
            },
        )
        .await
        {
            Ok(is_exist_) => is_exist_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        if !is_exist {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_NotFound),
                },
            );
        }

        let expires_at = match Generator::<ApplicationUserAccessToken_ExpiresAt>::generate() {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token = ApplicationUserAccessToken {
            id: Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user_id: incoming.application_user_id,
            application_user_device_id: Cow::Borrowed(&incoming.application_user_device_id),
            expires_at,
        };

        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token_id = &application_user_access_token.id;

        let application_user_access_refresh_token_obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();

        let application_user_access_refresh_token_expires_at = match Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate() {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_refresh_token_updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();
        // TODO  TRANZACTION
        let application_user_access_refresh_token_ = match application_user_access_refresh_token {
            Some(mut application_user_access_refresh_token__) => {
                application_user_access_refresh_token__.application_user_access_token_id = Cow::Borrowed(application_user_access_token_id);

                application_user_access_refresh_token__.obfuscation_value = application_user_access_refresh_token_obfuscation_value;

                application_user_access_refresh_token__.expires_at = application_user_access_refresh_token_expires_at;

                application_user_access_refresh_token__.updated_at = application_user_access_refresh_token_updated_at;

                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken1>::update(
                    database_2_postgresql_connection,
                    &Update2 {
                        application_user_access_token_id: application_user_access_refresh_token__.application_user_access_token_id.as_ref(),
                        application_user_access_refresh_token_obfuscation_value: &application_user_access_refresh_token__.obfuscation_value,
                        application_user_access_refresh_token_expires_at: application_user_access_refresh_token__.expires_at,
                        application_user_access_refresh_token_updated_at: application_user_access_refresh_token__.updated_at,
                    },
                    &by_4,
                )
                .await
                {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }

                application_user_access_refresh_token__
            }
            None => {
                let application_user_access_refresh_token__ = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create(
                    database_2_postgresql_connection,
                    Insert2 {
                        application_user_id: incoming.application_user_id,
                        application_user_device_id: &incoming.application_user_device_id,
                        application_user_access_token_id,
                        application_user_access_refresh_token_obfuscation_value,
                        application_user_access_refresh_token_expires_at,
                        application_user_access_refresh_token_updated_at,
                    },
                )
                .await
                {
                    Ok(application_user_access_refresh_token___) => application_user_access_refresh_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                application_user_access_refresh_token__
            }
        };

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }
        // TODO  TRANZACTION
        let application_user_access_token_encrypted = match FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(&application_user_access_token) {
            Ok(application_user_access_token_encrypted_) => application_user_access_token_encrypted_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_refresh_token_encrypted = match FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(&application_user_access_refresh_token_) {
            Ok(application_user_access_refresh_token_encrypted_) => application_user_access_refresh_token_encrypted_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserDevice>::create(
            database_1_postgresql_connection,
            Insert4 {
                application_user_device_id: incoming.application_user_device_id,
                application_user_id: incoming.application_user_id,
            },
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        };

        let outcoming = Outcoming {
            application_user_access_token_encrypted,
            application_user_access_refresh_token_encrypted,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
}