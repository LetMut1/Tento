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
                    Channel_Id,
                },
                user_access_token::UserAccessToken,
                channel_publication1::ChannelPublication1,
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
                    ChannelBy1,
                    ChannelPublication1By2,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::get_many::{
            Incoming,
            Precedent,
            Outcoming
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct ChannelPublication1_GetMany;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_GetMany> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user__id = match Extractor::<UserAccessToken>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
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
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            const LIMIT: i16 = 30;
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (
                channel__owner,
                channel__access_modifier,
            ) = match Repository::<Postgresql<Channel>>::find_6(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound));
                }
            };
            if user__id != channel__owner {
                if Channel_AccessModifier_::Close as i16 == channel__access_modifier {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
                }
            }
            let data_registry = Repository::<Postgresql<ChannelPublication1>>::find(
                &postgresql_database_3_client,
                ChannelPublication1By2 {
                    channel__id: incoming.channel__id,
                    channel_publication1__created_at: incoming.channel_publication1__created_at,
                },
                incoming.limit,
            ).await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        data_registry,
                    }
                )
            );
        };
    }
}