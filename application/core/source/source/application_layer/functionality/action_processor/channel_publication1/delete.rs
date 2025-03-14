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
                channel_publication1::{
                    ChannelPublication1,
                    ChannelPublication1_Id,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy1,
                        ChannelPublication1By1,
                        ChannelPublication1Update,
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
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.user_access_token_signed.user_access_token__expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
            }
            if !Validator::<ChannelPublication1_Id>::is_valid(incoming.channel_publication1__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (channel__id, channel_publication1__is_predeleted) = match Repository::<Postgresql<ChannelPublication1>>::find_2(
                &postgresql_database_3_client,
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1_NotFound)),
            };
            if channel_publication1__is_predeleted {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1_IsAlreadyDeleted));
            }
            let channel__owner = match Repository::<Postgresql<Channel>>::find_7(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id,
                },
            )
            .await?
            {
                Option::Some(channel__owner_) => channel__owner_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner)),
            };
            if incoming.user_access_token_signed.user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner));
            }
            Repository::<Postgresql<ChannelPublication1>>::update(
                &postgresql_database_3_client,
                ChannelPublication1Update {
                    channel_publication1__is_predeleted: true,
                },
                ChannelPublication1By1 {
                    channel_publication1__id: incoming.channel_publication1__id,
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
