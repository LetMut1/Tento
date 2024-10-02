use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            user_access_token::UserAccessToken,
            channel::{
                Channel,
                Channel_AccessModifier,
                Channel_Id,
            },
            channel_subscription::ChannelSubscription,
        },
        functionality::service::{
            extractor::{
                user_access_token::Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::{repository::postgresql::{
            channel::By1,
            channel_subscription::Insert1,
            PostgresqlRepository,
        }, service::resolver::{date_time::UnixTime, Resolver}},
    },
};
use action_processor_incoming_outcoming::action_processor::channel_subscription::create::{
    Incoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct ChannelSubscription_Create;
impl ActionProcessor_ for ActionProcessor<ChannelSubscription_Create> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
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
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            let channel = match PostgresqlRepository::<Channel<'_>>::find_1(
                database_1_postgresql_connection,
                By1 {
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
            if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
            }
            PostgresqlRepository::<ChannelSubscription>::create_1(
                database_1_postgresql_connection,
                Insert1 {
                    user__id: user_access_token.user__id,
                    channel__id: channel.id,
                    channel_subscription__created_at: Resolver::<UnixTime>::get_now(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
