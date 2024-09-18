use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::{
                Channel as EntityChannel,
                Channel_AccessModifier,
                Channel_Id,
            },
            channel_inner_link::ChannelInnerLink,
            channel_outer_link::ChannelOuterLink,
            channel_subscription::ChannelSubscription,
        },
        functionality::service::{
            extractor::{
                application_user_access_token::Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::repository::postgresql::{
            channel::By1 as By1___,
            channel_inner_link::By1 as By1__,
            channel_outer_link::By1 as By1_,
            channel_subscription::By1,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::{
    action_processor::channel___base::get_one_by_id::{
        Incoming,
        Outcoming,
        Precedent,
    },
    Channel2,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct Channel__Base___GetOneById;
impl ActionProcessor_ for ActionProcessor<Channel__Base___GetOneById> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )? {
                Extracted::ApplicationUserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                Extracted::ApplicationUserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
                }
                Extracted::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let channel = match PostgresqlRepository::<EntityChannel<'_>>::find_1(
                &*database_1_postgresql_pooled_connection,
                By1___ {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Some(channel_) => channel_,
                None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound));
                }
            };
            if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
                let is_exist = PostgresqlRepository::<ChannelSubscription>::is_exist_1(
                    &*database_1_postgresql_pooled_connection,
                    By1 {
                        application_user__id: application_user_access_token.application_user__id,
                        channel__id: channel.id,
                    },
                )
                .await?;
                if !is_exist && application_user_access_token.application_user__id != channel.owner {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
                }
            }
            let channel_inner_link_registry = PostgresqlRepository::<ChannelInnerLink>::find_1(
                &*database_1_postgresql_pooled_connection,
                By1__ {
                    channel_inner_link__from: channel.id,
                },
                ChannelInnerLink::MAXIMUM_QUANTITY,
            )
            .await?;
            let channel_outer_link_registry = PostgresqlRepository::<ChannelOuterLink>::find_1(
                &*database_1_postgresql_pooled_connection,
                By1_ {
                    channel_outer_link__from: channel.id,
                },
                ChannelOuterLink::MAXIMUM_QUANTITY,
            )
            .await?;
            let channel_2 = Channel2 {
                channel__owner: channel.owner,
                channel__name: channel.name.into_owned(),
                channel__linked_name: channel.linked_name,
                channel__description: channel.description,
                channel__access_modifier: channel.access_modifier,
                channel__visability_modifier: channel.visability_modifier,
                channel__orientation: channel.orientation,
                channel__cover_image_path: channel.cover_image_path,
                channel__background_image_path: channel.background_image_path,
                channel__subscribers_quantity: channel.subscribers_quantity,
                channel__marks_quantity: channel.marks_quantity,
                channel__viewing_quantity: channel.viewing_quantity,
            };
            let outcoming = Outcoming {
                channel: channel_2,
                channel_inner_link_registry,
                channel_outer_link_registry,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
