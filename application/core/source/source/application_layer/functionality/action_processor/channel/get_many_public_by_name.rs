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
                    Channel_VisabilityModifier,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                extractor::{
                    Extracted,
                    Extractor,
                },
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::{
                postgresql::{
                    ChannelBy4,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::get_many_public_by_name::{
            Incoming,
            Outcoming,
            Precedent,
            Data,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetManyPublicByName;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyPublicByName> {
    type Incoming<'a> = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user__id = match Extractor::<UserAccessToken>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::Data {
                    user_access_token__id: _,
                    user__id: user__id_,
                    user_device__id: _,
                    user_access_token__expires_at: _,
                } => user__id_,
                Extracted::AlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::InUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            const LIMIT: i16 = 15;
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if let Option::Some(ref requery___channel__name_) = incoming.requery___channel__name {
                if !Validator::<Channel_Name>::is_valid(requery___channel__name_.as_str()) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            let channel__visability_modifier = Channel_VisabilityModifier::Public as i16;
            let rows = Repository::<Postgresql<Channel>>::find_3(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                ChannelBy4 {
                    user__id,
                    channel__name: incoming.channel__name.as_str(),
                    requery___channel__name: incoming.requery___channel__name.as_deref(),
                    channel__visability_modifier,
                },
                incoming.limit,
            )
            .await?;
            let mut data_registry: Vec<Data> = vec![];
            '_a: for row in rows.iter() {
                let data = Data {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier,
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(4)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    is_user_subscribed: crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(6)).is_some(),
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
