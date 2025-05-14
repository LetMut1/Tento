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
                channel_token::ChannelToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::encoder::Encoder,
        },
        infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy1,
                        Postgresql,
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
        action_processor_incoming_outcoming::action_processor::channel::get_one_by_id::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct GetOneById;
impl ActionProcessor_ for ActionProcessor<GetOneById> {
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
            let postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (
                channel__owner,
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
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
            if incoming.channel_token_signed.channel_token__is_user_the_channel_owner
                && incoming.user_access_token_signed.user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__InvalidChannelOwnerDefinition));
            }
            if !incoming.channel_token_signed.channel_token__is_user_the_channel_owner
                && channel__access_modifier == Channel_AccessModifier_::Close as u8
                && !incoming.channel_token_signed.channel_token__is_user_the_channel_subscriber {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__IsClose));
            }
            let outcoming = Outcoming {
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
                channel__cover_image_path,
                channel__background_image_path,
                channel__subscribers_quantity,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
