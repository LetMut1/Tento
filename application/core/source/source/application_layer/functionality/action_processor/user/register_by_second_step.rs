use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user::User_Email,
                user_device::UserDevice_Id,
                user_registration_token::{
                    UserRegistrationToken,
                    UserRegistrationToken_Value,
                    UserRegistrationToken_WrongEnterTriesQuantity,
                },
            },
            functionality::service::validator::Validator,
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        Postgresql,
                        UserRegistrationTokenBy,
                        UserRegistrationTokenUpdate5,
                    },
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::register_by_second_step::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct RegisterBySecondStep;
impl ActionProcessor_ for ActionProcessor<RegisterBySecondStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_registration_token__value,
                mut user_registration_token__wrong_enter_tries_quantity,
                mut user_registration_token__is_approved,
                user_registration_token__expires_at,
            ) = match Repository::<Postgresql<UserRegistrationToken>>::find_2(
                &postgresql_client_database_2,
                UserRegistrationTokenBy {
                    user__email: incoming.user__email,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__NotFound)),
            };
            if user_registration_token__expires_at <= Resolver::<UnixTime>::get_now_in_microseconds() {
                if !Repository::<Postgresql<UserRegistrationToken>>::delete(
                    &postgresql_client_database_2,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__AlreadyExpired));
            }
            if user_registration_token__is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__AlreadyApproved));
            }
            if user_registration_token__value != incoming.user_registration_token__value {
                if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    user_registration_token__wrong_enter_tries_quantity += 1;
                }
                if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    if !Repository::<Postgresql<UserRegistrationToken>>::update_4(
                        &postgresql_client_database_2,
                        UserRegistrationTokenBy {
                            user__email: incoming.user__email,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                } else {
                    if !Repository::<Postgresql<UserRegistrationToken>>::delete(
                        &postgresql_client_database_2,
                        UserRegistrationTokenBy {
                            user__email: incoming.user__email,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserRegistrationToken__WrongValue {
                            user_registration_token__wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            user_registration_token__is_approved = true;
            if !Repository::<Postgresql<UserRegistrationToken>>::update_5(
                &postgresql_client_database_2,
                UserRegistrationTokenUpdate5 {
                    user_registration_token__is_approved,
                },
                UserRegistrationTokenBy {
                    user__email: incoming.user__email,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
