use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__obfuscation_value_generator::ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::application_user_access_refresh_token__serialization_form_resolver::ApplicationUserAccessRefreshToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_access_token__expires_at_generator::ApplicationUserAccessToken_ExpiresAtGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__id_generator::ApplicationUserAccessToken_IdGenerator;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__expiration_time_resolver::ApplicationUserAuthorizationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__validator::ApplicationUserAuthorizationToken_Validator;
use crate::domain_layer::functionality::service::application_user_authorization_token__wrong_enter_tries_quantity_incrementor::ApplicationUserAuthorizationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::ApplicationUserAccessRefreshToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Insert as ApplicationUserAccessRefreshTokenInsert;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Update as ApplicationUserAccessRefreshTokenUpdate;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::ApplicationUserAuthorizationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Update as ApplicationUserAuthorizationTokenUpdate;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::ApplicationUserDevice_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::Insert as ApplicationUserDeviceInsert;
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
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,             // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let is_valid_value = match ApplicationUserAuthorizationToken_Validator::is_valid_value(incoming.application_user_authorization_token_value.as_str()) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_valid_value {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserAuthorizationToken_Value });
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

        let application_user_authorization_token = match ApplicationUserAuthorizationToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection, incoming.application_user_id, incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let mut application_user_authorization_token_ = match application_user_authorization_token {
            Some(application_user_authorization_token__) => application_user_authorization_token__,
            None => {
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_NotFound
                        }
                    }
                );
            }
        };

        if ApplicationUserAuthorizationToken_ExpirationTimeResolver::is_expired(&application_user_authorization_token_) {
            if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::delete(
                database_2_postgresql_connection, application_user_authorization_token_.get_application_user_id(), application_user_authorization_token_.get_application_user_device_id()
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired
                    }
                }
            );
        }

        if application_user_authorization_token_.get_value() != incoming.application_user_authorization_token_value.as_str() {
            if let Err(mut error) = ApplicationUserAuthorizationToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_authorization_token_) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            if application_user_authorization_token_.get_wrong_enter_tries_quantity() <= ApplicationUserAuthorizationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::update(
                    database_2_postgresql_connection,
                    &mut application_user_authorization_token_,
                    ApplicationUserAuthorizationTokenUpdate { application_user_authorization_token_expires_at: false }
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::delete(
                    database_2_postgresql_connection, application_user_authorization_token_.get_application_user_id(), application_user_authorization_token_.get_application_user_device_id()
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_WrongValue
                    }
                }
            );
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

        let is_exist = match ApplicationUser_PostgresqlRepository::is_exist_3(database_1_postgresql_connection, incoming.application_user_id).await {
            Ok(is_exist_) => is_exist_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_exist {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NotFound
                    }
                }
            );
        }

        let expires_at = match ApplicationUserAccessToken_ExpiresAtGenerator::generate() {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user_access_token = ApplicationUserAccessToken::new(
            ApplicationUserAccessToken_IdGenerator::generate(),
            application_user_authorization_token_.get_application_user_id(),
            Cow::Borrowed(application_user_authorization_token_.get_application_user_device_id()),
            expires_at
        );

        let application_user_access_refresh_token = match ApplicationUserAccessRefreshToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection, application_user_authorization_token_.get_application_user_id(), application_user_authorization_token_.get_application_user_device_id()
        ).await {
            Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
// TODO  TRANZACTION
        let application_user_access_refresh_token_ = match application_user_access_refresh_token {
            Some(mut application_user_access_refresh_token__) => {
                application_user_access_refresh_token__
                    .set_application_user_access_token_id(Cow::Borrowed(application_user_access_token.get_id()))
                    .set_obfuscation_value(ApplicationUserAccessRefreshToken_ObfuscationValueGenerator::generate());

                let application_user_access_refresh_token_update = ApplicationUserAccessRefreshTokenUpdate {
                    application_user_access_refresh_token_expires_at: true,
                    application_user_access_refresh_token_updated_at: true
                };

                if let Err(mut error) = ApplicationUserAccessRefreshToken_PostgresqlRepository::update(
                    database_2_postgresql_connection,
                    &mut application_user_access_refresh_token__,
                    application_user_access_refresh_token_update
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }

                application_user_access_refresh_token__
            }
            None => {
                let application_user_access_refresh_token_insert = ApplicationUserAccessRefreshTokenInsert {
                    application_user_id: application_user_authorization_token_.get_application_user_id(),
                    application_user_device_id: Cow::Borrowed(application_user_authorization_token_.get_application_user_device_id()),
                    application_user_access_token_id: Cow::Borrowed(application_user_access_token.get_id()),
                    application_user_access_refresh_token_obfuscation_value: ApplicationUserAccessRefreshToken_ObfuscationValueGenerator::generate(),
                };

                match ApplicationUserAccessRefreshToken_PostgresqlRepository::create(database_2_postgresql_connection, application_user_access_refresh_token_insert).await {
                    Ok(application_user_access_refresh_token___) => application_user_access_refresh_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }
            }
        };

        if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::delete(
            database_2_postgresql_connection, application_user_authorization_token_.get_application_user_id(), application_user_authorization_token_.get_application_user_device_id()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
// TODO  TRANZACTION
        let application_user_access_token_deserialized_form = match ApplicationUserAccessToken_SerializationFormResolver::serialize(environment_configuration, &application_user_access_token) {
            Ok(application_user_access_token_deserialized_form_) => application_user_access_token_deserialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_refresh_token_deserialized_form = match ApplicationUserAccessRefreshToken_SerializationFormResolver::encode(environment_configuration, &application_user_access_refresh_token_) {
            Ok(application_user_access_refresh_token_deserialized_form_) => application_user_access_refresh_token_deserialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_device_insert = ApplicationUserDeviceInsert {
            application_user_device_id: incoming.application_user_device_id,
            application_user_id: incoming.application_user_id
        };
        if let Err(mut error) = ApplicationUserDevice_PostgresqlRepository::create(database_1_postgresql_connection, application_user_device_insert).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
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
    application_user_id: i64,
    application_user_device_id: String,
    application_user_authorization_token_value: String
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}