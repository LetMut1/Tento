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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserBy2,
                        UserRegistrationTokenBy,
                        UserRegistrationTokenInsert,
                        UserRegistrationTokenUpdate1,
                        UserRegistrationTokenUpdate2,
                        UserRegistrationTokenUpdate3,
                    }, Repository
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
        }, BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY, BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY,
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::register_by_first_step::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{future::Future, time::Duration},
};
pub struct UserAuthorization_RegisterByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByFirstStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if Repository::<Postgresql<User>>::is_exist_2(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy2 {
                    user__email: incoming.user__email,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User__EmailAlreadyExist));
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_registration_token__value, user_registration_token__can_be_resent_from, user_registration_token__wrong_enter_tries_quantity, can_send) =
                match Repository::<Postgresql<UserRegistrationToken>>::find_1(
                    &postgresql_database_2_client,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some((
                        mut user_registration_token__value_,
                        mut user_registration_token__wrong_enter_tries_quantity_,
                        mut user_registration_token__is_approved,
                        mut user_registration_token__expires_at,
                        mut user_registration_token__can_be_resent_from_,
                    )) => {
                        let (can_send_, need_to_update_1) = if user_registration_token__can_be_resent_from_ <= now {
                            user_registration_token__can_be_resent_from_ = Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?;
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
                        let need_to_update_2 = if user_registration_token__expires_at <= now || user_registration_token__is_approved {
                            user_registration_token__value_ = Generator::<UserRegistrationToken_Value>::generate();
                            user_registration_token__wrong_enter_tries_quantity_ = 0;
                            user_registration_token__is_approved = false;
                            user_registration_token__expires_at = Generator::<UserRegistrationToken_ExpiresAt>::generate(now)?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            if !Repository::<Postgresql<UserRegistrationToken>>::update_1(
                                &postgresql_database_2_client,
                                UserRegistrationTokenUpdate1 {
                                    user_registration_token__value: user_registration_token__value_.as_str(),
                                    user_registration_token__wrong_enter_tries_quantity: user_registration_token__wrong_enter_tries_quantity_,
                                    user_registration_token__is_approved,
                                    user_registration_token__expires_at,
                                    user_registration_token__can_be_resent_from: user_registration_token__can_be_resent_from_,
                                },
                                UserRegistrationTokenBy {
                                    user__email: incoming.user__email,
                                    user_device__id: incoming.user_device__id,
                                },
                            )
                            .await? {
                                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                            }
                        } else {
                            if need_to_update_1 {
                                if !Repository::<Postgresql<UserRegistrationToken>>::update_2(
                                    &postgresql_database_2_client,
                                    UserRegistrationTokenUpdate2 {
                                        user_registration_token__can_be_resent_from: user_registration_token__can_be_resent_from_,
                                    },
                                    UserRegistrationTokenBy {
                                        user__email: incoming.user__email,
                                        user_device__id: incoming.user_device__id,
                                    },
                                )
                                .await? {
                                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                                }
                            }
                            if need_to_update_2 {
                                if !Repository::<Postgresql<UserRegistrationToken>>::update_3(
                                    &postgresql_database_2_client,
                                    UserRegistrationTokenUpdate3 {
                                        user_registration_token__value: user_registration_token__value_.as_str(),
                                        user_registration_token__wrong_enter_tries_quantity: user_registration_token__wrong_enter_tries_quantity_,
                                        user_registration_token__is_approved,
                                        user_registration_token__expires_at,
                                    },
                                    UserRegistrationTokenBy {
                                        user__email: incoming.user__email,
                                        user_device__id: incoming.user_device__id,
                                    },
                                )
                                .await? {
                                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                                }
                            }
                        }
                        (
                            user_registration_token__value_,
                            user_registration_token__can_be_resent_from_,
                            user_registration_token__wrong_enter_tries_quantity_,
                            can_send_,
                        )
                    }
                    Option::None => {
                        let user_registration_token__value_ = Generator::<UserRegistrationToken_Value>::generate();
                        let user_registration_token__wrong_enter_tries_quantity_ = 0;
                        let user_registration_token__can_be_resent_from_ = Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?;
                        if !Repository::<Postgresql<UserRegistrationToken>>::create(
                            &postgresql_database_2_client,
                            UserRegistrationTokenInsert {
                                user__email: incoming.user__email,
                                user_device__id: incoming.user_device__id,
                                user_registration_token__value: user_registration_token__value_.as_str(),
                                user_registration_token__wrong_enter_tries_quantity: user_registration_token__wrong_enter_tries_quantity_,
                                user_registration_token__is_approved: false,
                                user_registration_token__can_be_resent_from: user_registration_token__can_be_resent_from_,
                                user_registration_token__expires_at: Generator::<UserRegistrationToken_ExpiresAt>::generate(now)?,
                            },
                        )
                        .await? {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                        }
                        (
                            user_registration_token__value_,
                            user_registration_token__can_be_resent_from_,
                            user_registration_token__wrong_enter_tries_quantity_,
                            true,
                        )
                    }
                };
            if can_send {
                let email_server = &inner.environment_configuration.subject.resource.email_server;
                let user__email = incoming.user__email.to_string();
                let user_device__id = incoming.user_device__id.to_string();
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                        let mut counter: usize = 0;
                        'a: loop {
                            interval.tick().await;
                            match EmailSender::<UserRegistrationToken>::send(
                                email_server,
                                user_registration_token__value.as_str(),
                                user__email.as_str(),
                                user_device__id.as_str(),
                            ).await {
                                Ok(_) => return Result::Ok(()),
                                Err(aggregate_error) => {
                                    counter += 1;
                                    if counter == BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                                        return Err(aggregate_error)
                                    }
                                    continue 'a;
                                }
                            }
                        }
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
