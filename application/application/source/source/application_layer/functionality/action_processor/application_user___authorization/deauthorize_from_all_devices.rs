use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___DeauthorizeFromAllDevices;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
            application_user_access_token::ApplicationUserAccessToken,
        },
        functionality::service::extractor::{
            application_user_access_token::ExtractorResult,
            Extractor,
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
        functionality::{
            repository::postgresql::{
                application_user_access_refresh_token::By1,
                PostgresqlRepository,
            },
            service::resolver::{
                cloud_message::CloudMessage,
                Resolver,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_all_devices::{
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
impl ActionProcessor<ApplicationUser__Authorization___DeauthorizeFromAllDevices> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        // TODO TODO TODO УДАляются ли АккессТокены все при массовом разлогине? Если не удаляются, можно просто при Ектракте АккессТокена использовать проверку на наличие рефреша, если нет, значит произошел разлогин.
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
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
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
            &*database_2_postgresql_pooled_connection,
            By1 {
                application_user_id: application_user_access_token.application_user_id,
            },
        )
        .await?;
        Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();
        return Ok(Ok(UnifiedReport::target_empty()));
    }
}
