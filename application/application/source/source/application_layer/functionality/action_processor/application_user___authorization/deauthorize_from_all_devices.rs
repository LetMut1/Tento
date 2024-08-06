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
            aggregate_error::{
                AggregateError,
            },
            control_type::{
                ApplicationUser__Authorization___DeauthorizeFromAllDevices,
                CloudMessage,
            },
            void::Void,
        },
        functionality::{
            repository::postgresql::{
                application_user_access_refresh_token::By1,
                PostgresqlRepository,
            },
            service::resolver::Resolver,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_all_devices::{
    Incoming,
    Precedent,
};
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
use crate::application_layer::functionality::action_processor::Inner;
use crate::application_layer::functionality::action_processor::ActionProcessor_;
use std::future::Future;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___DeauthorizeFromAllDevices> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a, T> (
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + 'a
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        async move {
            let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
                inner.environment_configuration,
                incoming.application_user_access_token_encrypted.as_str(),
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
                &*database_2_postgresql_pooled_connection,
                By1 {
                    application_user__id: application_user_access_token.application_user__id,
                },
            )
            .await?;
            Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();
            return Ok(UnifiedReport::target_empty());
        }
    }
}
