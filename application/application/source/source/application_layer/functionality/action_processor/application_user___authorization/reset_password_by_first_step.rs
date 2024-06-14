use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user::By2;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::By1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Insert1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Update1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Update2;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_reset_password_token::Update3;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___ResetPasswordByFirstStep;

impl ActionProcessor<ApplicationUser__Authorization___ResetPasswordByFirstStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;

        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user_email.as_str())? {
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

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let application_user = PostgresqlRepository::<ApplicationUser>::find_4(
            &*database_1_postgresql_pooled_connection,
            By2 {
                application_user_email: incoming_.application_user_email.as_str(),
            },
        )
        .await?;

        let application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound)));
            }
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let (application_user_reset_password_token_value, application_user_reset_password_token_can_be_resent_from, application_user_reset_password_token_wrong_enter_tries_quantity, can_send) = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_1(
            database_2_postgresql_connection,
            By1 {
                application_user_id: application_user_.id,
                application_user_device_id: incoming_.application_user_device_id.as_str(),
            },
        )
        .await?
        {
            Some(mut application_user_reset_password_token) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.can_be_resent_from) {
                    application_user_reset_password_token.can_be_resent_from = Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate()?;

                    (
                        true, true,
                    )
                } else {
                    (
                        false, false,
                    )
                };

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.expires_at) || application_user_reset_password_token.is_approved {
                    application_user_reset_password_token.value = Generator::<ApplicationUserResetPasswordToken_Value>::generate();

                    application_user_reset_password_token.wrong_enter_tries_quantity = 0;

                    application_user_reset_password_token.is_approved = false;

                    application_user_reset_password_token.expires_at = Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate()?;

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_1(
                        database_2_postgresql_connection,
                        &Update1 {
                            application_user_reset_password_token_value: application_user_reset_password_token.value.as_str(),
                            application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                            application_user_reset_password_token_is_approved: application_user_reset_password_token.is_approved,
                            application_user_reset_password_token_expires_at: application_user_reset_password_token.expires_at,
                            application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
                        },
                        By1 {
                            application_user_id: application_user_.id,
                            application_user_device_id: incoming_.application_user_device_id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    if need_to_update_1 {
                        PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_2(
                            database_2_postgresql_connection,
                            &Update2 {
                                application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
                            },
                            By1 {
                                application_user_id: application_user_.id,
                                application_user_device_id: incoming_.application_user_device_id.as_str(),
                            },
                        )
                        .await?;
                    }

                    if need_to_update_2 {
                        PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_3(
                            database_2_postgresql_connection,
                            &Update3 {
                                application_user_reset_password_token_value: application_user_reset_password_token.value.as_str(),
                                application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                                application_user_reset_password_token_is_approved: application_user_reset_password_token.is_approved,
                                application_user_reset_password_token_expires_at: application_user_reset_password_token.expires_at,
                            },
                            By1 {
                                application_user_id: application_user_.id,
                                application_user_device_id: incoming_.application_user_device_id.as_str(),
                            },
                        )
                        .await?;
                    }
                }

                (
                    application_user_reset_password_token.value,
                    application_user_reset_password_token.can_be_resent_from,
                    application_user_reset_password_token.wrong_enter_tries_quantity,
                    can_send_,
                )
            }
            None => {
                let application_user_reset_password_token = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::create_1(
                    database_2_postgresql_connection,
                    Insert1 {
                        application_user_id: application_user_.id,
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                        application_user_reset_password_token_value: Generator::<ApplicationUserResetPasswordToken_Value>::generate(),
                        application_user_reset_password_token_wrong_enter_tries_quantity: 0,
                        application_user_reset_password_token_is_approved: false,
                        application_user_reset_password_token_expires_at:Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate()?,
                        application_user_reset_password_token_can_be_resent_from: Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate()?,
                    },
                )
                .await?;

                (
                    application_user_reset_password_token.value,
                    application_user_reset_password_token.can_be_resent_from,
                    application_user_reset_password_token.wrong_enter_tries_quantity,
                    true,
                )
            }
        };

        if can_send {
            EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
                environment_configuration,
                application_user_reset_password_token_value.as_str(),
                incoming_.application_user_email.as_str(),
                incoming_.application_user_device_id.as_str(),
            )?;
        }

        let outcoming = Outcoming {
            application_user_id: application_user_.id,
            verification_message_sent: can_send,
            application_user_reset_password_token_can_be_resent_from,
            application_user_reset_password_token_wrong_enter_tries_quantity,
            application_user_reset_password_token_wrong_enter_tries_quantity_limit: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
