use crate::infrastructure_layer::data::control_type::ChannelSubscription__Base___Create;
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
            auditor::{
                Auditor,
                Backtrace,
                ErrorConverter,
                OptionConverter,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
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
    ) -> Result<Result<UnifiedReport<Void, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
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
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        let channel = match PostgresqlRepository::<Channel<'_>>::find_1(
            database_1_postgresql_connection,
            By1 {
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
        if channel.owner == application_user_access_token.application_user_id {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUser_IsChannelOwner,
            )));
        }
        if let Channel_AccessModifier::Close = Channel_AccessModifier::to_representation(channel.access_modifier) {
            return Ok(Ok(UnifiedReport::precedent(Precedent::Channel_IsClose)));
        }
        PostgresqlRepository::<ChannelSubscription>::create_1(
            database_1_postgresql_connection,
            Insert1 {
                application_user_id: application_user_access_token.application_user_id,
                channel_id: channel.id,
            },
        )
        .await?;
        return Ok(Ok(UnifiedReport::target_empty()));
    }
}
