use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::application_user_access_refresh_token_workflow_exception::ApplicationUserAccessRefreshTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver as ApplicationUserAccessRefreshTokenExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::obfuscation_value_generator::ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver as ApplicationUserAccessTokenExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::id_generator::IdGenerator;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserAccessRefreshTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserAccessRefreshTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Update;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
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

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionHandlerResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_access_token_web_form,
            application_user_access_refresh_token_web_form
        ) = incoming.into_inner();

        let application_user_access_token = match SerializationFormResolver::deserialize(environment_configuration_resolver, application_user_access_token_web_form.as_str()) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let is_expired = match ApplicationUserAccessTokenExpirationTimeResolver::is_expired(&application_user_access_token) {
            Ok(is_expired_) => is_expired_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if is_expired {   // TODO TODO TODO TODO СДелать интервал, когда можео менять. На 3 часа раньше, чем срок экспирации, например
            let authorization_postgresql_pooled_connection = match authorization_postgresql_connection_pool.get().await {
                Ok(authorization_postgresql_pooled_connection_) => authorization_postgresql_pooled_connection_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };
            let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

            let application_user_access_refresh_token = match ApplicationUserAccessRefreshTokenDataProviderPostgresql::find_1(
                authorization_postgresql_connection, application_user_access_token.get_application_user_id(), application_user_access_token.get_application_user_log_in_token_device_id()
            ).await {
                Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            if let Some(mut application_user_access_refresh_token_) = application_user_access_refresh_token {
                let is_valid = match Encoder::is_valid(environment_configuration_resolver, &application_user_access_refresh_token_, application_user_access_refresh_token_web_form.as_str()) {
                    Ok(is_valid_) => is_valid_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };
                if is_valid && application_user_access_token.get_id().as_bytes() == application_user_access_refresh_token_.get_application_user_access_token_id().as_bytes() {
                    if !ApplicationUserAccessRefreshTokenExpirationTimeResolver::is_expired(&application_user_access_refresh_token_) {
                        let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                            Ok(expires_at_) => expires_at_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };
                        let application_user_access_token_new = ApplicationUserAccessToken::new(
                            IdGenerator::generate(),
                            application_user_access_token.get_application_user_id(),
                            Cow::Borrowed(application_user_access_token.get_application_user_log_in_token_device_id()),
                            expires_at
                        );

                        application_user_access_refresh_token_
                                .set_application_user_access_token_id(Cow::Borrowed(application_user_access_token_new.get_id()))
                                .set_obfuscation_value(ObfuscationValueGenerator::generate());

                        let update = Update {
                            application_user_access_refresh_token_expires_at: true,
                            application_user_access_refresh_token_updated_at: true
                        };

                        if let Err(mut error) = ApplicationUserAccessRefreshTokenStateManagerPostgresql::update(
                            authorization_postgresql_connection,
                            &mut application_user_access_refresh_token_,
                            update
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }

                        let application_user_access_token_web_form_new = match SerializationFormResolver::serialize(environment_configuration_resolver, &application_user_access_token_new) {
                            Ok(application_user_access_token_web_form_new_) => application_user_access_token_web_form_new_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };

                        let application_user_access_refresh_token_web_form_new = match Encoder::encode(environment_configuration_resolver, &application_user_access_refresh_token_) {
                            Ok(application_user_access_refresh_token_web_form_new_) => application_user_access_refresh_token_web_form_new_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };

                        return Ok(
                            ActionHandlerResult::new_with_outcoming(
                                Outcoming::new(application_user_access_token_web_form_new, application_user_access_refresh_token_web_form_new)
                            )
                        );

                    }

                    if let Err(mut error) = ApplicationUserAccessRefreshTokenStateManagerPostgresql::delete_1(
                        authorization_postgresql_connection,
                        application_user_access_refresh_token_.get_application_user_id(),
                        application_user_access_refresh_token_.get_application_user_log_in_token_device_id()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }

                    return Ok(ActionHandlerResult::new_with_application_user_access_refresh_token_workflow_exception(ApplicationUserAccessRefreshTokenWorkflowException::AlreadyExpired));
                }

                return Err(
                    ErrorAuditor::new(
                        BaseError::InvalidArgumentError,
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            return Ok(ActionHandlerResult::new_with_application_user_access_refresh_token_workflow_exception(ApplicationUserAccessRefreshTokenWorkflowException::NotFound));
        }

        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::NotExpired));
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (String, String) {
        return (
            self.application_user_access_token_web_form,
            self.application_user_access_refresh_token_web_form
        );
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

impl Outcoming {
    pub fn new(
        application_user_access_token_web_form: String,
        application_user_access_refresh_token_web_form: String
    ) -> Self {
        return Self {
            application_user_access_token_web_form,
            application_user_access_refresh_token_web_form
        };
    }
}