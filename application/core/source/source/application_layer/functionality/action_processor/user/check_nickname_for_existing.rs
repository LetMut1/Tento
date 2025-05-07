use {
    crate::{
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
            data::aggregate_error::AggregateError,
            functionality::repository::{
                Repository,
                postgresql::{
                    Postgresql,
                    UserBy1,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::check_nickname_for_existing::{
            Incoming,
            Outcoming,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct User_CheckNicknameForExisting;
impl ActionProcessor_ for ActionProcessor<User_CheckNicknameForExisting> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Void;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_exist = Repository::<Postgresql<User>>::is_exist_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy1 {
                    user__nickname: incoming.user__nickname,
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
