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
            Extracted,
            Extractor,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                user_access_refresh_token::By1,
                PostgresqlRepository,
            },
            service::resolver::{
                CloudMessage,
                Resolver,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::user_authorization::deauthorize_from_all_devices::{
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
pub struct UserAuthorization_DeauthorizeFromAllDevices;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_DeauthorizeFromAllDevices> {
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
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: user_access_token_,
                } => user_access_token_,
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_2(
                &*database_2_postgresql_pooled_connection,
                By1 {
                    user__id: user_access_token.user__id,
                },
            )
            .await?;
            Resolver::<CloudMessage>::deauthorize_user_from_all_devices();
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
