use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::{
                    Channel,
                    Channel_AccessModifier_,
                },
                channel_subscription_token::{
                    ChannelSubscriptionToken,
                    ChannelSubscriptionToken_ExpiresAt,
                    ChannelSubscriptionToken_ObfuscationValue,
                },
                channel_token::ChannelToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy1,
                        Postgresql,
                    },
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::get_one_by_id::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetOneById;
impl ActionProcessor_ for ActionProcessor<Channel_GetOneById> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
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
            let postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (
                channel__owner,
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
                channel__orientation,
                channel__cover_image_path,
                channel__background_image_path,
                channel__subscribers_quantity,
            ) = match Repository::<Postgresql<Channel>>::find_1(
                &postgresql_client_database_3,
                ChannelBy1 {
                    channel__id: incoming.channel_token_signed.channel__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound)),
            };
            if incoming.user_access_token_signed.user__id != channel__owner
                && channel__access_modifier == Channel_AccessModifier_::Close as i16
                && !incoming.channel_token_signed.channel_token__is_channel_subscription_exist {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__IsClose));
            }
            let outcoming = Outcoming {
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
                channel__orientation,
                channel__cover_image_path,
                channel__background_image_path,
                channel__subscribers_quantity,
                user_is_channel_owner: incoming.user_access_token_signed.user__id == channel__owner,
                channel_subscription_token_signed: Encoder::<ChannelSubscriptionToken>::encode(
                    &inner.environment_configuration.subject.encryption.private_key,
                    incoming.user_access_token_signed.user__id,
                    incoming.channel_token_signed.channel__id,
                    Generator::<ChannelSubscriptionToken_ObfuscationValue>::generate(),
                    Generator::<ChannelSubscriptionToken_ExpiresAt>::generate(now)?,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
