use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::{
                Channel,
                Channel_AccessModifier,
                Channel_Id,
            },
            channel_subscription::ChannelSubscription,
        },
        functionality::service::{
            extractor::{
                application_user_access_token::Extracted,
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
use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::{
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
pub struct ChannelSubscription__Base___Create;
impl ActionProcessor_ for ActionProcessor<ChannelSubscription__Base___Create> {
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
            let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )? {
                Extracted::ApplicationUserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                Extracted::ApplicationUserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
                }
                Extracted::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
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
            if channel.owner == application_user_access_token.application_user__id {
                return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUser_IsChannelOwner));
            }
            if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
            }
            PostgresqlRepository::<ChannelSubscription>::create_1(
                database_1_postgresql_connection,
                Insert1 {
                    application_user__id: application_user_access_token.application_user__id,
                    channel__id: channel.id,
                    channel_subscription__created_at: Resolver::<UnixTime>::get_now(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
