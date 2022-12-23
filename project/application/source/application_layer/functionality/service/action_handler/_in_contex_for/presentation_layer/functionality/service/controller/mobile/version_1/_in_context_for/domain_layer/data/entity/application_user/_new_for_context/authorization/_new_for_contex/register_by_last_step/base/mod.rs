use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_exception::ApplicationUserRegistrationConfirmationTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__encoder::ApplicationUserAccessRefreshToken_Encoder;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__obfuscation_value_generator::ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__id_generator::ApplicationUserAccessToken_IdGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__expiration_time_resolver::ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__validator::ApplicationUserRegistrationConfirmationToken_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__wrong_enter_tries_quantity_incrementor::ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::Insert as ApplicationUserInsert;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::ApplicationUserAccessRefreshToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Insert as ApplicationUserAccessRefreshTokenInsert;
use crate::infrastructure_layer::functionality::repository::application_user_registration_confirmation_token__postgresql_repository::ApplicationUserRegistrationConfirmationToken_PostgresqlRepository;
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
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionHandlerResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {                                                                                               // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id,                                                // TODO Это значение должно быть одно для 1 устройствоа клиента. ЛУчше сделать его постоянным, - Mac устрйоства, или что-то подобное. То значение, которе будет для КЛаудМессадж. Хранить в БД.
            application_user_nickname,
            application_user_password,
            application_user_email,
            application_user_registration_confirmation_token_value
        ) = incoming.into_inner();

        if ApplicationUser_Validator::is_valid_password(application_user_password.as_str()) {
            if ApplicationUser_Validator::is_valid_nickname(application_user_nickname.as_str()) {
                let is_valid_email = match ApplicationUser_Validator::is_valid_email(application_user_email.as_str()) {
                    Ok(is_valid_email_) => is_valid_email_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };
                if is_valid_email {
                    let is_valid_value = match ApplicationUserRegistrationConfirmationToken_Validator::is_valid_value(application_user_registration_confirmation_token_value.as_str()) {
                        Ok(is_valid_value_) => is_valid_value_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };
                    if is_valid_value {
                        let core_postgresql_pooled_connection = match core_postgresql_connection_pool.get().await {
                            Ok(core_postgresql_pooled_connection_) => core_postgresql_pooled_connection_,
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        };
                        let core_postgresql_connection = &*core_postgresql_pooled_connection;

                        let is_exist_1 = match ApplicationUser_PostgresqlRepository::is_exist_1(core_postgresql_connection, application_user_nickname.as_str()).await {     // TODO написать один запро на два значения.!!!!!!!!!!
                            Ok(is_exist_1_) => is_exist_1_,
                            Err(mut error) => {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        };
                        if !is_exist_1 {
                            let is_exist_2 = match ApplicationUser_PostgresqlRepository::is_exist_2(core_postgresql_connection, application_user_email.as_str()).await {
                                Ok(is_exist_2_) => is_exist_2_,
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            };
                            if !is_exist_2 {
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

                                let application_user_registration_confirmation_token = match ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::find_1(
                                    authorization_postgresql_connection, application_user_email.as_str()
                                ).await {
                                    Ok(application_user_registration_confirmation_token_) => application_user_registration_confirmation_token_,
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                        return Err(error);
                                    }
                                };
                                if let Some(mut application_user_registration_confirmation_token_) = application_user_registration_confirmation_token {
                                    let is_expired = match ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver::is_expired(&application_user_registration_confirmation_token_) {
                                        Ok(is_expired_) => is_expired_,
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                            return Err(error);
                                        }
                                    };
                                    if !is_expired {
                                        if application_user_registration_confirmation_token_.get_is_approved() {
                                            if application_user_registration_confirmation_token_.get_value() == application_user_registration_confirmation_token_value.as_str() {
                                                let application_user_password_hash = match ApplicationUser_PasswordHashResolver::create(application_user_password.as_str()) {
                                                    Ok(application_user_password_hash_) => application_user_password_hash_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::delete(
                                                    authorization_postgresql_connection, application_user_registration_confirmation_token_.get_application_user_email()
                                                ).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                let application_usert_insert = ApplicationUserInsert::new(
                                                    application_user_email,
                                                    application_user_nickname,
                                                    application_user_password_hash,
                                                );

                                                let application_user = match ApplicationUser_PostgresqlRepository::create(core_postgresql_connection, application_usert_insert).await {
                                                    Ok(application_user_) => application_user_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                                                    Ok(expires_at_) => expires_at_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };
                                                let application_user_access_token = ApplicationUserAccessToken::new(
                                                    ApplicationUserAccessToken_IdGenerator::generate(),
                                                    application_user.get_id(),
                                                    Cow::Borrowed(application_user_log_in_token_device_id.as_str()),
                                                    expires_at
                                                );
// TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
                                                let application_user_access_refresh_token_insert = ApplicationUserAccessRefreshTokenInsert {
                                                    application_user_id: application_user.get_id(),
                                                    application_user_log_in_token_device_id: Cow::Borrowed(application_user_log_in_token_device_id.as_str()),
                                                    application_user_access_token_id: Cow::Borrowed(application_user_access_token.get_id()),
                                                    application_user_access_refresh_token_obfuscation_value: ApplicationUserAccessRefreshToken_ObfuscationValueGenerator::generate(),
                                                };

                                                let application_user_access_refresh_token = match ApplicationUserAccessRefreshToken_PostgresqlRepository::create(
                                                    authorization_postgresql_connection, application_user_access_refresh_token_insert
                                                ).await {
                                                    Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                let application_user_access_token_web_form = match ApplicationUserAccessToken_SerializationFormResolver::serialize(environment_configuration_resolver, &application_user_access_token) {
                                                    Ok(application_user_access_token_web_form_) => application_user_access_token_web_form_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                let application_user_access_refresh_token_web_form = match ApplicationUserAccessRefreshToken_Encoder::encode(environment_configuration_resolver, &application_user_access_refresh_token) {
                                                    Ok(application_user_access_refresh_token_web_form_) => application_user_access_refresh_token_web_form_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                return Ok(
                                                    ActionHandlerResult::new_with_outcoming(
                                                        Outcoming::new(application_user_access_token_web_form, application_user_access_refresh_token_web_form)
                                                    )
                                                );
                                            }

                                            if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_confirmation_token_) {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                return Err(error);
                                            }

                                            if application_user_registration_confirmation_token_.get_wrong_enter_tries_quantity() <= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                    authorization_postgresql_connection, &application_user_registration_confirmation_token_
                                                ).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }
                                            } else {
                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::delete(
                                                    authorization_postgresql_connection, application_user_registration_confirmation_token_.get_application_user_email()
                                                ).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }
                                            }

                                            return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::WrongValue));
                                        }

                                        return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::IsNotApproved));
                                    }

                                    return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::AlreadyExpired));
                                }

                                return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::NotFound));
                            }

                            return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::EmailAlreadyExist));
                        }

                        return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::NicknameAlreadyExist));
                    }

                    return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::InvalidValue));
                }

                return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidEmail));
            }

            return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidNickname));
        }

        return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidPassword));
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_log_in_token_device_id: String,
    application_user_nickname: String,
    application_user_password: String,
    application_user_email: String,
    application_user_registration_confirmation_token_value: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (String, String, String, String, String) {
        return (
            self.application_user_log_in_token_device_id,
            self.application_user_nickname,
            self.application_user_password,
            self.application_user_email,
            self.application_user_registration_confirmation_token_value
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