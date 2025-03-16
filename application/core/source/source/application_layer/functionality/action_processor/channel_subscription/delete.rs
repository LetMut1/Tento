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
                    Channel_Id,
                },
                channel_subscription::ChannelSubscription,
                channel_subscription_token::ChannelSubscriptionToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy1,
                        ChannelSubscriptionBy,
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
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
        action_processor_incoming_outcoming::action_processor::channel_subscription::delete::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ChannelSubscription_Delete;
impl ActionProcessor_ for ActionProcessor<ChannelSubscription_Delete> {
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
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Encoder::<ChannelSubscriptionToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                incoming.channel__id,
                &incoming.channel_subscription_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_subscription_token_signed.channel_subscription_token__expires_at < now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscriptionToken__AlreadyExpired));
            }
            let mut postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            if !Repository::<Postgresql<ChannelSubscription>>::is_exist(
                &postgresql_database_3_client,
                ChannelSubscriptionBy {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscription__NotFound));
            }
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_3_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            if let Result::Err(aggregate_error) = Repository::<Postgresql<ChannelSubscription>>::delete(
                transaction.get_client(),
                ChannelSubscriptionBy {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel__id,
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<Channel>>::update_2(
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
