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
            user_registration_token::{
                UserRegistrationToken,
                UserRegistrationToken_CanBeResentFrom,
                UserRegistrationToken_ExpiresAt,
                UserRegistrationToken_Value,
                UserRegistrationToken_WrongEnterTriesQuantity,
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
                    UserRegistrationTokenBy1,
                    UserRegistrationTokenUpdate1,
                    UserRegistrationTokenUpdate2,
                    UserRegistrationTokenUpdate3,
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
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_first_step::{
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
pub struct UserAuthorization_RegisterByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByFirstStep> {
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
            if Repository::<Postgresql<User<'_>>>::is_exist_2(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy2 {
                    user__email: incoming.user__email.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_EmailAlreadyExist));
            }
            let now = Resolver::<UnixTime>::get_now();
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_registration_token__value, user_registration_token__can_be_resent_from, user_registration_token__wrong_enter_tries_quantity, can_send) =
                match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_1(
                    &postgresql_database_2_client,
                    UserRegistrationTokenBy1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(mut user_registration_token) => {
                        let (can_send_, need_to_update_1) = if user_registration_token.can_be_resent_from <= now {
                            user_registration_token.can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?;
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
                        let need_to_update_2 = if user_registration_token.expires_at <= now || user_registration_token.is_approved {
                            user_registration_token.value = Generator::<UserRegistrationToken_Value>::generate();
                            user_registration_token.wrong_enter_tries_quantity = 0;
                            user_registration_token.is_approved = false;
                            user_registration_token.expires_at = Generator::<UserRegistrationToken_ExpiresAt>::generate(now)?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            Repository::<Postgresql<UserRegistrationToken<'_>>>::update_1(
                                &postgresql_database_2_client,
                                UserRegistrationTokenUpdate1 {
                                    user_registration_token__value: user_registration_token.value.as_str(),
                                    user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                                    user_registration_token__is_approved: user_registration_token.is_approved,
                                    user_registration_token__expires_at: user_registration_token.expires_at,
                                    user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
                                },
                                UserRegistrationTokenBy1 {
                                    user__email: incoming.user__email.as_str(),
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        } else {
                            if need_to_update_1 {
                                Repository::<Postgresql<UserRegistrationToken<'_>>>::update_2(
                                    &postgresql_database_2_client,
                                    UserRegistrationTokenUpdate2 {
                                        user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
                                    },
                                    UserRegistrationTokenBy1 {
                                        user__email: incoming.user__email.as_str(),
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                            if need_to_update_2 {
                                Repository::<Postgresql<UserRegistrationToken<'_>>>::update_3(
                                    &postgresql_database_2_client,
                                    UserRegistrationTokenUpdate3 {
                                        user_registration_token__value: user_registration_token.value.as_str(),
                                        user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                                        user_registration_token__is_approved: user_registration_token.is_approved,
                                        user_registration_token__expires_at: user_registration_token.expires_at,
                                    },
                                    UserRegistrationTokenBy1 {
                                        user__email: incoming.user__email.as_str(),
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                        }
                        (
                            user_registration_token.value,
                            user_registration_token.can_be_resent_from,
                            user_registration_token.wrong_enter_tries_quantity,
                            can_send_,
                        )
                    }
                    Option::None => {
                        let user_registration_token = UserRegistrationToken::new(
                            Cow::Borrowed(incoming.user__email.as_str()),
                            Cow::Borrowed(incoming.user_device__id.as_str()),
                            Generator::<UserRegistrationToken_Value>::generate(),
                            0,
                            false,
                            Generator::<UserRegistrationToken_ExpiresAt>::generate(now)?,
                            Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?,
                        );
                        Repository::<Postgresql<UserRegistrationToken<'_>>>::create_1(
                            &postgresql_database_2_client,
                            &user_registration_token,
                        )
                        .await?;
                        (
                            user_registration_token.value,
                            user_registration_token.can_be_resent_from,
                            user_registration_token.wrong_enter_tries_quantity,
                            true,
                        )
                    }
                };
            if can_send {
                let environment_configuration = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserRegistrationToken<'_>>::repeatable_send(
                            &environment_configuration.subject.resource.email_server,
                            user_registration_token__value.as_str(),
                            incoming.user__email.as_str(),
                            incoming.user_device__id.as_str(),
                        )
                        .await?;
                        return Result::Ok(());
                    },
                );
            }
            let outcoming = Outcoming {
                verification_message_sent: can_send,
                user_registration_token__can_be_resent_from,
                user_registration_token__wrong_enter_tries_quantity,
                user_registration_token__wrong_enter_tries_quantity_limit: UserRegistrationToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
