use {
    crate::{
        BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY,
        BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY,
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user::{
                    User,
                    User_Id,
                },
                user_device::UserDevice_Id,
                user_reset_password_token::{
                    UserResetPasswordToken,
                    UserResetPasswordToken_CanBeResentFrom,
                },
            },
            functionality::service::{
                email_sender::EmailSender,
                generator::Generator,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        Postgresql,
                        UserBy3,
                        UserResetPasswordTokenBy,
                        UserResetPasswordTokenUpdate2,
                    },
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    spawner::{
                        Spawner,
                        TokioNonBlockingTask,
                    },
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_reset_password::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        future::Future,
        time::Duration,
    },
};
pub struct UserAuthorization_SendEmailForResetPassword;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForResetPassword> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user__email = match Repository::<Postgresql<User>>::find_6(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await?
            {
                Option::Some(user_) => user_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User__NotFound)),
            };
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_reset_password_token__value,
                user_reset_password_token__is_approved,
                user_reset_password_token__expires_at,
                mut user_reset_password_token__can_be_resent_from,
            ) = match Repository::<Postgresql<UserResetPasswordToken>>::find_3(
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
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if user_reset_password_token__expires_at <= now {
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
            if user_reset_password_token__can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__TimeToResendHasNotCome));
            }
            user_reset_password_token__can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
            if Repository::<Postgresql<UserResetPasswordToken>>::update_2(
                &postgresql_client_database_2,
                UserResetPasswordTokenUpdate2 {
                    user_reset_password_token__can_be_resent_from,
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
            let email_server = &inner.environment_configuration.subject.resource.email_server;
            let user_device__id = incoming.user_device__id.to_string();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match EmailSender::<UserResetPasswordToken>::send(
                            email_server,
                            user_reset_password_token__value.as_str(),
                            user__email.as_str(),
                            user_device__id.as_str(),
                        )
                        .await
                        {
                            Ok(_) => return Result::Ok(()),
                            Err(aggregate_error) => {
                                if quantity == BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                                    return Err(aggregate_error);
                                }
                            }
                        }
                    }
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_reset_password_token__can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
