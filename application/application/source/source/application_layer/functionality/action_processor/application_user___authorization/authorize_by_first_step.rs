use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::control_type::TokioBlockingTask;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By1;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By2;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert3;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update3;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update4;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update5;
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
use crate::application_layer::functionality::action_processor::ActionProcessor;
use tokio_postgres::Socket;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___AuthorizeByFirstStep;

impl ActionProcessor<ApplicationUser__Authorization___AuthorizeByFirstStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
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

        if !Validator::<ApplicationUser_Password>::is_valid_part_1(incoming_.application_user_password.as_str()) {
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

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let (application_user_id, application_user_email, application_user_nickname, application_user_password_hash) =
        if Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user_email_or_application_user_nickname.as_str())? {
            let application_user_ = PostgresqlRepository::<ApplicationUser>::find_3(
                database_1_postgresql_connection,
                &By2 {
                    application_user_email: incoming_.application_user_email_or_application_user_nickname.as_str(),
                },
            )
            .await?;

            let application_user__ = match application_user_ {
                Some(application_user___) => application_user___,
                None => {
                    return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
                }
            };

            (
                application_user__.id,
                incoming_.application_user_email_or_application_user_nickname,
                application_user__.nickname,
                application_user__.password_hash,
            )
        } else {
            if Validator::<ApplicationUser_Nickname>::is_valid(incoming_.application_user_email_or_application_user_nickname.as_str()) {
                let application_user_ = PostgresqlRepository::<ApplicationUser>::find_2(
                    database_1_postgresql_connection,
                    &By1 {
                        application_user_nickname: incoming_.application_user_email_or_application_user_nickname.as_str(),
                    },
                )
                .await?;

                let application_user__ = match application_user_ {
                    Some(application_user___) => application_user___,
                    None => {
                        return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
                    }
                };

                (
                    application_user__.id,
                    application_user__.email,
                    incoming_.application_user_email_or_application_user_nickname,
                    application_user__.password_hash,
                )
            } else {
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
        };

        if !Validator::<ApplicationUser_Password>::is_valid_part_2(
            incoming_.application_user_password.as_str(),
            application_user_email.as_str(),
            application_user_nickname.as_str(),
        ) {
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

        let join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
            move || -> _ {
                return Encoder::<ApplicationUser_Password>::is_valid(
                    incoming_.application_user_password.as_str(),
                    application_user_password_hash.as_str(),
                );
            }
        );

        if !join_handle.await.convert(Backtrace::new(line!(), file!()))?? {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
        }

        let by_4 = By4 {
            application_user_id,
            application_user_device_id: incoming_.application_user_device_id.as_str(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let (application_user_authorization_token_value, application_user_authorization_token_can_be_resent_from, application_user_authorization_token_wrong_enter_tries_quantity, can_send) = match PostgresqlRepository::<ApplicationUserAuthorizationToken>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await?
        {
            Some(mut application_user_authorization_token) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.can_be_resent_from) {
                    application_user_authorization_token.can_be_resent_from = Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?;

                    (
                        true, true,
                    )
                } else {
                    (
                        false, false,
                    )
                };

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.expires_at) {
                    application_user_authorization_token.value = Generator::<ApplicationUserAuthorizationToken_Value>::generate();

                    application_user_authorization_token.wrong_enter_tries_quantity = 0;

                    application_user_authorization_token.expires_at = Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate()?;

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_1(
                        database_2_postgresql_connection,
                        &Update3 {
                            application_user_authorization_token_value: application_user_authorization_token.value.as_str(),
                            application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token.wrong_enter_tries_quantity,
                            application_user_authorization_token_expires_at: application_user_authorization_token.expires_at,
                            application_user_authorization_token_can_be_resent_from: application_user_authorization_token.can_be_resent_from,
                        },
                        &by_4,
                    )
                    .await?;
                } else {
                    if need_to_update_1 {
                        PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_3(
                            database_2_postgresql_connection,
                            &Update5 {
                                application_user_authorization_token_can_be_resent_from: application_user_authorization_token.can_be_resent_from,
                            },
                            &by_4,
                        )
                        .await?;
                    }

                    if need_to_update_2 {
                        PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_2(
                            database_2_postgresql_connection,
                            &Update4 {
                                application_user_authorization_token_value: application_user_authorization_token.value.as_str(),
                                application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token.wrong_enter_tries_quantity,
                                application_user_authorization_token_expires_at: application_user_authorization_token.expires_at,
                            },
                            &by_4,
                        )
                        .await?;
                    }
                }

                (
                    application_user_authorization_token.value,
                    application_user_authorization_token.can_be_resent_from,
                    application_user_authorization_token.wrong_enter_tries_quantity,
                    can_send_,
                )
            }
            None => {
                let application_user_authorization_token = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::create(
                    database_2_postgresql_connection,
                    Insert3 {
                        application_user_id,
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                        application_user_authorization_token_value: Generator::<ApplicationUserAuthorizationToken_Value>::generate(),
                        application_user_authorization_token_wrong_enter_tries_quantity: 0,
                        application_user_authorization_token_expires_at: Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate()?,
                        application_user_authorization_token_can_be_resent_from: Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?,
                    },
                )
                .await?;

                (
                    application_user_authorization_token.value,
                    application_user_authorization_token.can_be_resent_from,
                    application_user_authorization_token.wrong_enter_tries_quantity,
                    true,
                )
            }
        };

        if can_send {
            EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                environment_configuration,
                application_user_authorization_token_value.as_str(),
                application_user_email.as_str(),
                incoming_.application_user_device_id.as_str(),
            )?;
        }

        let outcoming = Outcoming {
            application_user_id,
            verification_message_sent: can_send,
            application_user_authorization_token_can_be_resent_from,
            application_user_authorization_token_wrong_enter_tries_quantity,
            application_user_authorization_token_wrong_enter_tries_quantity_limit: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}