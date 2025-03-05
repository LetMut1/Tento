use {
    crate::{
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
                    postgresql::{
                        Postgresql,
                        UserBy3,
                        UserResetPasswordTokenBy,
                        UserResetPasswordTokenUpdate2,
                    },
                    Repository,
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
    std::future::Future,
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
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
                }
            };
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_reset_password_token__value,
                user_reset_password_token__is_approved,
                user_reset_password_token__expires_at,
                mut user_reset_password_token__can_be_resent_from
            ) = match Repository::<Postgresql<UserResetPasswordToken>>::find_3(
                &postgresql_database_2_client,
                UserResetPasswordTokenBy {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_NotFound));
                }
            };
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user_reset_password_token__expires_at <= now {
                Repository::<Postgresql<UserResetPasswordToken>>::delete(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyExpired));
            }
            if user_reset_password_token__is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyApproved));
            }
            if user_reset_password_token__can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_TimeToResendHasNotCome));
            }
            user_reset_password_token__can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
            Repository::<Postgresql<UserResetPasswordToken>>::update_2(
                &postgresql_database_2_client,
                UserResetPasswordTokenUpdate2 {
                    user_reset_password_token__can_be_resent_from,
                },
                UserResetPasswordTokenBy {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?;
            let environment_configuration = inner.environment_configuration;
            let user_device__id = incoming.user_device__id.to_string();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserResetPasswordToken>::repeatable_send(
                        &environment_configuration.subject.resource.email_server,
                        user_reset_password_token__value.as_str(),
                        user__email.as_str(),
                        user_device__id.as_str(),
                    )
                    .await?;
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
