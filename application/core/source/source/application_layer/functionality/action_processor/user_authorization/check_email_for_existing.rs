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
                User_Email,
            },
            functionality::service::validator::Validator,
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::{
                postgresql::{
                    Postgresql,
                    UserBy2,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::check_email_for_existing::{
            Incoming,
            Outcoming,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct UserAuthorization_CheckEmailForExisting;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_CheckEmailForExisting> {
    type Incoming<'a> = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Void;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_exist = Repository::<Postgresql<User>>::is_exist_2(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy2 {
                    user__email: incoming.user__email.as_str(),
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
