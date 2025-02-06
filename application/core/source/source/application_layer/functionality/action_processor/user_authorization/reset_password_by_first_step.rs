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
                User_Email,
            },
            user_device::UserDevice_Id,
            user_reset_password_token::{
                UserResetPasswordToken,
                UserResetPasswordToken_CanBeResentFrom,
                UserResetPasswordToken_ExpiresAt,
                UserResetPasswordToken_Value,
                UserResetPasswordToken_WrongEnterTriesQuantity,
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
                    UserBy2,
                    UserResetPasswordTokenBy1,
                    UserResetPasswordTokenUpdate1,
                    UserResetPasswordTokenUpdate2,
                    UserResetPasswordTokenUpdate3,
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
    action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_first_step::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::{
    borrow::Cow,
    future::Future,
};
pub struct UserAuthorization_ResetPasswordByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordByFirstStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user = Repository::<Postgresql<User<'_>>>::find_4(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy2 {
                    user__email: incoming.user__email.as_str(),
                },
            )
            .await?;
            let user_ = match user {
                Option::Some(user__) => user__,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
                }
            };
            let now = Resolver::<UnixTime>::get_now();
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_reset_password_token__value, user_reset_password_token__can_be_resent_from, user_reset_password_token__wrong_enter_tries_quantity, can_send) =
                match Repository::<Postgresql<UserResetPasswordToken<'_>>>::find_1(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy1 {
                        user__id: user_.id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(mut user_reset_password_token) => {
                        let (can_send_, need_to_update_1) = if user_reset_password_token.can_be_resent_from <= now {
                            user_reset_password_token.can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
                            (
                                true,
                                true,
                            )
                        } else {
                            (
                                false,
                                false,
                            )
                        };
                        let need_to_update_2 = if user_reset_password_token.expires_at <= now || user_reset_password_token.is_approved {
                            user_reset_password_token.value = Generator::<UserResetPasswordToken_Value>::generate();
                            user_reset_password_token.wrong_enter_tries_quantity = 0;
                            user_reset_password_token.is_approved = false;
                            user_reset_password_token.expires_at = Generator::<UserResetPasswordToken_ExpiresAt>::generate(now)?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_1(
                                &postgresql_database_2_client,
                                UserResetPasswordTokenUpdate1 {
                                    user_reset_password_token__value: user_reset_password_token.value.as_str(),
                                    user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                                    user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                                    user_reset_password_token__expires_at: user_reset_password_token.expires_at,
                                    user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                                },
                                UserResetPasswordTokenBy1 {
                                    user__id: user_.id,
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        } else {
                            if need_to_update_1 {
                                Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_2(
                                    &postgresql_database_2_client,
                                    UserResetPasswordTokenUpdate2 {
                                        user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                                    },
                                    UserResetPasswordTokenBy1 {
                                        user__id: user_.id,
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                            if need_to_update_2 {
                                Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_3(
                                    &postgresql_database_2_client,
                                    UserResetPasswordTokenUpdate3 {
                                        user_reset_password_token__value: user_reset_password_token.value.as_str(),
                                        user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                                        user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                                        user_reset_password_token__expires_at: user_reset_password_token.expires_at,
                                    },
                                    UserResetPasswordTokenBy1 {
                                        user__id: user_.id,
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                        }
                        (
                            user_reset_password_token.value,
                            user_reset_password_token.can_be_resent_from,
                            user_reset_password_token.wrong_enter_tries_quantity,
                            can_send_,
                        )
                    }
                    Option::None => {
                        let user_reset_password_token = UserResetPasswordToken::new(
                            user_.id,
                            Cow::Borrowed(incoming.user_device__id.as_str()),
                            Generator::<UserResetPasswordToken_Value>::generate(),
                            0,
                            false,
                            Generator::<UserResetPasswordToken_ExpiresAt>::generate(now)?,
                            Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?,
                        );
                        Repository::<Postgresql<UserResetPasswordToken<'_>>>::create_1(
                            &postgresql_database_2_client,
                            &user_reset_password_token,
                        )
                        .await?;
                        (
                            user_reset_password_token.value,
                            user_reset_password_token.can_be_resent_from,
                            user_reset_password_token.wrong_enter_tries_quantity,
                            true,
                        )
                    }
                };
            if can_send {
                let environment_configuration = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserResetPasswordToken<'_>>::repeatable_send(
                            &environment_configuration.subject.resource.email_server,
                            user_reset_password_token__value.as_str(),
                            incoming.user__email.as_str(),
                            incoming.user_device__id.as_str(),
                        )
                        .await?;
                        return Result::Ok(());
                    },
                );
            }
            let outcoming = Outcoming {
                user__id: user_.id,
                verification_message_sent: can_send,
                user_reset_password_token__can_be_resent_from,
                user_reset_password_token__wrong_enter_tries_quantity,
                user_reset_password_token__wrong_enter_tries_quantity_limit: UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
