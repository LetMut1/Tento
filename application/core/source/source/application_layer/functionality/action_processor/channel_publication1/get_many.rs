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
                channel_publication1::ChannelPublication1,
                channel_publication1_token::{
                    ChannelPublication1Token,
                    ChannelPublication1Token_ExpiresAt,
                    ChannelPublication1Token_ObfuscationValue,
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
            data::{
                aggregate_error::AggregateError,
                sended::Sended_,
            },
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy1,
                        ChannelPublication1By2,
                        Postgresql,
                    },
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    task_spawner::TaskSpawner,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::get_many::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct GetMany;
impl ActionProcessor_ for ActionProcessor<GetMany> {
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
            let (channel__owner, channel__access_modifier) = match Repository::<Postgresql<Channel>>::find_6(
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
            if incoming.channel_token_signed.channel_token__is_user_the_channel_owner && incoming.user_access_token_signed.user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__InvalidChannelOwnerDefinition));
            }
            if !incoming.channel_token_signed.channel_token__is_user_the_channel_owner
                && !incoming.channel_token_signed.channel_token__is_user_the_channel_subscriber
                && Channel_AccessModifier_::Close as u8 == channel__access_modifier
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__IsClose));
            }
            const LIMIT: i16 = 30;
            let rows = Repository::<Postgresql<ChannelPublication1>>::find_1(
                &postgresql_client_database_3,
                ChannelPublication1By2 {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel_publication1__created_at: incoming.channel_publication1__created_at,
                },
                LIMIT,
            )
            .await?;
            let data_registry = if rows.is_empty() {
                vec![]
            } else {
                let private_key = &inner.environment_configuration.subject.encryption.private_key;
                crate::result_return_runtime!(
                    TaskSpawner::spawn_rayon_task_processed(
                        move || -> _ {
                            let mut data_registry: Vec<Data> = Vec::with_capacity(rows.len());
                            '_a: for row in rows.iter() {
                                let channel_publication1__commentaries_quantity = crate::result_return_logic!(row.try_get::<'_, usize, i64>(3));
                                if channel_publication1__commentaries_quantity < u32::MIN as i64 || channel_publication1__commentaries_quantity > u32::MAX as i64 {
                                    return Result::Err(crate::new_logic_unreachable_state!());
                                }
                                let channel_publication1__marks_quantity = crate::result_return_logic!(row.try_get::<'_, usize, i64>(4));
                                if channel_publication1__marks_quantity < u32::MIN as i64 || channel_publication1__marks_quantity > u32::MAX as i64 {
                                    return Result::Err(crate::new_logic_unreachable_state!());
                                }
                                let channel_publication1__view_quantity = crate::result_return_logic!(row.try_get::<'_, usize, i64>(5));
                                if channel_publication1__view_quantity < u32::MIN as i64 || channel_publication1__view_quantity > u32::MAX as i64 {
                                    return Result::Err(crate::new_logic_unreachable_state!());
                                }
                                let channel_publication1__id = crate::result_return_logic!(row.try_get::<'_, usize, i64>(0));
                                data_registry.push(
                                    Data {
                                        channel_publication1__images_pathes: crate::result_return_logic!(row.try_get::<'_, usize, Vec<String>>(1)),
                                        channel_publication1__text: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(2)),
                                        channel_publication1__commentaries_quantity: channel_publication1__commentaries_quantity as u32,
                                        channel_publication1__marks_quantity: channel_publication1__marks_quantity as u32,
                                        channel_publication1__view_quantity: channel_publication1__view_quantity as u32,
                                        channel_publication1__created_at: crate::result_return_logic!(row.try_get::<'_, usize, i64>(6)),
                                        channel_publication1_marked_view__marked_at: crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(7)),
                                        channel_publication1_token_signed: Encoder::<ChannelPublication1Token>::encode(
                                            private_key,
                                            incoming.user_access_token_signed.user__id,
                                            incoming.channel_token_signed.channel__id,
                                            channel_publication1__id,
                                            Generator::<ChannelPublication1Token_ObfuscationValue>::generate(),
                                            Generator::<ChannelPublication1Token_ExpiresAt>::generate(now)?,
                                        )?,
                                    },
                                );
                            }
                            return Ok(data_registry);
                        },
                    ).await
                )?
            };
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        data_registry,
                    },
                ),
            );
        };
    }
}
