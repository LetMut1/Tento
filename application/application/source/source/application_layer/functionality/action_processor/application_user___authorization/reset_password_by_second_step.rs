use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Update4;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Update5;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_second_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_second_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___ResetPasswordBySecondStep;

impl ActionProcessor<ApplicationUser__Authorization___ResetPasswordBySecondStep> {
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

        if !Validator::<ApplicationUserResetPasswordToken_Value>::is_valid(incoming_.application_user_reset_password_token_value.as_str())? {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }

        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user_id) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device_id.as_str()) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }

        let by_4 = By4 {
            application_user_id: incoming_.application_user_id,
            application_user_device_id: incoming_.application_user_device_id.as_str(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let mut application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_2(
            database_2_postgresql_connection,
            &by_4,
        )
        .await?
        {
            Some(application_user_reset_password_token_) => application_user_reset_password_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_NotFound)));
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.expires_at) {
            PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                database_2_postgresql_connection,
                &by_4,
            )
            .await?;

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired)));
        }

        if application_user_reset_password_token.is_approved {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyApproved)));
        }

        if application_user_reset_password_token.value != incoming_.application_user_reset_password_token_value {
            application_user_reset_password_token.wrong_enter_tries_quantity = application_user_reset_password_token.wrong_enter_tries_quantity
                .checked_add(1)
                .convert_out_of_range(Backtrace::new(line!(), file!()))?;

            if application_user_reset_password_token.wrong_enter_tries_quantity < ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_4(
                    database_2_postgresql_connection,
                    &Update4 {
                        application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                    },
                    &by_4,
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    &by_4,
                )
                .await?;
            }

            return Ok(
                Ok(
                    UnifiedReport::precedent(
                        Precedent::ApplicationUserResetPasswordToken_WrongValue {
                            application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                        },
                    ),
                ),
            );
        }

        application_user_reset_password_token.is_approved = true;

        PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_5(
            database_2_postgresql_connection,
            &Update5 {
                application_user_reset_password_token_is_approved: application_user_reset_password_token.is_approved,
            },
            &by_4,
        )
        .await?;

        return Ok(Ok(UnifiedReport::target_empty()));
    }
}
