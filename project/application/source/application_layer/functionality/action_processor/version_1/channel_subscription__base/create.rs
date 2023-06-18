use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::Precedent;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::domain_layer::functionality::service::channel__access_modifier_resolver::Channel_AccessModifierResolver;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::channel_subscription__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Serialize;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming
    ) -> Result<InvalidArgumentResult<ActionProcessorResult<Void>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration, incoming.application_user_access_token_serialized_form.as_str()
        ).await {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token = match extractor_result {
            InvalidArgumentResult::Ok { subject: extractor_result_ } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: application_user_access_token__ } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: ActionProcessorResult::Precedent {
                                    precedent: Precedent::ApplicationUserAccessToken_AlreadyExpired
                                }
                            }
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: ActionProcessorResult::Precedent {
                                    precedent: Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList
                                }
                            }
                        );
                    }
                };

                application_user_access_token_
            }
            InvalidArgumentResult::InvalidArgument { invalid_argument } => {
                return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument });
            }
        };

        if !Validator::<Channel_Id>::is_valid(incoming.channel_id) {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Id });
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

        let channel = match PostgresqlRepository::<Channel<'_>>::find_1(
            database_1_postgresql_connection,
            incoming.channel_id
        ).await {
            Ok(channel_) => channel_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let channel_ = match channel {
            Some(channel_) => channel_,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: ActionProcessorResult::Precedent {
                            precedent: Precedent::Channel_NotFound
                        }
                    }
                );
            }
        };

        if channel_.get_owner().get() == application_user_access_token.get_application_user_id().get() {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: ActionProcessorResult::Precedent {
                        precedent: Precedent::ApplicationUser_IsChannelOwner
                    }
                }
            );
        }

        let channel_access_modifier = Channel_AccessModifierResolver::to_representation(channel_.get_access_modifier());

        if let Channel_AccessModifier_::Close = channel_access_modifier {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: ActionProcessorResult::Precedent {
                        precedent: Precedent::Channel_IsClosed
                    }
                }
            );
        }

        let insert = Insert {
            application_user_id: application_user_access_token.get_application_user_id(),
            channel_id: channel_.get_id()
        };

        if let Err(mut error) = PostgresqlRepository::<ChannelSubscription>::create(
            database_1_postgresql_connection, insert
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(InvalidArgumentResult::Ok { subject: ActionProcessorResult::Void });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_serialized_form: String,
    channel_id: Channel_Id
}