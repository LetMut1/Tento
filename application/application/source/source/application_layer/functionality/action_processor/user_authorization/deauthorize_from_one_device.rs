use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            user_access_refresh_token::UserAccessRefreshToken,
            user_access_token::UserAccessToken,
        },
        functionality::service::extractor::{
            application_user_access_token::Extracted,
            Extractor,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::repository::postgresql::{
            application_user_access_refresh_token::By2,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::user_authorization::deauthorize_from_one_device::{
    Incoming,
    Precedent,
};
use aggregate_error::AggregateError;
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
pub struct UserAuthorization_DeauthorizeFromOneDevice;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_DeauthorizeFromOneDevice> {
    type Incoming = Incoming;
    type Outcoming = Void;
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
            let application_user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_1(
                &*database_2_postgresql_pooled_connection,
                By2 {
                    application_user__id: application_user_access_token.application_user__id,
                    application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}