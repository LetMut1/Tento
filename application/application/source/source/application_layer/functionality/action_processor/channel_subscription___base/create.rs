use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::{
                Channel,
                Channel_AccessModifier,
                Channel_Id,
            },
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
            alternative_workflow::{
                AlternativeWorkflow,
                OptionConverter,
                ResultConverter,
            },
            auditor::Backtrace,
            control_type::ChannelSubscription__Base___Create,
            environment_configuration::EnvironmentConfiguration,
            void::Void,
        },
        functionality::repository::postgresql::{
            channel::By1,
            channel_subscription::Insert1,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::{
    Incoming,
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
impl ActionProcessor<ChannelSubscription__Base___Create> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Void, Precedent>, AlternativeWorkflow>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.into_internal_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration,
            incoming_.application_user_access_token_encrypted.as_str(),
        )
        .await?
        {
            ExtractorResult::ApplicationUserAccessToken {
                application_user_access_token: application_user_access_token_,
            } => application_user_access_token_,
            ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
            }
            ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
            }
        };
        if !Validator::<Channel_Id>::is_valid(incoming_.channel__id) {
            return Err(
                AlternativeWorkflow::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        let channel = match PostgresqlRepository::<Channel<'_>>::find_1(
            database_1_postgresql_connection,
            By1 {
                channel__id: incoming_.channel__id,
            },
        )
        .await?
        {
            Some(channel_) => channel_,
            None => {
                return Ok(UnifiedReport::precedent(Precedent::Channel_NotFound));
            }
        };
        if channel.owner == application_user_access_token.application_user__id {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_IsChannelOwner));
        }
        if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
            return Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
        }
        PostgresqlRepository::<ChannelSubscription>::create_1(
            database_1_postgresql_connection,
            Insert1 {
                application_user__id: application_user_access_token.application_user__id,
                channel__id: channel.id,
            },
        )
        .await?;
        return Ok(UnifiedReport::target_empty());
    }
}
