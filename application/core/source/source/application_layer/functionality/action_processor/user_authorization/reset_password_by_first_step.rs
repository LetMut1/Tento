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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        Postgresql,
                        UserBy2,
                        UserResetPasswordTokenBy,
                        UserResetPasswordTokenInsert,
                        UserResetPasswordTokenUpdate1,
                        UserResetPasswordTokenUpdate2,
                        UserResetPasswordTokenUpdate3,
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
        action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_first_step::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct UserAuthorization_ResetPasswordByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordByFirstStep> {
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
            let user__id = match Repository::<Postgresql<User>>::find_4(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy2 {
                    user__email: incoming.user__email,
                },
            )
            .await?
            {
                Option::Some(user__id_) => user__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound)),
            };
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_reset_password_token__value, user_reset_password_token__can_be_resent_from, user_reset_password_token__wrong_enter_tries_quantity, can_send) =
                match Repository::<Postgresql<UserResetPasswordToken>>::find_1(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy {
                        user__id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some((
                        mut user_reset_password_token__value_,
                        mut user_reset_password_token__wrong_enter_tries_quantity_,
                        mut user_reset_password_token__is_approved,
                        mut user_reset_password_token__expires_at,
                        mut user_reset_password_token__can_be_resent_from_,
                    )) => {
                        let (can_send_, need_to_update_1) = if user_reset_password_token__can_be_resent_from_ <= now {
                            user_reset_password_token__can_be_resent_from_ = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
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
                        let need_to_update_2 = if user_reset_password_token__expires_at <= now || user_reset_password_token__is_approved {
                            user_reset_password_token__value_ = Generator::<UserResetPasswordToken_Value>::generate();
                            user_reset_password_token__wrong_enter_tries_quantity_ = 0;
                            user_reset_password_token__is_approved = false;
                            user_reset_password_token__expires_at = Generator::<UserResetPasswordToken_ExpiresAt>::generate(now)?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            Repository::<Postgresql<UserResetPasswordToken>>::update_1(
                                &postgresql_database_2_client,
                                UserResetPasswordTokenUpdate1 {
                                    user_reset_password_token__value: user_reset_password_token__value_.as_str(),
                                    user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token__wrong_enter_tries_quantity_,
                                    user_reset_password_token__is_approved,
                                    user_reset_password_token__expires_at,
                                    user_reset_password_token__can_be_resent_from: user_reset_password_token__can_be_resent_from_,
                                },
                                UserResetPasswordTokenBy {
                                    user__id,
                                    user_device__id: incoming.user_device__id,
                                },
                            )
                            .await?;
                        } else {
                            if need_to_update_1 {
                                Repository::<Postgresql<UserResetPasswordToken>>::update_2(
                                    &postgresql_database_2_client,
                                    UserResetPasswordTokenUpdate2 {
                                        user_reset_password_token__can_be_resent_from: user_reset_password_token__can_be_resent_from_,
                                    },
                                    UserResetPasswordTokenBy {
                                        user__id,
                                        user_device__id: incoming.user_device__id,
                                    },
                                )
                                .await?;
                            }
                            if need_to_update_2 {
                                Repository::<Postgresql<UserResetPasswordToken>>::update_3(
                                    &postgresql_database_2_client,
                                    UserResetPasswordTokenUpdate3 {
                                        user_reset_password_token__value: user_reset_password_token__value_.as_str(),
                                        user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token__wrong_enter_tries_quantity_,
                                        user_reset_password_token__is_approved,
                                        user_reset_password_token__expires_at,
                                    },
                                    UserResetPasswordTokenBy {
                                        user__id,
                                        user_device__id: incoming.user_device__id,
                                    },
                                )
                                .await?;
                            }
                        }
                        (
                            user_reset_password_token__value_,
                            user_reset_password_token__can_be_resent_from_,
                            user_reset_password_token__wrong_enter_tries_quantity_,
                            can_send_,
                        )
                    }
                    Option::None => {
                        let user_reset_password_token__value_ = Generator::<UserResetPasswordToken_Value>::generate();
                        let user_reset_password_token__wrong_enter_tries_quantity_ = 0;
                        let user_reset_password_token__can_be_resent_from_ = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate(now)?;
                        Repository::<Postgresql<UserResetPasswordToken>>::create(
                            &postgresql_database_2_client,
                            UserResetPasswordTokenInsert {
                                user__id,
                                user_device__id: incoming.user_device__id,
                                user_reset_password_token__value: user_reset_password_token__value_.as_str(),
                                user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token__wrong_enter_tries_quantity_,
                                user_reset_password_token__is_approved: false,
                                user_reset_password_token__can_be_resent_from: user_reset_password_token__can_be_resent_from_,
                                user_reset_password_token__expires_at: Generator::<UserResetPasswordToken_ExpiresAt>::generate(now)?,
                            },
                        )
                        .await?;
                        (
                            user_reset_password_token__value_,
                            user_reset_password_token__can_be_resent_from_,
                            user_reset_password_token__wrong_enter_tries_quantity_,
                            true,
                        )
                    }
                };
            if can_send {
                let environment_configuration = inner.environment_configuration;
                let user__email = incoming.user__email.to_string();
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
            }
            let outcoming = Outcoming {
                user__id,
                verification_message_sent: can_send,
                user_reset_password_token__can_be_resent_from,
                user_reset_password_token__wrong_enter_tries_quantity,
                user_reset_password_token__wrong_enter_tries_quantity_limit: UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
