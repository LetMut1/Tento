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
                    User_Email,
                    User_Nickname,
                    User_Password,
                },
                user_authorization_token::{
                    UserAuthorizationToken,
                    UserAuthorizationToken_CanBeResentFrom,
                    UserAuthorizationToken_ExpiresAt,
                    UserAuthorizationToken_Value,
                    UserAuthorizationToken_WrongEnterTriesQuantity,
                },
                user_device::UserDevice_Id,
            },
            functionality::service::{
                email_sender::EmailSender,
                encoder::Encoder,
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
                        UserAuthorizationTokenBy1,
                        UserAuthorizationTokenUpdate1,
                        UserAuthorizationTokenUpdate2,
                        UserAuthorizationTokenUpdate3,
                        UserBy1,
                        UserBy2,
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
                        TokioBlockingTask,
                        TokioNonBlockingTask,
                    },
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::authorize_by_first_step::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        borrow::Cow,
        future::Future,
    },
};
pub struct UserAuthorization_AuthorizeByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByFirstStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Password>::is_valid_part_1(incoming.user__password.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let (user__id, user__email, user__nickname, user__password_hash) = if Validator::<User_Email>::is_valid(incoming.user__email___or___user__nickname.as_str())? {
                let user_ = Repository::<Postgresql<User>>::find_3(
                    &postgresql_database_1_client,
                    UserBy2 {
                        user__email: incoming.user__email___or___user__nickname.as_str(),
                    },
                )
                .await?;
                let user__ = match user_ {
                    Option::Some(user___) => user___,
                    Option::None => {
                        return Result::Ok(UnifiedReport::precedent(Precedent::User_WrongEmailOrNicknameOrPassword));
                    }
                };
                (
                    user__.id,
                    incoming.user__email___or___user__nickname,
                    user__.nickname,
                    user__.password_hash,
                )
            } else {
                if Validator::<User_Nickname>::is_valid(incoming.user__email___or___user__nickname.as_str()) {
                    let user_ = Repository::<Postgresql<User>>::find_2(
                        &postgresql_database_1_client,
                        UserBy1 {
                            user__nickname: incoming.user__email___or___user__nickname.as_str(),
                        },
                    )
                    .await?;
                    let user__ = match user_ {
                        Option::Some(user___) => user___,
                        Option::None => {
                            return Result::Ok(UnifiedReport::precedent(Precedent::User_WrongEmailOrNicknameOrPassword));
                        }
                    };
                    (
                        user__.id,
                        user__.email,
                        incoming.user__email___or___user__nickname,
                        user__.password_hash,
                    )
                } else {
                    return Result::Err(crate::new_invalid_argument!());
                }
            };
            if !Validator::<User_Password>::is_valid_part_2(
                incoming.user__password.as_str(),
                user__email.as_str(),
                user__nickname.as_str(),
            ) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_valid_join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::is_valid(
                        incoming.user__password.as_str(),
                        user__password_hash.as_str(),
                    );
                },
            );
            if !crate::result_return_runtime!(is_valid_join_handle.await)? {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_WrongEmailOrNicknameOrPassword));
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_authorization_token__value, user_authorization_token__can_be_resent_from, user_authorization_token__wrong_enter_tries_quantity, can_send) =
                match Repository::<Postgresql<UserAuthorizationToken<'_>>>::find_1(
                    &postgresql_database_2_client,
                    UserAuthorizationTokenBy1 {
                        user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(mut user_authorization_token) => {
                        let (can_send_, need_to_update_1) = if user_authorization_token.can_be_resent_from <= now {
                            user_authorization_token.can_be_resent_from = Generator::<UserAuthorizationToken_CanBeResentFrom>::generate(now)?;
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
                        let need_to_update_2 = if user_authorization_token.expires_at <= now {
                            user_authorization_token.value = Generator::<UserAuthorizationToken_Value>::generate();
                            user_authorization_token.wrong_enter_tries_quantity = 0;
                            user_authorization_token.expires_at = Generator::<UserAuthorizationToken_ExpiresAt>::generate(now)?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_1(
                                &postgresql_database_2_client,
                                UserAuthorizationTokenUpdate1 {
                                    user_authorization_token__value: user_authorization_token.value.as_str(),
                                    user_authorization_token__wrong_enter_tries_quantity: user_authorization_token.wrong_enter_tries_quantity,
                                    user_authorization_token__expires_at: user_authorization_token.expires_at,
                                    user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
                                },
                                UserAuthorizationTokenBy1 {
                                    user__id,
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        } else {
                            if need_to_update_1 {
                                Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_3(
                                    &postgresql_database_2_client,
                                    UserAuthorizationTokenUpdate3 {
                                        user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
                                    },
                                    UserAuthorizationTokenBy1 {
                                        user__id,
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                            if need_to_update_2 {
                                Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_2(
                                    &postgresql_database_2_client,
                                    UserAuthorizationTokenUpdate2 {
                                        user_authorization_token__value: user_authorization_token.value.as_str(),
                                        user_authorization_token__wrong_enter_tries_quantity: user_authorization_token.wrong_enter_tries_quantity,
                                        user_authorization_token__expires_at: user_authorization_token.expires_at,
                                    },
                                    UserAuthorizationTokenBy1 {
                                        user__id,
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                        }
                        (
                            user_authorization_token.value,
                            user_authorization_token.can_be_resent_from,
                            user_authorization_token.wrong_enter_tries_quantity,
                            can_send_,
                        )
                    }
                    Option::None => {
                        let user_authorization_token = UserAuthorizationToken::new(
                            user__id,
                            Cow::Borrowed(incoming.user_device__id.as_str()),
                            Generator::<UserAuthorizationToken_Value>::generate(),
                            0,
                            Generator::<UserAuthorizationToken_ExpiresAt>::generate(now)?,
                            Generator::<UserAuthorizationToken_CanBeResentFrom>::generate(now)?,
                        );
                        Repository::<Postgresql<UserAuthorizationToken<'_>>>::create_1(
                            &postgresql_database_2_client,
                            &user_authorization_token,
                        )
                        .await?;
                        (
                            user_authorization_token.value,
                            user_authorization_token.can_be_resent_from,
                            user_authorization_token.wrong_enter_tries_quantity,
                            true,
                        )
                    }
                };
            if can_send {
                let environment_configuration = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserAuthorizationToken<'_>>::repeatable_send(
                            &environment_configuration.subject.resource.email_server,
                            user_authorization_token__value.as_str(),
                            user__email.as_str(),
                            incoming.user_device__id.as_str(),
                        )
                        .await?;
                        return Result::Ok(());
                    },
                );
            }
            let outcoming = Outcoming {
                user__id,
                verification_message_sent: can_send,
                user_authorization_token__can_be_resent_from,
                user_authorization_token__wrong_enter_tries_quantity,
                user_authorization_token__wrong_enter_tries_quantity_limit: UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
