use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_4;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_5;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use crate::infrastructure_layer::functionality::service::macro_rules::r#enum;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Serialize;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming
    ) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, ErrorAuditor>
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

        let is_valid_value = match Validator::<ApplicationUserRegistrationToken_Value>::is_valid(&incoming.application_user_registration_token_value) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_value {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserRegistrationToken_Value });
        }

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

        let application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken_3>::find_1(
            database_2_postgresql_connection,
            &incoming.application_user_email,
            &incoming.application_user_device_id
        ).await {
            Ok(application_user_registration_token_) => application_user_registration_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut application_user_registration_token_ = match application_user_registration_token {
            Some(application_user_registration_token__) => application_user_registration_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound)
                    }
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.get_expires_at().get()) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                database_2_postgresql_connection,
                &incoming.application_user_email,
                &incoming.application_user_device_id
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired)
                }
            );
        }

        if application_user_registration_token_.get_is_approved().get() {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyApproved)
                }
            );
        }

        if application_user_registration_token_.get_value().get() != incoming.application_user_registration_token_value.get() {
            let application_user_registration_token_wrong_enter_tries_quantity = match application_user_registration_token_.get_wrong_enter_tries_quantity().get()
                .checked_add(1) {
                Some(application_user_registration_token_wrong_enter_tries_quantity_) => application_user_registration_token_wrong_enter_tries_quantity_,
                None => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::create_out_of_range(),
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            if application_user_registration_token_wrong_enter_tries_quantity <= ApplicationUserRegistrationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                application_user_registration_token_.set_wrong_enter_tries_quantity(
                    ApplicationUserRegistrationToken_WrongEnterTriesQuantity::new(application_user_registration_token_wrong_enter_tries_quantity)
                );

                if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken_4>::update(
                    database_2_postgresql_connection,
                    &application_user_registration_token_,
                    &incoming.application_user_email,
                    &incoming.application_user_device_id
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    &incoming.application_user_email,
                    &incoming.application_user_device_id
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_WrongValue)
                }
            );
        }

        application_user_registration_token_.set_is_approved(ApplicationUserRegistrationToken_IsApproved::new(true));

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken_5>::update(
            database_2_postgresql_connection,
            &application_user_registration_token_,
            &incoming.application_user_email,
            &incoming.application_user_device_id
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(InvalidArgumentResult::Ok { subject: UnifiedReport::empty() });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: ApplicationUser_Email,
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_registration_token_value: ApplicationUserRegistrationToken_Value
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_WrongValue
    }
);