use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::domain_layer::functionality::service::application_user_access_token___extractor::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By6;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert10;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
pub use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::Incoming;
pub use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::Precedent;

pub struct Create;

impl Create {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, ErrorAuditor_>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    ErrorAuditor_::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(&incoming_.application_user_access_token_encrypted).await {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token = match extractor_result {
            InvalidArgumentResult::Ok {
                subject: extractor_result_,
            } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken {
                        application_user_access_token: application_user_access_token__,
                    } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired),
                            },
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList),
                            },
                        );
                    }
                };

                application_user_access_token_
            }
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument,
                    },
                );
            }
        };

        if !Validator::<Channel_Id>::is_valid(incoming_.channel_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::Channel_Id,
                },
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let channel = match PostgresqlRepository::<Channel<'_>>::find_1(
            database_1_postgresql_connection,
            &By6 {
                channel_id: incoming_.channel_id,
            },
        )
        .await
        {
            Ok(channel_) => channel_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let channel_ = match channel {
            Some(channel_) => channel_,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::Channel_NotFound),
                    },
                );
            }
        };

        if channel_.owner.0 == application_user_access_token.application_user_id.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_IsChannelOwner),
                },
            );
        }

        let channel_access_modifier = FormResolver::<Channel_AccessModifier>::to_representation(channel_.access_modifier);

        if let Channel_AccessModifier_::Close = channel_access_modifier {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::Channel_IsClose),
                },
            );
        }

        if let Err(mut error) = PostgresqlRepository::<ChannelSubscription>::create(
            database_1_postgresql_connection,
            Insert10 {
                application_user_id: application_user_access_token.application_user_id,
                channel_id: channel_.id,
            },
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_empty(),
            },
        );
    }
}
