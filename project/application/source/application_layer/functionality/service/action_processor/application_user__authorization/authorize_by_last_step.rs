use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_4;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::functionality::service::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::Insert as ApplicationUserAccessRefreshTokenInsert;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::Insert as ApplicationUserDeviceInsert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
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
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user_id) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Id });
        }

        let is_valid_value = match Validator::<ApplicationUserAuthorizationToken_Value>::is_valid(
            incoming.application_user_authorization_token_value.as_str()
        ) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_value {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserAuthorizationToken_Value });
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device_id.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
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

        let application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken_2>::find_1(
            database_2_postgresql_connection,
            incoming.application_user_id,
            incoming.application_user_device_id.as_str()
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

        if ExpirationTimeResolver::<ApplicationUserAuthorizationToken<'_>>::is_expired(&application_user_authorization_token_) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
                database_2_postgresql_connection,
                incoming.application_user_id,
                incoming.application_user_device_id.as_str()
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
            let application_user_authorization_token_wrong_enter_tries_quantity = match application_user_authorization_token_.get_wrong_enter_tries_quantity()
                .checked_add(1) {
                Some(application_user_authorization_token_wrong_enter_tries_quantity_) => application_user_authorization_token_wrong_enter_tries_quantity_,
                None => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::create_out_of_range(),
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            if application_user_authorization_token_wrong_enter_tries_quantity <= ApplicationUserAuthorizationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                application_user_authorization_token_.set_wrong_enter_tries_quantity(application_user_authorization_token_wrong_enter_tries_quantity);

                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken_4>::update(
                    database_2_postgresql_connection,
                    &application_user_authorization_token_,
                    incoming.application_user_id,
                    incoming.application_user_device_id.as_str()
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    incoming.application_user_id,
                    incoming.application_user_device_id.as_str()
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

        let is_exist = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_3(
            database_1_postgresql_connection,
            incoming.application_user_id
        ).await {
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

        let expires_at = match Generator::<ApplicationUserAccessToken_ExpiresAt>::generate() {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            incoming.application_user_id,
            Cow::Borrowed(incoming.application_user_device_id.as_str()),
            expires_at
        );

        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            incoming.application_user_id,
            incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token_id = Cow::Borrowed(application_user_access_token.get_id());

        let application_user_access_refresh_token_obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();

        let application_user_access_refresh_token_expires_at = match Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate() {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_refresh_token_updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();
// TODO  TRANZACTION
        let application_user_access_refresh_token_ = match application_user_access_refresh_token {
            Some(mut application_user_access_refresh_token__) => {
                application_user_access_refresh_token__
                    .set_application_user_access_token_id(application_user_access_token_id)
                    .set_obfuscation_value(application_user_access_refresh_token_obfuscation_value)
                    .set_expires_at(application_user_access_refresh_token_expires_at)
                    .set_updated_at(application_user_access_refresh_token_updated_at);

                if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken_1>::update(
                    database_2_postgresql_connection,
                    &application_user_access_refresh_token__,
                    incoming.application_user_id,
                    incoming.application_user_device_id.as_str()
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }

                application_user_access_refresh_token__
            }
            None => {
                let application_user_access_refresh_token_insert = ApplicationUserAccessRefreshTokenInsert {
                    application_user_id: incoming.application_user_id,
                    application_user_device_id: Cow::Borrowed(incoming.application_user_device_id.as_str()),
                    application_user_access_token_id,
                    application_user_access_refresh_token_obfuscation_value,
                    application_user_access_refresh_token_expires_at,
                    application_user_access_refresh_token_updated_at
                };

                let application_user_access_refresh_token__ = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create(
                    database_2_postgresql_connection,
                    application_user_access_refresh_token_insert
                ).await {
                    Ok(application_user_access_refresh_token___) => application_user_access_refresh_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                application_user_access_refresh_token__
            }
        };

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
            database_2_postgresql_connection,
            incoming.application_user_id,
            incoming.application_user_device_id.as_str()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }
// TODO  TRANZACTION
        let application_user_access_token_serialized_form = match SerializationFormResolver::<ApplicationUserAccessToken<'_>>::serialize(
            environment_configuration,
            &application_user_access_token
        ) {
            Ok(application_user_access_token_serialized_form_) => application_user_access_token_serialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_refresh_token_serialized_form = match SerializationFormResolver::<ApplicationUserAccessRefreshToken<'_>>::serialize(
            environment_configuration,
            &application_user_access_refresh_token_
        ) {
            Ok(application_user_access_refresh_token_serialized_form_) => application_user_access_refresh_token_serialized_form_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_device_insert = ApplicationUserDeviceInsert {
            application_user_device_id: incoming.application_user_device_id,
            application_user_id: incoming.application_user_id
        };

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserDevice>::create(
            database_1_postgresql_connection,
            application_user_device_insert
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        };

        return Ok(
            ArgumentResult::Ok {
                subject: ActionProcessorResult::Outcoming {
                    outcoming: Outcoming {
                        application_user_access_token_serialized_form,
                        application_user_access_refresh_token_serialized_form
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
    application_user_access_token_serialized_form: String,
    application_user_access_refresh_token_serialized_form: String
}