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
                    Channel_Name,
                    Channel_VisabilityModifier_,
                },
                channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy4,
                        Postgresql,
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
        action_processor_incoming_outcoming::action_processor::channel::get_many_public_by_name::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetManyPublicByName;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyPublicByName> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
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
            const LIMIT: i16 = 15;
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if let Option::Some(requery___channel__name_) = incoming.requery___channel__name {
                if !Validator::<Channel_Name>::is_valid(requery___channel__name_) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            let rows = Repository::<Postgresql<Channel>>::find_3(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy4 {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__name: incoming.channel__name,
                    requery___channel__name: incoming.requery___channel__name,
                    channel__visability_modifier: Channel_VisabilityModifier_::Public as i16,
                },
                incoming.limit,
            )
            .await?;
            let mut data_registry: Vec<Data> = Vec::with_capacity(rows.len());
            '_a: for row in rows.iter() {
                let channel__id = crate::result_return_logic!(row.try_get::<'_, usize, i64>(0));
                let channel_token_hashed_for_unsubscribed_users = if crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(7)).is_some() {
                    Option::None
                } else {
                    Option::Some(
                        Encoder::<ChannelToken>::encode(
                            incoming.user_access_token_signed.user__id,
                            channel__id,
                            crate::result_return_logic!(row.try_get::<'_, usize, i64>(6)),
                            Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                        )?,
                    )
                };
                let data = Data {
                    channel__id,
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(4)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel_token_hashed_for_unsubscribed_users,
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
