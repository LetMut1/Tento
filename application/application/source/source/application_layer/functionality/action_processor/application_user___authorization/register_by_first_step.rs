use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::{
                ApplicationUser,
                ApplicationUser_Email,
            },
            application_user_device::ApplicationUserDevice_Id,
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_CanBeResentFrom,
                ApplicationUserRegistrationToken_ExpiresAt,
                ApplicationUserRegistrationToken_Value,
                ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            email_sender::EmailSender,
            generator::Generator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Backtrace,
            },
            control_type::{
                ApplicationUser__Authorization___RegisterByFirstStep,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::{
                AlternativeWorkflow,
                OptionConverter,
                ResultConverter,
            }
        },
        functionality::{
            repository::postgresql::{
                application_user::By2,
                application_user_registration_token::{
                    By1,
                    Insert1,
                    Update1,
                    Update2,
                    Update3,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::ExpirationTimeChecker,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::{
    Incoming,
    Outcoming,
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
impl ActionProcessor<ApplicationUser__Authorization___RegisterByFirstStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, AlternativeWorkflow>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user__email.as_str())? {
            return Err(
                AlternativeWorkflow::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    )
                )
            );
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device__id.as_str()) {
            return Err(
                AlternativeWorkflow::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    )
                )
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            &*database_1_postgresql_pooled_connection,
            By2 {
                application_user__email: incoming_.application_user__email.as_str(),
            },
        )
        .await?
        {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_EmailAlreadyExist));
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let (
            application_user_registration_token__value,
            application_user_registration_token__can_be_resent_from,
            application_user_registration_token__wrong_enter_tries_quantity,
            can_send,
        ) = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_1(
            database_2_postgresql_connection,
            By1 {
                application_user__email: incoming_.application_user__email.as_str(),
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(mut application_user_registration_token) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.can_be_resent_from) {
                    application_user_registration_token.can_be_resent_from = Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate()?;
                    (
                        true,
                        true,
                    )
                } else {
                    (
                        false,
                        false,
                    )
                };
                let need_to_update_2 =
                    if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at) || application_user_registration_token.is_approved {
                        application_user_registration_token.value = Generator::<ApplicationUserRegistrationToken_Value>::generate();
                        application_user_registration_token.wrong_enter_tries_quantity = 0;
                        application_user_registration_token.is_approved = false;
                        application_user_registration_token.expires_at = Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate()?;
                        true
                    } else {
                        false
                    };
                if need_to_update_1 && need_to_update_2 {
                    PostgresqlRepository::<ApplicationUserRegistrationToken>::update_1(
                        database_2_postgresql_connection,
                        Update1 {
                            application_user_registration_token__value: application_user_registration_token.value.as_str(),
                            application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                            application_user_registration_token__is_approved: application_user_registration_token.is_approved,
                            application_user_registration_token__expires_at: application_user_registration_token.expires_at,
                            application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
                        },
                        By1 {
                            application_user__email: incoming_.application_user__email.as_str(),
                            application_user_device__id: incoming_.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    if need_to_update_1 {
                        PostgresqlRepository::<ApplicationUserRegistrationToken>::update_2(
                            database_2_postgresql_connection,
                            Update2 {
                                application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
                            },
                            By1 {
                                application_user__email: incoming_.application_user__email.as_str(),
                                application_user_device__id: incoming_.application_user_device__id.as_str(),
                            },
                        )
                        .await?;
                    }
                    if need_to_update_2 {
                        PostgresqlRepository::<ApplicationUserRegistrationToken>::update_3(
                            database_2_postgresql_connection,
                            Update3 {
                                application_user_registration_token__value: application_user_registration_token.value.as_str(),
                                application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                                application_user_registration_token__is_approved: application_user_registration_token.is_approved,
                                application_user_registration_token__expires_at: application_user_registration_token.expires_at,
                            },
                            By1 {
                                application_user__email: incoming_.application_user__email.as_str(),
                                application_user_device__id: incoming_.application_user_device__id.as_str(),
                            },
                        )
                        .await?;
                    }
                }
                (
                    application_user_registration_token.value,
                    application_user_registration_token.can_be_resent_from,
                    application_user_registration_token.wrong_enter_tries_quantity,
                    can_send_,
                )
            }
            None => {
                let application_user_registration_token = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::create_1(
                    database_2_postgresql_connection,
                    Insert1 {
                        application_user__email: incoming_.application_user__email.as_str(),
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                        application_user_registration_token__value: Generator::<ApplicationUserRegistrationToken_Value>::generate(),
                        application_user_registration_token__wrong_enter_tries_quantity: 0,
                        application_user_registration_token__is_approved: false,
                        application_user_registration_token__expires_at: Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate()?,
                        application_user_registration_token__can_be_resent_from: Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate()?,
                    },
                )
                .await?;
                (
                    application_user_registration_token.value,
                    application_user_registration_token.can_be_resent_from,
                    application_user_registration_token.wrong_enter_tries_quantity,
                    true,
                )
            }
        };
        if can_send {
            EmailSender::<ApplicationUserRegistrationToken<'_>>::send(
                environment_configuration,
                application_user_registration_token__value.as_str(),
                incoming_.application_user__email.as_str(),
                incoming_.application_user_device__id.as_str(),
            )?;
        }
        let outcoming = Outcoming {
            verification_message_sent: can_send,
            application_user_registration_token__can_be_resent_from,
            application_user_registration_token__wrong_enter_tries_quantity,
            application_user_registration_token__wrong_enter_tries_quantity_limit: ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
