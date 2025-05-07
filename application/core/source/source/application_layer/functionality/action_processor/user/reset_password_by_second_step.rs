use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user::User_Id,
                user_device::UserDevice_Id,
                user_reset_password_token::{
                    UserResetPasswordToken,
                    UserResetPasswordToken_Value,
                    UserResetPasswordToken_WrongEnterTriesQuantity,
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
                        UserResetPasswordTokenBy,
                        UserResetPasswordTokenUpdate5,
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
        action_processor_incoming_outcoming::action_processor::user::reset_password_by_second_step::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct ResetPasswordBySecondStep;
impl ActionProcessor_ for ActionProcessor<ResetPasswordBySecondStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_reset_password_token__value,
                mut user_reset_password_token__wrong_enter_tries_quantity,
                user_reset_password_token__is_approved,
                user_reset_password_token__expires_at,
            ) = match Repository::<Postgresql<UserResetPasswordToken>>::find_2(
                &postgresql_client_database_2,
                UserResetPasswordTokenBy {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__NotFound)),
            };
            if user_reset_password_token__expires_at <= Resolver::<UnixTime>::get_now_in_microseconds() {
                if !Repository::<Postgresql<UserResetPasswordToken>>::delete(
                    &postgresql_client_database_2,
                    UserResetPasswordTokenBy {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__AlreadyExpired));
            }
            if user_reset_password_token__is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__AlreadyApproved));
            }
            if user_reset_password_token__value != incoming.user_reset_password_token__value {
                if user_reset_password_token__wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    user_reset_password_token__wrong_enter_tries_quantity += 1;
                }
                if user_reset_password_token__wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    if !Repository::<Postgresql<UserResetPasswordToken>>::update_4(
                        &postgresql_client_database_2,
                        UserResetPasswordTokenBy {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                } else {
                    if !Repository::<Postgresql<UserResetPasswordToken>>::delete(
                        &postgresql_client_database_2,
                        UserResetPasswordTokenBy {
                            user__id: incoming.user__id,
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
                        Precedent::UserResetPasswordToken__WrongValue {
                            user_reset_password_token__wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            if !Repository::<Postgresql<UserResetPasswordToken>>::update_5(
                &postgresql_client_database_2,
                UserResetPasswordTokenUpdate5 {
                    user_reset_password_token__is_approved: true,
                },
                UserResetPasswordTokenBy {
                    user__id: incoming.user__id,
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
