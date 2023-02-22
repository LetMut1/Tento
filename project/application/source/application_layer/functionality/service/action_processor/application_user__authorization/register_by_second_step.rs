use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_registration_token__expiration_time_resolver::ApplicationUserRegistrationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_token__validator::ApplicationUserRegistrationToken_Validator;
use crate::domain_layer::functionality::service::application_user_registration_token__wrong_enter_tries_quantity_incrementor::ApplicationUserRegistrationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::application_user_registration_token__postgresql_repository::ApplicationUserRegistrationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_registration_token__postgresql_repository::Update;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<Void>, ErrorAuditor>
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
            return Ok(ActionProcessorResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Email });
        }

        let is_valid_value = match ApplicationUserRegistrationToken_Validator::is_valid_value(incoming.application_user_registration_token_value.as_str()) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_valid_value {
            return Ok(ActionProcessorResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserRegistrationToken_Value });
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

        let application_user_registration_token = match ApplicationUserRegistrationToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection, incoming.application_user_email.as_str()
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
                return Ok(ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_NotFound });
            }
        };

        if ApplicationUserRegistrationToken_ExpirationTimeResolver::is_expired(&application_user_registration_token_) {
            if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::delete(
                database_2_postgresql_connection, application_user_registration_token_.get_application_user_email()
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_AlreadyExpired });
        }

        if application_user_registration_token_.get_is_approved() {
            return Ok(ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_AlreadyApproved });
        }

        if application_user_registration_token_.get_value().as_bytes() != incoming.application_user_registration_token_value.as_bytes() {
            if let Err(mut error) = ApplicationUserRegistrationToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_token_) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            if application_user_registration_token_.get_wrong_enter_tries_quantity() <= ApplicationUserRegistrationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::update(
                    database_2_postgresql_connection,
                    &mut application_user_registration_token_,
                    Update { application_user_registration_token_expires_at: false }
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::delete(
                    database_2_postgresql_connection, application_user_registration_token_.get_application_user_email()
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }

            return Ok(ActionProcessorResult::UserWorkflowPrecedent { user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_WrongValue });
        }

        application_user_registration_token_.set_is_approved(true);

        if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::update(
            database_2_postgresql_connection,
            &mut application_user_registration_token_,
            Update { application_user_registration_token_expires_at: false }
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(ActionProcessorResult::Empty);
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: String,
    application_user_registration_token_value: String
}