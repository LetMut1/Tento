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
                channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
                    ChannelToken_ObfuscationValue,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
                validator::Validator,
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
                        ChannelBy6,
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
        action_processor_incoming_outcoming::action_processor::channel::get_many_by_subscription::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct GetManyBySubscription;
impl ActionProcessor_ for ActionProcessor<GetManyBySubscription> {
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
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            if let Option::Some(requery___channel__id_) = incoming.requery___channel__id {
                if !Validator::<Channel_Id>::is_valid(requery___channel__id_) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            const LIMIT: i16 = 30;
            let rows = Repository::<Postgresql<Channel>>::find_5(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy6 {
                    user__id: incoming.user_access_token_signed.user__id,
                    requery___channel__id: incoming.requery___channel__id,
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
                                let channel__access_modifier = crate::result_return_logic!(row.try_get::<'_, usize, i16>(3));
                                if channel__access_modifier < u8::MIN as i16 || channel__access_modifier > u8::MAX as i16 {
                                    return Result::Err(crate::new_logic_unreachable_state!());
                                }
                                let channel__visability_modifier = crate::result_return_logic!(row.try_get::<'_, usize, i16>(4));
                                if channel__visability_modifier < u8::MIN as i16 || channel__visability_modifier > u8::MAX as i16 {
                                    return Result::Err(crate::new_logic_unreachable_state!());
                                }
                                let data = Data {
                                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                                    channel__access_modifier: channel__access_modifier as u8,
                                    channel__visability_modifier: channel__visability_modifier as u8,
                                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                                    channel_token_signed: Encoder::<ChannelToken>::encode(
                                        private_key,
                                        incoming.user_access_token_signed.user__id,
                                        crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                                        Generator::<ChannelToken_ObfuscationValue>::generate(),
                                        Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                                        true,
                                        false,
                                    )?,
                                };
                                data_registry.push(data);
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
