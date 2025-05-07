use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel_publication1::ChannelPublication1,
                channel_publication1_mark::ChannelPublication1Mark,
                channel_publication1_token::ChannelPublication1Token,
                user_access_token::UserAccessToken,
            },
            functionality::service::encoder::Encoder,
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelPublication1By1,
                        ChannelPublication1MarkInsert,
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
        action_processor_incoming_outcoming::action_processor::channel_publication1_mark::create::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct Create;
impl ActionProcessor_ for ActionProcessor<Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
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
            if !Encoder::<ChannelPublication1Token>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                &incoming.channel_publication1_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_publication1_token_signed.channel_publication1_token__expires_at < now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Token__AlreadyExpired));
            }
            let mut postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_3,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_created = match Repository::<Postgresql<ChannelPublication1Mark>>::create(
                transaction.get_client(),
                ChannelPublication1MarkInsert {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_mark__created_at: now,
                },
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Mark__AlreadyExist));
            }
            let is_updated = match Repository::<Postgresql<ChannelPublication1>>::update_1(
                transaction.get_client(),
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                },
            )
            .await
            {
                Result::Ok(is_updated_) => is_updated_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_updated {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1__NotFound));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
