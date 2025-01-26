use crate::{
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
        data::{
            aggregate_error::AggregateError,
            capture::Capture,
        },
        functionality::{
            repository::{
                postgresql::{
                    Postgresql,
                    UserBy3,
                    UserResetPasswordTokenBy1,
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
};
use dedicated::{
    action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_reset_password::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_SendEmailForResetPassword;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForResetPassword> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user = match Repository::<Postgresql<User<'_>>>::find_6(
                &crate::result_return_result_runtime!(inner.postgresql_connection_pool_database_1.get().await),
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
            let postgresql_database_2_client = crate::result_return_result_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let mut user_reset_password_token = match Repository::<Postgresql<UserResetPasswordToken<'_>>>::find_3(
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
            let now = Resolver::<UnixTime>::get_now();
            if user_reset_password_token.expires_at <= now {
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
            if user_reset_password_token.can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_TimeToResendHasNotCome));
            }
            user_reset_password_token.can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
            Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_2(
                &postgresql_database_2_client,
                UserResetPasswordTokenUpdate2 {
                    user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                },
                UserResetPasswordTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserResetPasswordToken<'_>>::repeatable_send(
                        &environment_configuration.subject.resource.email_server,
                        user_reset_password_token.value.as_str(),
                        user.email.as_str(),
                        incoming.user_device__id.as_str(),
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
