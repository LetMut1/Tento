use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken4;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::incrementor::Incrementor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::application_user___postgresql_repository::Insert as ApplicationUserInsert;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token___postgresql_repository::Insert as ApplicationUserAccessRefreshTokenInsert;
use crate::infrastructure_layer::functionality::repository::application_user_device___postgresql_repository::Insert as ApplicationUserDeviceInsert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::By1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::By2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::By5;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::macro_rules::r#enum;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if !Validator::<ApplicationUser_Password>::is_valid(&incoming.application_user_password) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
                },
            );
        }

        if !Validator::<ApplicationUser_Nickname>::is_valid(&incoming.application_user_nickname) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Nickname,
                },
            );
        }

        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&incoming.application_user_email) {
            Ok(is_valid_email_) => is_valid_email_,
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

        if !is_valid_email {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Email,
                },
            );
        }

        let is_valid_value = match Validator::<ApplicationUserRegistrationToken_Value>::is_valid(&incoming.application_user_registration_token_value) {
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
                    invalid_argument: InvalidArgument::ApplicationUserRegistrationToken_Value,
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

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
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

        let is_exist_1 = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
            database_1_postgresql_connection,
            &By1 {
                application_user_nickname: &incoming.application_user_nickname,
            },
        )
        .await
        {
            Ok(is_exist_1_) => is_exist_1_,
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

        if is_exist_1 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_NicknameAlreadyExist),
                },
            );
        }

        let is_exist_2 = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            database_1_postgresql_connection,
            &By2 {
                application_user_email: &incoming.application_user_email,
            },
        )
        .await
        {
            Ok(is_exist_2_) => is_exist_2_,
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

        if is_exist_2 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_EmailAlreadyExist),
                },
            );
        }

        let by_5 = By5 {
            application_user_email: &incoming.application_user_email,
            application_user_device_id: &incoming.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
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

        let application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken3>::find_1(
            database_2_postgresql_connection,
            &by_5,
        )
        .await
        {
            Ok(application_user_registration_token_) => application_user_registration_token_,
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

        let mut application_user_registration_token_ = match application_user_registration_token {
            Some(application_user_registration_token__) => application_user_registration_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.get_expires_at().get()) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_5,
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
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired),
                },
            );
        }

        if !application_user_registration_token_.get_is_approved().get() {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_IsNotApproved),
                },
            );
        }

        if application_user_registration_token_.get_value().get() != incoming.application_user_registration_token_value.get() {
            if let Err(mut error) = Incrementor::<ApplicationUserRegistrationToken_WrongEnterTriesQuantity>::increment(application_user_registration_token_.get_wrong_enter_tries_quantity_()) {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            };

            if application_user_registration_token_.get_wrong_enter_tries_quantity().get() <= ApplicationUserRegistrationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken4>::update(
                    database_2_postgresql_connection,
                    &application_user_registration_token_,
                    &by_5,
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
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    &by_5,
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
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_WrongValue),
                },
            );
        }

        let application_user_password_hash = match Encoder::<ApplicationUser_Password>::encode(&incoming.application_user_password) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
            database_2_postgresql_connection,
            &by_5,
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

        let application_user_insert = ApplicationUserInsert {
            application_user_email: incoming.application_user_email,
            application_user_nickname: incoming.application_user_nickname,
            application_user_password_hash,
        };

        let application_user = match PostgresqlRepository::<ApplicationUser<'_>>::create(
            database_1_postgresql_connection,
            application_user_insert,
        )
        .await
        {
            Ok(application_user_) => application_user_,
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

        let application_user_device_insert = ApplicationUserDeviceInsert {
            application_user_device_id: incoming.application_user_device_id,
            application_user_id: application_user.get_id(),
        };

        let application_user_device = match PostgresqlRepository::<ApplicationUserDevice>::create(
            database_1_postgresql_connection,
            application_user_device_insert,
        )
        .await
        {
            Ok(application_user_device_) => application_user_device_,
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

        let application_user_acces_token_expires_at = match Generator::<ApplicationUserAccessToken_ExpiresAt>::generate() {
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

        let application_user_access_token = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user.get_id(),
            Cow::Borrowed(application_user_device.get_id()),
            application_user_acces_token_expires_at,
        );

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

        // TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
        let application_user_access_refresh_token_insert = ApplicationUserAccessRefreshTokenInsert {
            application_user_id: application_user.get_id(),
            application_user_device_id: application_user_device.get_id(),
            application_user_access_token_id: application_user_access_token.get_id(),
            application_user_access_refresh_token_obfuscation_value: Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate(),
            application_user_access_refresh_token_expires_at,
            application_user_access_refresh_token_updated_at: Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate(),
        };

        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create(
            database_2_postgresql_connection,
            application_user_access_refresh_token_insert,
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

        let application_user_access_refresh_token_encrypted = match FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(&application_user_access_refresh_token) {
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

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_nickname: ApplicationUser_Nickname,
    application_user_password: ApplicationUser_Password,
    application_user_email: ApplicationUser_Email,
    application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NicknameAlreadyExist,
        CommonPrecedent::ApplicationUser_EmailAlreadyExist,
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_IsNotApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_WrongValue,
    }
);
