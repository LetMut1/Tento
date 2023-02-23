use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__serialization_form_resolver::ApplicationUserAccessRefreshToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__obfuscation_value_generator::ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__expires_at_generator::ApplicationUserAccessToken_ExpiresAtGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__id_generator::ApplicationUserAccessToken_IdGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_registration_token__expiration_time_resolver::ApplicationUserRegistrationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_token__validator::ApplicationUserRegistrationToken_Validator;
use crate::domain_layer::functionality::service::application_user_registration_token__wrong_enter_tries_quantity_incrementor::ApplicationUserRegistrationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::Insert as ApplicationUserInsert;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::ApplicationUserAccessRefreshToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Insert as ApplicationUserAccessRefreshTokenInsert;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::ApplicationUserDevice_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::Insert as ApplicationUserDeviceInsert;
use crate::infrastructure_layer::functionality::repository::application_user_registration_token__postgresql_repository::ApplicationUserRegistrationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_registration_token__postgresql_repository::Update;
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
        environment_configuration_resolver: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {                                                                                              // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        if !ApplicationUser_Validator::is_valid_password(incoming.application_user_password.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
        }

        if !ApplicationUser_Validator::is_valid_nickname(incoming.application_user_nickname.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Nickname });
        }

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

        let is_valid_value = match ApplicationUserRegistrationToken_Validator::is_valid_value(incoming.application_user_registration_token_value.as_str()) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_valid_value {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserRegistrationToken_Value });
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
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let is_exist_1 = match ApplicationUser_PostgresqlRepository::is_exist_1(database_1_postgresql_connection, incoming.application_user_nickname.as_str()).await {
            Ok(is_exist_1_) => is_exist_1_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if is_exist_1 {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NicknameAlreadyExist
                    }
                }
            );
        }

        let is_exist_2 = match ApplicationUser_PostgresqlRepository::is_exist_2(database_1_postgresql_connection, incoming.application_user_email.as_str()).await {
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
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_NotFound
                        }
                    }
                );
            }
        };
        if ApplicationUserRegistrationToken_ExpirationTimeResolver::is_expired(&application_user_registration_token_) {
            if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::delete(
                database_2_postgresql_connection, application_user_registration_token_.get_application_user_email()
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_AlreadyExpired
                    }
                }
            );
        }

        if !application_user_registration_token_.get_is_approved() {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_IsNotApproved
                    }
                }
            );
        }

        if application_user_registration_token_.get_value() != incoming.application_user_registration_token_value.as_str() {
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

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken_WrongValue
                    }
                }
            );
        }

        let application_user_password_hash = match ApplicationUser_PasswordHashResolver::create(incoming.application_user_password.as_str()) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if let Err(mut error) = ApplicationUserRegistrationToken_PostgresqlRepository::delete(
            database_2_postgresql_connection, application_user_registration_token_.get_application_user_email()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        let application_user_insert = ApplicationUserInsert {
            application_user_email: incoming.application_user_email,
            application_user_nickname: incoming.application_user_nickname,
            application_user_password_hash,
        };
        let application_user = match ApplicationUser_PostgresqlRepository::create(database_1_postgresql_connection, application_user_insert).await {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_device_insert = ApplicationUserDeviceInsert {
            application_user_device_id: incoming.application_user_device_id,
            application_user_id: application_user.get_id()
        };
        let application_user_device = match ApplicationUserDevice_PostgresqlRepository::create(database_1_postgresql_connection, application_user_device_insert).await {
            Ok(application_user_device_) => application_user_device_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let expires_at = match ApplicationUserAccessToken_ExpiresAtGenerator::generate() {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user_access_token = ApplicationUserAccessToken::new(
            ApplicationUserAccessToken_IdGenerator::generate(),
            application_user.get_id(),
            Cow::Borrowed(application_user_device.get_id()),
            expires_at
        );
                // TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
        let application_user_access_refresh_token_insert = ApplicationUserAccessRefreshTokenInsert {
            application_user_id: application_user.get_id(),
            application_user_device_id: Cow::Borrowed(application_user_device.get_id()),
            application_user_access_token_id: Cow::Borrowed(application_user_access_token.get_id()),
            application_user_access_refresh_token_obfuscation_value: ApplicationUserAccessRefreshToken_ObfuscationValueGenerator::generate(),
        };

        let application_user_access_refresh_token = match ApplicationUserAccessRefreshToken_PostgresqlRepository::create(
            database_2_postgresql_connection, application_user_access_refresh_token_insert
        ).await {
            Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token_deserialized_form = match ApplicationUserAccessToken_SerializationFormResolver::serialize(environment_configuration_resolver, &application_user_access_token) {
            Ok(application_user_access_token_deserialized_form_) => application_user_access_token_deserialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_refresh_token_deserialized_form = match ApplicationUserAccessRefreshToken_SerializationFormResolver::encode(environment_configuration_resolver, &application_user_access_refresh_token) {
            Ok(application_user_access_refresh_token_deserialized_form_) => application_user_access_refresh_token_deserialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(
            ArgumentResult::Ok {
                subject: ActionProcessorResult::Outcoming {
                    outcoming: Outcoming {
                        application_user_access_token_deserialized_form,
                        application_user_access_refresh_token_deserialized_form
                    }
                }
            }
        );
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: String,
    application_user_nickname: String,
    application_user_password: String,
    application_user_email: String,
    application_user_registration_token_value: String
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}