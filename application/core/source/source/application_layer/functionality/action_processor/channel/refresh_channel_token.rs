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
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy9, Postgresql,
                    }, Repository
                },
                service::{resolver::{
                    Resolver,
                    UnixTime,
                }, task_spawner::TaskSpawner},
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
pub struct RefreshChannelToken;
impl ActionProcessor_ for ActionProcessor<RefreshChannelToken> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            let sended = Sended_::new(&raw const incoming as *const Self::Incoming<'static>);
            if let Option::Some(precedent) = crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> Result<Option<Precedent>, AggregateError> {
                        let incoming_ = unsafe { sended.read_() };
                        if !Encoder::<UserAccessToken>::is_valid(
                            private_key,
                            &incoming_.user_access_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.user_access_token_signed.user_access_token__expires_at <= now {
                            return Result::Ok(Option::Some(Precedent::UserAccessToken__AlreadyExpired));
                        }
                        if !Encoder::<ChannelToken>::is_valid(
                            private_key,
                            incoming_.user_access_token_signed.user__id,
                            &incoming_.channel_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.channel_token_signed.channel_token__expires_at <= now {
                            return Result::Ok(Option::Some(Precedent::ChannelToken__AlreadyExpired));
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
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
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_token_signed: crate::result_return_runtime!(
                            TaskSpawner::spawn_rayon_task_processed(
                                move || -> _ {
                                    return Encoder::<ChannelToken>::encode(
                                        private_key,
                                        incoming.user_access_token_signed.user__id,
                                        incoming.channel_token_signed.channel__id,
                                        Generator::<ChannelToken_ObfuscationValue>::generate(),
                                        Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                                        is_user_the_channel_subscriber,
                                        incoming.user_access_token_signed.user__id == channel__owner,
                                    );
                                }
                            ).await
                        )?,
                    },
                ),
            );
        };
    }
}
