use crate::{
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
                postgresql::{
                    Postgresql,
                    UserResetPasswordTokenBy1,
                    UserResetPasswordTokenUpdate5,
                },
                Repository,
            },
            service::resolver::{
                UnixTime,
                Resolver,
            },
        },
    },
};
use dedicated::{
    action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_second_step::{
        Incoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_ResetPasswordBySecondStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordBySecondStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let mut user_reset_password_token = match Repository::<Postgresql<UserResetPasswordToken<'_>>>::find_2(
                &postgresql_database_2_client,
                UserResetPasswordTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_reset_password_token_) => user_reset_password_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_NotFound));
                }
            };
            if user_reset_password_token.expires_at <= Resolver::<UnixTime>::get_now() {
                Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyExpired));
            }
            if user_reset_password_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyApproved));
            }
            if user_reset_password_token.value != incoming.user_reset_password_token__value {
                if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    user_reset_password_token.wrong_enter_tries_quantity += 1;
                }
                if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_4(
                        &postgresql_database_2_client,
                        UserResetPasswordTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                        &postgresql_database_2_client,
                        UserResetPasswordTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserResetPasswordToken_WrongValue {
                            user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            user_reset_password_token.is_approved = true;
            Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_5(
                &postgresql_database_2_client,
                UserResetPasswordTokenUpdate5 {
                    user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                },
                UserResetPasswordTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
