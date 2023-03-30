use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_device__validator::ApplicationUserDevice_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__property_generator::ApplicationUserResetPasswordToken_PropertyGenerator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__sending_opportunity_resolver::ApplicationUserResetPasswordToken_SendingOpportunityResolver;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::ApplicationUserResetPasswordToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::Insert;
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
        let is_valid_email = match ApplicationUser_Validator::is_valid_email(incoming.application_user_email.as_str()) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_email {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Email });
        }

        if !ApplicationUserDevice_Validator::is_valid_id(incoming.application_user_device_id.as_str()) {
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

        let application_user = match ApplicationUser_PostgresqlRepository::find_2(
            &*database_1_postgresql_pooled_connection, incoming.application_user_email.as_str()
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
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NotFound
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

        let application_user_reset_password_token = match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection,
            application_user_.get_id(),
            incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (application_user_reset_password_token_, can_send) = match application_user_reset_password_token {
            Some(mut application_user_reset_password_token__) => {
                let (can_send_, mut need_to_update) = if ApplicationUserResetPasswordToken_SendingOpportunityResolver::can_send(
                    &application_user_reset_password_token__
                ) {
                    let application_user_reset_password_token_can_be_resent_from = match ApplicationUserResetPasswordToken_PropertyGenerator::generate_can_be_resent_from() {
                        Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_reset_password_token__.set_can_be_resent_from(application_user_reset_password_token_can_be_resent_from);

                    (true, true)
                } else {
                    (false, false)
                };

                if ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token__)
                    || application_user_reset_password_token__.get_is_approved() {
                    need_to_update = true;

                    let application_user_reset_password_token_expires_at = match ApplicationUserResetPasswordToken_PropertyGenerator::generate_expires_at() {
                        Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_reset_password_token__
                        .set_value(ApplicationUserResetPasswordToken_PropertyGenerator::generate_value())
                        .set_wrong_enter_tries_quantity(0)
                        .set_is_approved(false)
                        .set_expires_at(application_user_reset_password_token_expires_at);
                }

                if need_to_update {
                    if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                        database_2_postgresql_connection,
                        &application_user_reset_password_token__
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }

                (application_user_reset_password_token__, can_send_)
            }
            None => {
                let application_user_reset_password_token_expires_at = match ApplicationUserResetPasswordToken_PropertyGenerator::generate_expires_at() {
                    Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user_reset_password_token_can_be_resent_from = match ApplicationUserResetPasswordToken_PropertyGenerator::generate_can_be_resent_from() {
                    Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_id: application_user_.get_id(),
                    application_user_device_id: Cow::Borrowed(incoming.application_user_device_id.as_str()),
                    application_user_reset_password_token_value: ApplicationUserResetPasswordToken_PropertyGenerator::generate_value(),
                    application_user_reset_password_token_wrong_enter_tries_quantity: 0,
                    application_user_reset_password_token_is_approved: false,
                    application_user_reset_password_token_expires_at,
                    application_user_reset_password_token_can_be_resent_from
                };

                let application_user_reset_password_token__ = match ApplicationUserResetPasswordToken_PostgresqlRepository::create(
                    database_2_postgresql_connection, insert
                ).await {
                    Ok(application_user_reset_password_token___) => application_user_reset_password_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                (application_user_reset_password_token__, true)
            }
        };

        if can_send {
            if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_reset_password_token(
                environment_configuration,
                application_user_reset_password_token_.get_value(),
                application_user_.get_email(),
                application_user_reset_password_token_.get_application_user_device_id()
            ) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

        let outcoming = Outcoming {
            application_user_id: application_user_.get_id(),
            verification_message_sent: can_send,
            application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token_.get_can_be_resent_from()
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
    application_user_id: i64,
    verification_message_sent: bool,
    application_user_reset_password_token_can_be_resent_from: i64
}