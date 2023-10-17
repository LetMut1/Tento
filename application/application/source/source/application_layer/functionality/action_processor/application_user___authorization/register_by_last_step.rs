use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
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
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By5;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update10;
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
use crate::application_layer::functionality::action_processor::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterByLastStep;

impl ActionProcessor<ApplicationUser__Authorization___RegisterByLastStep> {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor_>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    ErrorAuditor_::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if !Validator::<ApplicationUser_Password>::is_valid(
            &incoming_.application_user_password,
            &incoming_.application_user_email,
            &incoming_.application_user_nickname,
        ) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
                },
            );
        }

        if !Validator::<ApplicationUser_Nickname>::is_valid(&incoming_.application_user_nickname) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Nickname,
                },
            );
        }

        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&incoming_.application_user_email) {
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

        let is_valid_value = match Validator::<ApplicationUserRegistrationToken_Value>::is_valid(&incoming_.application_user_registration_token_value) {
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

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming_.application_user_device_id) {
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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
                application_user_nickname: &incoming_.application_user_nickname,
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
                application_user_email: &incoming_.application_user_email,
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
            application_user_email: &incoming_.application_user_email,
            application_user_device_id: &incoming_.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.expires_at.0) {
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

        if !application_user_registration_token_.is_approved.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_IsNotApproved),
                },
            );
        }

        if application_user_registration_token_.value.0 != incoming_.application_user_registration_token_value.0 {
            if let Err(mut error) = Incrementor::<ApplicationUserRegistrationToken_WrongEnterTriesQuantity>::increment(&mut application_user_registration_token_.wrong_enter_tries_quantity) {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            };

            if application_user_registration_token_.wrong_enter_tries_quantity.0 < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken4>::update(
                    database_2_postgresql_connection,
                    &Update10 {
                        application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token_.wrong_enter_tries_quantity,
                    },
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

        let application_user_password_hash = match Encoder::<ApplicationUser_Password>::encode(&incoming_.application_user_password) {
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

        let application_user = match PostgresqlRepository::<ApplicationUser<'_>>::create(
            database_1_postgresql_connection,
            Insert1 {
                application_user_email: incoming_.application_user_email,
                application_user_nickname: incoming_.application_user_nickname,
                application_user_password_hash,
            },
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

        let application_user_device = match PostgresqlRepository::<ApplicationUserDevice>::create(
            database_1_postgresql_connection,
            Insert4 {
                application_user_device_id: incoming_.application_user_device_id,
                application_user_id: application_user.id,
            },
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

        let application_user_access_token = ApplicationUserAccessToken {
            id: Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user_id: application_user.id,
            application_user_device_id: Cow::Borrowed(&application_user_device.id),
            expires_at: application_user_acces_token_expires_at,
        };

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
        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create(
            database_2_postgresql_connection,
            Insert2 {
                application_user_id: application_user.id,
                application_user_device_id: &application_user_device.id,
                application_user_access_token_id: &application_user_access_token.id,
                application_user_access_refresh_token_obfuscation_value: Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate(),
                application_user_access_refresh_token_expires_at,
                application_user_access_refresh_token_updated_at: Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate(),
            },
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
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}
