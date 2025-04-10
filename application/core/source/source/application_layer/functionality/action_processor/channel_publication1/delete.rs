use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::Channel,
                channel_publication1::ChannelPublication1,
                channel_publication1_delayed_deletion::{ChannelPublication1DelayedDeletion, ChannelPublication1DelayedDeletion_CanBeDeletedFrom},
                channel_publication1_token::ChannelPublication1Token,
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
                    postgresql::{
                        ChannelBy1, ChannelPublication1By1, ChannelPublication1DelayedDeletionInsert, IsolationLevel, Postgresql, Resolver as Resolver_, Transaction
                    }, Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::delete::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ChannelPublication1_Delete;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_Delete> {
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
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
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
            let mut postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let channel__owner = match Repository::<Postgresql<Channel>>::find_7(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel_publication1_token_signed.channel__id,
                },
            )
            .await?
            {
                Option::Some(channel__owner_) => channel__owner_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound)),
            };
            if incoming.user_access_token_signed.user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User__IsNotChannelOwner));
            }
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_3_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_deleted = match Repository::<Postgresql<ChannelPublication1>>::delete(
                transaction.get_client(),
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                }
            )
            .await
            {
                Result::Ok(is_deleted_) => is_deleted_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_deleted {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1__NotFound));
            }
            let is_created = match Repository::<Postgresql<ChannelPublication1DelayedDeletion>>::create(
                transaction.get_client(),
                ChannelPublication1DelayedDeletionInsert {
                    channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_delayed_deletion__can_be_deleted_from: Generator::<ChannelPublication1DelayedDeletion_CanBeDeletedFrom>::generate(now)?,
                    channel_publication1_delayed_deletion__created_at: now,
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1__NotFound));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
