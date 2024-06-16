pub use crate::infrastructure_layer::data::control_type::Channel__Base___GetOneById;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
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
                application_user_access_token::ExtractorResult,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
                ErrorConverter,
                OptionConverter,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
        },
        functionality::repository::postgresql::{
            channel::By1 as By1___,
            channel_inner_link::By1 as By1__,
            channel_outer_link::By1 as By1_,
            channel_subscription::By1,
            PostgresqlRepository,
        },
    },
};
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_one_by_id::{
    Channel2,
    Incoming,
    Outcoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor<Channel__Base___GetOneById> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;
        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration,
            incoming_.application_user_access_token_encrypted.as_str(),
        )
        .await?
        {
            Ok(extractor_result) => {
                let application_user_access_token_ = match extractor_result {
                    ExtractorResult::ApplicationUserAccessToken {
                        application_user_access_token: application_user_access_token__,
                    } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(Ok(UnifiedReport::precedent(
                            Precedent::ApplicationUserAccessToken_AlreadyExpired,
                        )));
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(Ok(UnifiedReport::precedent(
                            Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
                        )));
                    }
                };
                application_user_access_token_
            }
            Err(invalid_argument_auditor) => {
                return Ok(Err(invalid_argument_auditor));
            }
        };
        if !Validator::<Channel_Id>::is_valid(incoming_.channel_id) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let channel = match PostgresqlRepository::<EntityChannel<'_>>::find_1(
            &*database_1_postgresql_pooled_connection,
            By1___ {
                channel_id: incoming_.channel_id,
            },
        )
        .await?
        {
            Some(channel_) => channel_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::Channel_NotFound)));
            }
        };
        if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
            let is_exist = PostgresqlRepository::<ChannelSubscription>::is_exist_1(
                &*database_1_postgresql_pooled_connection,
                By1 {
                    application_user_id: application_user_access_token.application_user_id,
                    channel_id: channel.id,
                },
            )
            .await?;
            if !is_exist && application_user_access_token.application_user_id != channel.owner {
                return Ok(Ok(UnifiedReport::precedent(Precedent::Channel_IsClose)));
            }
        }
        let channel_inner_link_registry = PostgresqlRepository::<ChannelInnerLink>::find_1(
            &*database_1_postgresql_pooled_connection,
            By1__ {
                channel_inner_link_from: channel.id,
            },
            ChannelInnerLink::MAXIMUM_QUANTITY,
        )
        .await?;
        let channel_outer_link_registry = PostgresqlRepository::<ChannelOuterLink>::find_1(
            &*database_1_postgresql_pooled_connection,
            By1_ {
                channel_outer_link_from: channel.id,
            },
            ChannelOuterLink::MAXIMUM_QUANTITY,
        )
        .await?;
        let channel_2 = Channel2 {
            channel_owner: channel.owner,
            channel_name: channel.name.into_owned(),
            channel_linked_name: channel.linked_name,
            channel_description: channel.description,
            channel_access_modifier: channel.access_modifier,
            channel_visability_modifier: channel.visability_modifier,
            channel_orientation: channel.orientation,
            channel_cover_image_path: channel.cover_image_path,
            channel_background_image_path: channel.background_image_path,
            channel_subscribers_quantity: channel.subscribers_quantity,
            channel_marks_quantity: channel.marks_quantity,
            channel_viewing_quantity: channel.viewing_quantity,
        };
        let outcoming = Outcoming {
            channel: channel_2,
            channel_inner_link_registry,
            channel_outer_link_registry,
        };
        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
