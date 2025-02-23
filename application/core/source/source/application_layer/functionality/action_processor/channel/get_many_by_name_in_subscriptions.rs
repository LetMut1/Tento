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
                    Channel_Name,
                    Channel,
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
                    ChannelBy5,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::get_many_by_name_in_subscriptions::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetManyByNameInSubscriptions;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyByNameInSubscriptions> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
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
            const LIMIT: i16 = 100;
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
            let data_registry = Repository::<Postgresql<Channel>>::find_4(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                ChannelBy5 {
                    user__id,
                    channel__name: incoming.channel__name.as_str(),
                    requery___channel__name: incoming.requery___channel__name.as_deref(),
                },
                incoming.limit,
            )
            .await?;
            let outcoming = Outcoming {
                data_registry,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
