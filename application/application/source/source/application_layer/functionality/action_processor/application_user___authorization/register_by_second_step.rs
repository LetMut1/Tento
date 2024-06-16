pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterBySecondStep;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::ApplicationUser_Email,
            application_user_device::ApplicationUserDevice_Id,
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_Value,
                ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
                ErrorConverter,
                OptionConverter,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
            void::Void,
        },
        functionality::{
            repository::postgresql::{
                application_user_registration_token::{
                    By1,
                    Update4,
                    Update5,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::{
                unix_time::UnixTime,
                ExpirationTimeChecker,
            },
        },
    },
};
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::{
    Incoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor<ApplicationUser__Authorization___RegisterBySecondStep> {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Void, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;
        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user_email.as_str())? {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device_id.as_str()) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(incoming_.application_user_registration_token_value.as_str())? {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_2(
            database_2_postgresql_connection,
            By1 {
                application_user_email: incoming_.application_user_email.as_str(),
                application_user_device_id: incoming_.application_user_device_id.as_str(),
            },
        )
        .await?
        {
            Some(application_user_registration_token_) => application_user_registration_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(
                    Precedent::ApplicationUserRegistrationToken_NotFound,
                )));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at) {
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                database_2_postgresql_connection,
                By1 {
                    application_user_email: incoming_.application_user_email.as_str(),
                    application_user_device_id: incoming_.application_user_device_id.as_str(),
                },
            )
            .await?;
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_AlreadyExpired,
            )));
        }
        if application_user_registration_token.is_approved {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_AlreadyApproved,
            )));
        }
        if application_user_registration_token.value != incoming_.application_user_registration_token_value {
            application_user_registration_token.wrong_enter_tries_quantity =
                application_user_registration_token.wrong_enter_tries_quantity.checked_add(1).convert_out_of_range(Backtrace::new(line!(), file!()))?;
            if application_user_registration_token.wrong_enter_tries_quantity < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserRegistrationToken>::update_4(
                    database_2_postgresql_connection,
                    Update4 {
                        application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                    },
                    By1 {
                        application_user_email: incoming_.application_user_email.as_str(),
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                    },
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1 {
                        application_user_email: incoming_.application_user_email.as_str(),
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                    },
                )
                .await?;
            }
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_WrongValue {
                    application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                },
            )));
        }
        application_user_registration_token.is_approved = true;
        PostgresqlRepository::<ApplicationUserRegistrationToken>::update_5(
            database_2_postgresql_connection,
            Update5 {
                application_user_registration_token_is_approved: application_user_registration_token.is_approved,
            },
            By1 {
                application_user_email: incoming_.application_user_email.as_str(),
                application_user_device_id: incoming_.application_user_device_id.as_str(),
            },
        )
        .await?;
        return Ok(Ok(UnifiedReport::target_empty()));
    }
}
