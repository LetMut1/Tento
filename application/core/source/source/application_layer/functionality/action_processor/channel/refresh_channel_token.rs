use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::Channel,channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
                    ChannelToken_ObfuscationValue,
                }, user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder, generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy9, Postgresql,
                    }, Repository
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::refresh_channel_token::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_RefreshChannelToken;
impl ActionProcessor_ for ActionProcessor<Channel_RefreshChannelToken> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Encoder::<ChannelToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                &incoming.channel_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_token_signed.channel_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__AlreadyExpired));
            }
            let (
                channel__owner,
                is_user_the_channel_subscriber,
            ) = match Repository::<Postgresql<Channel>>::find_8(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy9 {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel_token_signed.channel__id,
                }
            )
            .await? {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound))
            };
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_token_signed: Encoder::<ChannelToken>::encode(
                            &inner.environment_configuration.subject.encryption.private_key,
                            incoming.user_access_token_signed.user__id,
                            incoming.channel_token_signed.channel__id,
                            Generator::<ChannelToken_ObfuscationValue>::generate(),
                            Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                            is_user_the_channel_subscriber,
                            incoming.user_access_token_signed.user__id == channel__owner,
                        )?,
                    },
                ),
            );
        };
    }
}
