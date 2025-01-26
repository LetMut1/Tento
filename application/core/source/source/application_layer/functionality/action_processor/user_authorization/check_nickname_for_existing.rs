use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::user::{
            User,
            User_Nickname,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            capture::Capture,
        },
        functionality::repository::{
            postgresql::{
                Postgresql,
                UserBy1,
            },
            Repository,
        },
    },
};
use dedicated::{
    action_processor_incoming_outcoming::action_processor::user_authorization::check_nickname_for_existing::{
        Incoming,
        Outcoming,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_CheckNicknameForExisting;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_CheckNicknameForExisting> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Void;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_exist = Repository::<Postgresql<User<'_>>>::is_exist_1(
                &crate::result_return_result_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy1 {
                    user__nickname: incoming.user__nickname.as_str(),
                },
            )
            .await?;
            let outcoming = Outcoming {
                result: is_exist,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
