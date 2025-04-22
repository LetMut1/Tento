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
                channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
                    ChannelToken_ObfuscationValue,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder, generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                       ChannelBy8, Postgresql
                    }, Repository
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::get_many_owned::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetManyOwned;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyOwned> {
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
            const LIMIT: i16 = Channel::OWNED_QUANTITY_PER_USER as i16;
            static_assertions::const_assert!(
                LIMIT <= 30
            );
            let rows = Repository::<Postgresql<Channel>>::find_7(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy8 {
                    channel__owner: incoming.user_access_token_signed.user__id,
                },
                LIMIT,
            )
            .await?;
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
                        &inner.environment_configuration.subject.encryption.private_key,
                        incoming.user_access_token_signed.user__id,
                        crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                        Generator::<ChannelToken_ObfuscationValue>::generate(),
                        Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                        false,
                        true,
                    )?,
                };
                data_registry.push(data);
            }
            let outcoming = Outcoming {
                data_registry,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
