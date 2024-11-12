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
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                user::By3,
                user_reset_password_token::{
                    By1,
                    Update2,
                },
                Postgresql,
            },
            service::{
                resolver::{
                    Expiration,
                    Resolver,
                },
                spawner::{
                    Spawner,
                    TokioNonBlockingTask,
                },
            },
        },
    },
};
use crate::infrastructure_layer::functionality::repository::Repository;
use dedicated_crate::{
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
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let user = match Repository::<Postgresql<User<'_>>>::find_6(
                &inner.database_1_postgresql_connection_pool.get().await.into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?,
                By3 {
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
            let database_2_postgresql_client = inner.database_2_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let mut user_reset_password_token = match Repository::<Postgresql<UserResetPasswordToken<'_>>>::find_3(
                &database_2_postgresql_client,
                By1 {
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
            if Resolver::<Expiration>::is_expired(user_reset_password_token.expires_at) {
                Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                    &database_2_postgresql_client,
                    By1 {
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
            if !Resolver::<Expiration>::is_expired(user_reset_password_token.can_be_resent_from) {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_TimeToResendHasNotCome));
            }
            user_reset_password_token.can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate()?;
            Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_2(
                &database_2_postgresql_client,
                Update2 {
                    user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                },
                By1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration_ = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserResetPasswordToken<'_>>::repeatable_send(
                        environment_configuration_,
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
