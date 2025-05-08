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
                    ChannelPublication1,
                    ChannelPublication1_ImagesPathes,
                    ChannelPublication1_Text,
                }, channel_publication1_token::{
                    ChannelPublication1Token,
                    ChannelPublication1Token_ExpiresAt,
                    ChannelPublication1Token_ObfuscationValue,
                }, channel_token::ChannelToken, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelPublication1Insert, Postgresql,
                    }, Repository,
                },
                service::{resolver::{
                    Resolver,
                    UnixTime,
                }, spawner::{Spawner, TokioBlockingTask}},
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Create;
impl ActionProcessor_ for ActionProcessor<Create> {
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
            if incoming.channel_publication1__text.is_none() && incoming.channel_publication1__images_pathes.is_empty() {
                return Result::Err(crate::new_invalid_argument!());
            }
            if let Option::Some(channel_publication1__text) = incoming.channel_publication1__text {
                if !Validator::<ChannelPublication1_Text>::is_valid(channel_publication1__text) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            if !incoming.channel_publication1__images_pathes.is_empty() {
                if !Validator::<ChannelPublication1_ImagesPathes>::is_valid(incoming.channel_publication1__images_pathes.as_slice()) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            if !incoming.channel_token_signed.channel_token__is_user_the_channel_owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User__IsNotChannelOwner));
            }
            let channel_publication1__id = match Repository::<Postgresql<ChannelPublication1>>::create(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelPublication1Insert {
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel_publication1__images_pathes: incoming.channel_publication1__images_pathes.as_slice(),
                    channel_publication1__text: incoming.channel_publication1__text,
                    channel_publication1__commentaries_quantity: 0,
                    channel_publication1__marks_quantity: 0,
                    channel_publication1__view_quantity: 0,
                    channel_publication1__created_at: now,
                    channel_publication1__can_be_deleted_from: 0,
                },
            )
            .await?
            {
                Option::Some(channel_publication1__id_) => channel_publication1__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution)),
            };
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_publication1__created_at: now,
                        channel_publication1_token_signed: crate::result_return_runtime!(
                            Spawner::<TokioBlockingTask>::spawn_processed(
                                move || -> _ {
                                    return Encoder::<ChannelPublication1Token>::encode(
                                        private_key,
                                        incoming.user_access_token_signed.user__id,
                                        incoming.channel_token_signed.channel__id,
                                        channel_publication1__id,
                                        Generator::<ChannelPublication1Token_ObfuscationValue>::generate(),
                                        Generator::<ChannelPublication1Token_ExpiresAt>::generate(now)?,
                                    );
                                },
                            ).await
                        )?,
                    },
                ),
            );
        };
    }
}
