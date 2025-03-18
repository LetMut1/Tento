use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel_publication1::{
                    ChannelPublication1, ChannelPublication1_Id,
                }, channel_publication1_token::ChannelPublication1Token, channel_publication1_view::ChannelPublication1View, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder, validator::Validator
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelPublication1By1, ChannelPublication1ViewInsert, Postgresql,
                    },
                    Repository,
                },
                service::{resolver::{
                    Resolver,
                    UnixTime,
                }, spawner::{Spawner, TokioNonBlockingTask}}
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1_view::create::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ChannelPublication1View_Create;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1View_Create> {
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
            if !Validator::<ChannelPublication1_Id>::is_valid(incoming.channel_publication1__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Encoder::<ChannelPublication1Token>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                incoming.channel_publication1__id,
                &incoming.channel_publication1_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_publication1_token_signed.channel_publication1_token__expires_at < now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Token__AlreadyExpired));
            }
            let mut postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let _ = Repository::<Postgresql<ChannelPublication1View>>::create(
                        &crate::result_return_runtime!(postgresql_connection_pool_database_3.get().await),
                        ChannelPublication1ViewInsert {
                            user__id: incoming.user_access_token_signed.user__id,
                            channel_publication1__id: incoming.channel_publication1__id,
                            channel_publication1_view__created_at: now,
                        }
                    ).await?;
                    return Result::Ok(());
                },
            );
            postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let _ = Repository::<Postgresql<ChannelPublication1>>::update_4(
                        &crate::result_return_runtime!(postgresql_connection_pool_database_3.get().await),
                        ChannelPublication1By1 {
                            channel_publication1__id: incoming.channel_publication1__id,
                        }
                    ).await?;
                    return Result::Ok(());
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
