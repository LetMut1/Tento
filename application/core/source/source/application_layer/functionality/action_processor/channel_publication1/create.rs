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
                }, channel_publication1::{
                    ChannelPublication1,
                    ChannelPublication1_ImagesPathes,
                    ChannelPublication1_Text,
                }, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder, validator::Validator
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy1,
                        ChannelPublication1Insert,
                        Postgresql,
                    },
                    Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                }
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::create::{
            Incoming, Outcoming, Precedent
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct ChannelPublication1_Create;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_Create> {
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
            if incoming.user_access_token_signed.user_access_token__expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
            }
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<ChannelPublication1_Text>::is_valid(incoming.channel_publication1__text) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<ChannelPublication1_ImagesPathes>::is_valid(incoming.channel_publication1__images_pathes.as_slice()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let channel__owner = match Repository::<Postgresql<Channel>>::find_7(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(channel__owner_) => channel__owner_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound))
            };
            if incoming.user_access_token_signed.user__id != channel__owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner));
            }
            let channel_publication1__created_at = Resolver::<UnixTime>::get_now_in_seconds();
            let channel_publication1__id = Repository::<Postgresql<ChannelPublication1>>::create(
                &postgresql_database_3_client,
                ChannelPublication1Insert {
                    channel__id: incoming.channel__id,
                    channel_publication1__images_pathes: incoming.channel_publication1__images_pathes.as_slice(),
                    channel_publication1__text: incoming.channel_publication1__text,
                    channel_publication1__marks_quantity: 0,
                    channel_publication1__viewing_quantity: 0,
                    channel_publication1__created_at,
                },
            ).await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_publication1__id,
                        channel_publication1__created_at,
                    }
                )
            );
        };
    }
}