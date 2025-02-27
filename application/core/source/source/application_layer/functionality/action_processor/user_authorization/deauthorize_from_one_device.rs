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
    type Incoming<'a> = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let (user__id, user_device__id) = match Extractor::<UserAccessToken>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                Extracted::Data {
                    user_access_token__id: _,
                    user__id: user__id_,
                    user_device__id: user_device__id_,
                    user_access_token__expires_at: _,
                } => (user__id_, user_device__id_),
                Extracted::AlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::InUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            Repository::<Postgresql<UserAccessRefreshToken>>::delete_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await),
                UserAccessRefreshTokenBy2 {
                    user__id,
                    user_device__id,
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
