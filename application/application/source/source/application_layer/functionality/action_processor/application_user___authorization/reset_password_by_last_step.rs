use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser4;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken4;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::incrementor::Incrementor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By3;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update15;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use crate::infrastructure_layer::functionality::service::resolver::CloudMessage;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::Precedent;

pub struct ResetPasswordByLastStep;

impl ResetPasswordByLastStep {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, ErrorAuditor_>
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

        let is_valid_value = match Validator::<ApplicationUserResetPasswordToken_Value>::is_valid(&incoming_.application_user_reset_password_token_value) {
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
                    invalid_argument: InvalidArgument::ApplicationUserResetPasswordToken_Value,
                },
            );
        }

        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Id,
                },
            );
        }

        if !Validator::<ApplicationUser_Password>::is_valid_part_1(&incoming_.application_user_password) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
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

        let by_4 = By4 {
            application_user_id: incoming_.application_user_id,
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

        let application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken3>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
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

        let mut application_user_reset_password_token_ = match application_user_reset_password_token {
            Some(application_user_reset_password_token__) => application_user_reset_password_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.expires_at.0) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
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
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired),
                },
            );
        }

        if !application_user_reset_password_token_.is_approved.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_IsNotApproved),
                },
            );
        }

        if application_user_reset_password_token_.value.0 != incoming_.application_user_reset_password_token_value.0 {
            if let Err(mut error) = Incrementor::<ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>::increment(&mut application_user_reset_password_token_.wrong_enter_tries_quantity) {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }

            if application_user_reset_password_token_.wrong_enter_tries_quantity.0 < ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken4>::update(
                    database_2_postgresql_connection,
                    &Update15 {
                        application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token_.wrong_enter_tries_quantity,
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
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
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
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_WrongValue),
                },
            );
        }

        let by_3 = By3 {
            application_user_id: incoming_.application_user_id,
        };

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

        let application_user = match PostgresqlRepository::<ApplicationUser4>::find_1(
            database_1_postgresql_connection,
            &by_3,
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

        let mut application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUser_NotFound),
                    },
                );
            }
        };

        if !Validator::<ApplicationUser_Password>::is_valid_part_2(
            &incoming_.application_user_password,
            &application_user_.email,
            &application_user_.nickname,
        ) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
                },
            );
        }

        application_user_.password_hash = match Encoder::<ApplicationUser_Password>::encode(&incoming_.application_user_password) {
            Ok(password_hash_) => password_hash_,
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUser4>::update(
            database_1_postgresql_connection,
            &Update1 {
                application_user_password_hash: &application_user_.password_hash,
            },
            &by_3,
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
            &*database_2_postgresql_pooled_connection,
            &by_3,
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

        Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_empty(),
            },
        );
    }
}
