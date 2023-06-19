use crate::application_layer::data::common_precedent::ActionProcessorResult;
use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_1;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_IsApproved;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_3;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming
    ) -> Result<InvalidArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&incoming.application_user_email) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_email {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Email });
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming.application_user_device_id) {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user = match PostgresqlRepository::<ApplicationUser_3>::find_2(
            &*database_1_postgresql_pooled_connection,
            &incoming.application_user_email
        ).await {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: ActionProcessorResult::Precedent {
                            precedent: CommonPrecedent::ApplicationUser_NotFound
                        }
                    }
                );
            }
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken_1>::find_1(
            database_2_postgresql_connection,
            application_user_.get_id(),
            &incoming.application_user_device_id
        ).await {
            Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (application_user_reset_password_token_aggregator, can_send) = match application_user_reset_password_token {
            Some(mut application_user_reset_password_token_) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(
                    application_user_reset_password_token_.get_can_be_resent_from().get()
                ) {
                    let application_user_reset_password_token_can_be_resent_from = match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
                        Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_reset_password_token_.set_can_be_resent_from(application_user_reset_password_token_can_be_resent_from);

                    (true, true)
                } else {
                    (false, false)
                };

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.get_expires_at().get())
                    || application_user_reset_password_token_.get_is_approved().get() {
                    let application_user_reset_password_token_expires_at = match Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate() {
                        Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_reset_password_token_
                        .set_value(Generator::<ApplicationUserResetPasswordToken_Value>::generate())
                        .set_wrong_enter_tries_quantity(ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(0))
                        .set_is_approved(ApplicationUserResetPasswordToken_IsApproved::new(false))
                        .set_expires_at(application_user_reset_password_token_expires_at);

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken_1>::update(
                        database_2_postgresql_connection,
                        &application_user_reset_password_token_,
                        application_user_.get_id(),
                        &incoming.application_user_device_id
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                } else {
                    if need_to_update_1 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken_2>::update(
                            database_2_postgresql_connection,
                            &application_user_reset_password_token_,
                            application_user_.get_id(),
                            &incoming.application_user_device_id
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }

                    if need_to_update_2 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken_3>::update(
                            database_2_postgresql_connection,
                            &application_user_reset_password_token_,
                            application_user_.get_id(),
                            &incoming.application_user_device_id
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }

                (
                    ApplicationUserResetPasswordToken_Aggregator::Second {
                        application_user_reset_password_token: application_user_reset_password_token_
                    },
                    can_send_
                )
            }
            None => {
                let application_user_reset_password_token_expires_at = match Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate() {
                    Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user_reset_password_token_can_be_resent_from = match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
                    Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_id: application_user_.get_id(),
                    application_user_device_id: Cow::Borrowed(&incoming.application_user_device_id),
                    application_user_reset_password_token_value: Generator::<ApplicationUserResetPasswordToken_Value>::generate(),
                    application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(0),
                    application_user_reset_password_token_is_approved: ApplicationUserResetPasswordToken_IsApproved::new(false),
                    application_user_reset_password_token_expires_at,
                    application_user_reset_password_token_can_be_resent_from
                };

                let application_user_reset_password_token_ = match PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::create(
                    database_2_postgresql_connection,
                    insert
                ).await {
                    Ok(application_user_reset_password_token___) => application_user_reset_password_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                (
                    ApplicationUserResetPasswordToken_Aggregator::First {
                        application_user_reset_password_token: application_user_reset_password_token_
                    },
                    true
                )
            }
        };

        if can_send {
            let application_user_reset_password_token_value = match application_user_reset_password_token_aggregator {
                ApplicationUserResetPasswordToken_Aggregator::First { application_user_reset_password_token: ref application_user_reset_password_token_} => application_user_reset_password_token_.get_value(),
                ApplicationUserResetPasswordToken_Aggregator::Second { application_user_reset_password_token: ref application_user_reset_password_token_} => application_user_reset_password_token_.get_value(),
            };

            if let Err(mut error) = EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
                environment_configuration,
                application_user_reset_password_token_value,
                &incoming.application_user_email,
                &incoming.application_user_device_id
            ) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

        let application_user_reset_password_token_can_be_resent_from = match application_user_reset_password_token_aggregator {
            ApplicationUserResetPasswordToken_Aggregator::First { application_user_reset_password_token: ref application_user_reset_password_token_} => application_user_reset_password_token_.get_can_be_resent_from(),
            ApplicationUserResetPasswordToken_Aggregator::Second { application_user_reset_password_token: ref application_user_reset_password_token_} => application_user_reset_password_token_.get_can_be_resent_from(),
        };

        let outcoming = Outcoming {
            application_user_id: application_user_.get_id(),
            verification_message_sent: can_send,
            application_user_reset_password_token_can_be_resent_from
        };

        return Ok(InvalidArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: ApplicationUser_Email,
    application_user_device_id: ApplicationUserDevice_Id
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_id: ApplicationUser_Id,
    verification_message_sent: bool,
    application_user_reset_password_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom
}

enum ApplicationUserResetPasswordToken_Aggregator<'a> {
    First {
        application_user_reset_password_token: ApplicationUserResetPasswordToken<'a>
    },
    Second {
        application_user_reset_password_token: ApplicationUserResetPasswordToken_1
    }
}