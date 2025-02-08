use {
    crate::{
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
            data::aggregate_error::AggregateError,
            functionality::repository::{
                postgresql::{
                    Postgresql,
                    UserAccessRefreshTokenBy2,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::deauthorize_from_one_device::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct UserAuthorization_DeauthorizeFromOneDevice;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_DeauthorizeFromOneDevice> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
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
            Repository::<Postgresql<UserAccessRefreshToken<'_>>>::delete_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await),
                UserAccessRefreshTokenBy2 {
                    user__id: user_access_token.user__id,
                    user_device__id: user_access_token.user_device__id,
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
