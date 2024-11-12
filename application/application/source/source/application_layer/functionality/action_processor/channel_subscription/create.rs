use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            channel::{
                Channel,
                Channel_AccessModifier,
                Channel_Id,
            },
            channel_subscription::ChannelSubscription,
            user_access_token::UserAccessToken,
        },
        functionality::service::{
            extractor::{
                Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                ChannelBy1,
                ChannelSubscriptionInsert1,
                Postgresql,
            },
            service::resolver::{
                    PostgresqlTransaction,
                    Resolver,
                    TransactionIsolationLevel,
                    UnixTime,
                },
        },
    },
};
use crate::infrastructure_layer::functionality::repository::Repository;
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::channel_subscription::create::{
        Incoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct ChannelSubscription_Create;
impl ActionProcessor_ for ActionProcessor<ChannelSubscription_Create> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: user_access_token_,
                } => user_access_token_,
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let mut database_1_postgresql_client = inner.database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let channel = match Repository::<Postgresql<Channel<'_>>>::find_1(
                &database_1_postgresql_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(channel_) => channel_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound));
                }
            };
            if channel.owner == user_access_token.user__id {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_IsChannelOwner));
            }
            if const { Channel_AccessModifier::Close as i16 } == channel.access_modifier {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
            }
            let postgresql_transaction = Resolver::<PostgresqlTransaction<'_>>::start(
                &mut database_1_postgresql_client,
                TransactionIsolationLevel::ReadCommitted,
            )
            .await?;
            if let Result::Err(aggregate_error) = Repository::<Postgresql<ChannelSubscription>>::create_1(
                postgresql_transaction.get_client(),
                ChannelSubscriptionInsert1 {
                    user__id: user_access_token.user__id,
                    channel__id: channel.id,
                    channel_subscription__created_at: Resolver::<UnixTime>::get_now(),
                },
            )
            .await
            {
                Resolver::<PostgresqlTransaction<'_>>::rollback(postgresql_transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<Channel<'_>>>::update_1(
                postgresql_transaction.get_client(),
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await
            {
                Resolver::<PostgresqlTransaction<'_>>::rollback(postgresql_transaction).await?;
                return Result::Err(aggregate_error);
            }
            Resolver::<PostgresqlTransaction<'_>>::commit(postgresql_transaction).await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
// TODO TODO TODO: qkwmdjndsicjpewem,lskdncyebchsdnjnsuhv[fo[sdccn]]
// Канал приватный, то есть, его не видно в поиске, но открытый, то есть, можно подписываться без одобрения.
// Значит, на него можно будет подписаться перебором со скрипта. А значит нужнг объединять приватность и закрытость в одно понятие.
// Разделять на 2 понятия бессмысленно. Для чего видеть в поиске закрытый канал?
