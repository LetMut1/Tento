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
                user_access_token::UserAccessToken,
                channel_publication1::{
                    ChannelPublication1,
                    ChannelPublication1_Id,
                }
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
                    Postgresql,
                    ChannelPublication1By1,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::delete::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ChannelPublication1_Delete;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_Delete> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
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
                Extracted::AlreadyExpired => return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired))
            };
            if !Validator::<ChannelPublication1_Id>::is_valid(incoming.channel_publication1__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let channel__id = match Repository::<Postgresql<ChannelPublication1>>::find_2(
                &postgresql_database_3_client,
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1__id,
                },
            ).await? {
                Option::Some(channel__id_) => channel__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1_NotFound))
            };
            let channel__owner = match Repository::<Postgresql<Channel>>::find_7(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id,
                },
            )
            .await?
            {
                Option::Some(channel__owner_) => channel__owner_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner))
            };
            if user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner));
            }
            Repository::<Postgresql<ChannelPublication1>>::delete(
                &postgresql_database_3_client,
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1__id,
                },
            ).await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}