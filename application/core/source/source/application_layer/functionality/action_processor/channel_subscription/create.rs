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
                    Channel_Id,
                },
                channel_subscription::ChannelSubscription,
                user_access_token::UserAccessToken,
                channel_subscription_token::ChannelSubscriptionToken,
            },
            functionality::service::{
                validator::Validator,
                encoder::Encoder,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy1,
                        ChannelSubscriptionInsert,
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                    },
                    Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_subscription::create::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ChannelSubscription_Create;
impl ActionProcessor_ for ActionProcessor<ChannelSubscription_Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
            }
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (
                channel__owner,
                channel__access_modifier,
                channel__obfuscation_value,
            ) = match Repository::<Postgresql<Channel>>::find_2(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound))
            };
            if !Encoder::<ChannelSubscriptionToken>::is_valid(
                incoming.user_access_token_signed.user__id,
                incoming.channel__id,
                channel__obfuscation_value,
                &incoming.channel_subscription_token_hashed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_subscription_token_hashed.channel_subscription_token__expires_at < now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscriptionToken_AlreadyExpired));
            }
            if incoming.user_access_token_signed.user__id == channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_IsChannelOwner));
            }
            if Channel_AccessModifier_::Close as i16 == channel__access_modifier {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
            }
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_3_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_created = match Repository::<Postgresql<ChannelSubscription>>::create(
                transaction.get_client(),
                ChannelSubscriptionInsert {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel__id,
                    channel_subscription__created_at: now,
                }
            )
            .await
            {
                Result::Ok(is_created_) => is_created_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_created {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscription_AlreadyExist));
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<Channel>>::update_1(
                transaction.get_client(),
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}