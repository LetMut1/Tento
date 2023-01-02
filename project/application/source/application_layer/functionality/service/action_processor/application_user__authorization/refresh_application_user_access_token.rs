use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessRefreshToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessToken_WorkflowException;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__encoder::ApplicationUserAccessRefreshToken_Encoder;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__expiration_time_resolver::ApplicationUserAccessRefreshToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__obfuscation_value_generator::ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__expiration_time_resolver::ApplicationUserAccessToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_access_token__expires_at_generator::ApplicationUserAccessToken_ExpiresAtGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__id_generator::ApplicationUserAccessToken_IdGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::ApplicationUserAccessRefreshToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Update;
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

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let application_user_access_token = match ApplicationUserAccessToken_SerializationFormResolver::deserialize(environment_configuration_resolver, incoming.application_user_access_token_web_form.as_str()) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if ApplicationUserAccessToken_ExpirationTimeResolver::is_expired(&application_user_access_token) {   // TODO TODO TODO TODO СДелать интервал, когда можео менять. На 3 часа раньше, чем срок экспирации, например
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

            let application_user_access_refresh_token = match ApplicationUserAccessRefreshToken_PostgresqlRepository::find_1(
                authorization_postgresql_connection, application_user_access_token.get_application_user_id(), application_user_access_token.get_application_user_device_id()
            ).await {
                Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            if let Some(mut application_user_access_refresh_token_) = application_user_access_refresh_token {
                let is_valid = match ApplicationUserAccessRefreshToken_Encoder::is_valid(environment_configuration_resolver, &application_user_access_refresh_token_, incoming.application_user_access_refresh_token_web_form.as_str()) {
                    Ok(is_valid_) => is_valid_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };
                if is_valid && application_user_access_token.get_id().as_bytes() == application_user_access_refresh_token_.get_application_user_access_token_id().as_bytes() {
                    if !ApplicationUserAccessRefreshToken_ExpirationTimeResolver::is_expired(&application_user_access_refresh_token_) {
                        let expires_at = match ApplicationUserAccessToken_ExpiresAtGenerator::generate() {
                            Ok(expires_at_) => expires_at_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };
                        let application_user_access_token_new = ApplicationUserAccessToken::new(
                            ApplicationUserAccessToken_IdGenerator::generate(),
                            application_user_access_token.get_application_user_id(),
                            Cow::Borrowed(application_user_access_token.get_application_user_device_id()),
                            expires_at
                        );

                        application_user_access_refresh_token_
                                .set_application_user_access_token_id(Cow::Borrowed(application_user_access_token_new.get_id()))
                                .set_obfuscation_value(ApplicationUserAccessRefreshToken_ObfuscationValueGenerator::generate());

                        let update = Update {
                            application_user_access_refresh_token_expires_at: true,
                            application_user_access_refresh_token_updated_at: true
                        };

                        if let Err(mut error) = ApplicationUserAccessRefreshToken_PostgresqlRepository::update(
                            authorization_postgresql_connection,
                            &mut application_user_access_refresh_token_,
                            update
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }

                        let application_user_access_token_web_form_new = match ApplicationUserAccessToken_SerializationFormResolver::serialize(environment_configuration_resolver, &application_user_access_token_new) {
                            Ok(application_user_access_token_web_form_new_) => application_user_access_token_web_form_new_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };

                        let application_user_access_refresh_token_web_form_new = match ApplicationUserAccessRefreshToken_Encoder::encode(environment_configuration_resolver, &application_user_access_refresh_token_) {
                            Ok(application_user_access_refresh_token_web_form_new_) => application_user_access_refresh_token_web_form_new_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };

                        return Ok(
                            ActionProcessorResult::outcoming(
                                Outcoming {
                                    application_user_access_token_web_form: application_user_access_token_web_form_new,
                                    application_user_access_refresh_token_web_form: application_user_access_refresh_token_web_form_new
                                }
                            )
                        );

                    }

                    if let Err(mut error) = ApplicationUserAccessRefreshToken_PostgresqlRepository::delete_1(
                        authorization_postgresql_connection,
                        application_user_access_refresh_token_.get_application_user_id(),
                        application_user_access_refresh_token_.get_application_user_device_id()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }

                    return Ok(ActionProcessorResult::application_user_access_refresh_token__workflow_exception(ApplicationUserAccessRefreshToken_WorkflowException::AlreadyExpired));
                }

                return Err(
                    ErrorAuditor::new(
                        BaseError::InvalidArgumentError,
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            return Ok(ActionProcessorResult::application_user_access_refresh_token__workflow_exception(ApplicationUserAccessRefreshToken_WorkflowException::NotFound));
        }

        return Ok(ActionProcessorResult::application_user_access_token__workflow_exception(ApplicationUserAccessToken_WorkflowException::NotExpired));
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
struct Outcoming {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}