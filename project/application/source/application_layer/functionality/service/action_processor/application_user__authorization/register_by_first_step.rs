use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_1;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_2;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::functionality::service::application_user_registration_token__expiration_time_resolver::ApplicationUserRegistrationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_token__sending_opportunity_resolver::ApplicationUserRegistrationToken_SendingOpportunityResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user_registration_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
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
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(incoming.application_user_email.as_str()) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_email {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Email });
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device_id.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
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

        let is_exist_2 = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            &*database_1_postgresql_pooled_connection,
            incoming.application_user_email.as_str()
        ).await {
            Ok(is_exist_2_) => is_exist_2_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if is_exist_2 {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_EmailAlreadyExist
                    }
                }
            );
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

        let application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken_1>::find_1(
            database_2_postgresql_connection,
            incoming.application_user_email.as_str(),
            incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_registration_token_) => application_user_registration_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (application_user_registration_token_aggregator, can_send) = match application_user_registration_token {
            Some(mut application_user_registration_token_) => {
                let (can_send_, need_to_update_1) = if ApplicationUserRegistrationToken_SendingOpportunityResolver::can_send(
                    &application_user_registration_token_
                ) {
                    let application_user_registration_token_can_be_resent_from = match Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate() {
                        Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_registration_token_.set_can_be_resent_from(application_user_registration_token_can_be_resent_from);

                    (true, true)
                } else {
                    (false, false)
                };

                let need_to_update_2 = if ApplicationUserRegistrationToken_ExpirationTimeResolver::is_expired(&application_user_registration_token_)
                    || application_user_registration_token_.get_is_approved() {
                    let application_user_registration_token_expires_at = match Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate() {
                        Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_registration_token_
                        .set_value(Generator::<ApplicationUserRegistrationToken_Value>::generate())
                        .set_wrong_enter_tries_quantity(0)
                        .set_is_approved(false)
                        .set_expires_at(application_user_registration_token_expires_at);

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken_1>::update(
                        database_2_postgresql_connection,
                        &application_user_registration_token_,
                        incoming.application_user_email.as_str(),
                        incoming.application_user_device_id.as_str()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                } else {
                    if need_to_update_1 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken_2>::update(
                            database_2_postgresql_connection,
                            &application_user_registration_token_,
                            incoming.application_user_email.as_str(),
                            incoming.application_user_device_id.as_str()
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }

                    if need_to_update_2 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken_3>::update(
                            database_2_postgresql_connection,
                            &application_user_registration_token_,
                            incoming.application_user_email.as_str(),
                            incoming.application_user_device_id.as_str()
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }

                (
                    ApplicationUserRegistrationToken_Aggregator::Second {
                        application_user_registration_token: application_user_registration_token_
                    },
                    can_send_
                )
            }
            None => {
                let application_user_registration_token_expires_at = match Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate() {
                    Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user_registration_token_can_be_resent_from = match Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate() {
                    Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_email: Cow::Borrowed(incoming.application_user_email.as_str()),
                    application_user_device_id: Cow::Borrowed(incoming.application_user_device_id.as_str()),
                    application_user_registration_token_value: Generator::<ApplicationUserRegistrationToken_Value>::generate(),
                    application_user_registration_token_wrong_enter_tries_quantity: 0,
                    application_user_registration_token_is_approved: false,
                    application_user_registration_token_expires_at,
                    application_user_registration_token_can_be_resent_from
                };

                let application_user_registration_token_ = match PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::create(
                    database_2_postgresql_connection, insert
                ).await {
                    Ok(application_user_registration_token___) => application_user_registration_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                (
                    ApplicationUserRegistrationToken_Aggregator::First {
                        application_user_registration_token: application_user_registration_token_
                    },
                    true
                )
            }
        };

        if can_send {
            let application_user_registration_token_value = match application_user_registration_token_aggregator {
                ApplicationUserRegistrationToken_Aggregator::First { application_user_registration_token: ref application_user_registration_token_ } => application_user_registration_token_.get_value(),
                ApplicationUserRegistrationToken_Aggregator::Second { application_user_registration_token: ref application_user_registration_token_ } => application_user_registration_token_.get_value()
            };

            if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_registration_token(
                environment_configuration,
                application_user_registration_token_value,
                incoming.application_user_email.as_str(),
                incoming.application_user_device_id.as_str()
            ) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

        let application_user_registration_token_can_be_resent_from = match application_user_registration_token_aggregator {
            ApplicationUserRegistrationToken_Aggregator::First { application_user_registration_token: ref application_user_registration_token_ } => application_user_registration_token_.get_can_be_resent_from(),
            ApplicationUserRegistrationToken_Aggregator::Second { application_user_registration_token: ref application_user_registration_token_ } => application_user_registration_token_.get_can_be_resent_from()
        };

        let outcoming = Outcoming {
            verification_message_sent: can_send,
            application_user_registration_token_can_be_resent_from
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: String,
    application_user_device_id: String
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    verification_message_sent: bool,
    application_user_registration_token_can_be_resent_from: i64
}

enum ApplicationUserRegistrationToken_Aggregator<'a> {
    First {
        application_user_registration_token: ApplicationUserRegistrationToken<'a>
    },
    Second {
        application_user_registration_token: ApplicationUserRegistrationToken_1
    }
}