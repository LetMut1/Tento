use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken4;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken5;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::incrementor::Incrementor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By5;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update10;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update11;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterBySecondStep;

impl ActionProcessor<ApplicationUser__Authorization___RegisterBySecondStep> {
    pub async fn process<'a, T>(
        _environment_configuration: &'static EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;

        if !Validator::<ApplicationUser_Email>::is_valid(&incoming_.application_user_email)? {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Email,
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

        if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(&incoming_.application_user_registration_token_value)? {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserRegistrationToken_Value,
                },
            );
        }

        let by_5 = By5 {
            application_user_email: &incoming_.application_user_email,
            application_user_device_id: &incoming_.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken3>::find_1(
            database_2_postgresql_connection,
            &by_5,
        )
        .await?
        {
            Some(application_user_registration_token_) => application_user_registration_token_,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at.0) {
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_5,
            )
            .await?;

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired),
                },
            );
        }

        if application_user_registration_token.is_approved.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyApproved),
                },
            );
        }

        if application_user_registration_token.value.0 != incoming_.application_user_registration_token_value.0 {
            Incrementor::<ApplicationUserRegistrationToken_WrongEnterTriesQuantity>::increment(&mut application_user_registration_token.wrong_enter_tries_quantity)?;

            if application_user_registration_token.wrong_enter_tries_quantity.0 < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserRegistrationToken4>::update(
                    database_2_postgresql_connection,
                    &Update10 {
                        application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                    },
                    &by_5,
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    &by_5,
                )
                .await?;
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(
                        Precedent::ApplicationUserRegistrationToken_WrongValue {
                            application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                        },
                    ),
                },
            );
        }

        application_user_registration_token.is_approved = ApplicationUserRegistrationToken_IsApproved(true);

        PostgresqlRepository::<ApplicationUserRegistrationToken5>::update(
            database_2_postgresql_connection,
            &Update11 {
                application_user_registration_token_is_approved: application_user_registration_token.is_approved,
            },
            &by_5,
        )
        .await?;

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_empty(),
            },
        );
    }
}