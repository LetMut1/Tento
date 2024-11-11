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
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                user::By2,
                user_registration_token::{
                    By1,
                    Insert1,
                    Update1,
                    Update2,
                    Update3,
                },
                PostgresqlRepository,
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
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_first_step::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
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
            if PostgresqlRepository::<User<'_>>::is_exist_2(
                &inner.get_database_1_postgresql_client().await?,
                By2 {
                    user__email: incoming.user__email.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_EmailAlreadyExist));
            }
            let database_2_postgresql_client = inner.get_database_2_postgresql_client().await?;
            let (user_registration_token__value, user_registration_token__can_be_resent_from, user_registration_token__wrong_enter_tries_quantity, can_send) =
                match PostgresqlRepository::<UserRegistrationToken<'_>>::find_1(
                    &database_2_postgresql_client,
                    By1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(mut user_registration_token) => {
                        let (can_send_, need_to_update_1) = if Resolver::<Expiration>::is_expired(user_registration_token.can_be_resent_from) {
                            user_registration_token.can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate()?;
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
                        let need_to_update_2 = if Resolver::<Expiration>::is_expired(user_registration_token.expires_at) || user_registration_token.is_approved {
                            user_registration_token.value = Generator::<UserRegistrationToken_Value>::generate();
                            user_registration_token.wrong_enter_tries_quantity = 0;
                            user_registration_token.is_approved = false;
                            user_registration_token.expires_at = Generator::<UserRegistrationToken_ExpiresAt>::generate()?;
                            true
                        } else {
                            false
                        };
                        if need_to_update_1 && need_to_update_2 {
                            PostgresqlRepository::<UserRegistrationToken<'_>>::update_1(
                                &database_2_postgresql_client,
                                Update1 {
                                    user_registration_token__value: user_registration_token.value.as_str(),
                                    user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                                    user_registration_token__is_approved: user_registration_token.is_approved,
                                    user_registration_token__expires_at: user_registration_token.expires_at,
                                    user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
                                },
                                By1 {
                                    user__email: incoming.user__email.as_str(),
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        } else {
                            if need_to_update_1 {
                                PostgresqlRepository::<UserRegistrationToken<'_>>::update_2(
                                    &database_2_postgresql_client,
                                    Update2 {
                                        user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
                                    },
                                    By1 {
                                        user__email: incoming.user__email.as_str(),
                                        user_device__id: incoming.user_device__id.as_str(),
                                    },
                                )
                                .await?;
                            }
                            if need_to_update_2 {
                                PostgresqlRepository::<UserRegistrationToken<'_>>::update_3(
                                    &database_2_postgresql_client,
                                    Update3 {
                                        user_registration_token__value: user_registration_token.value.as_str(),
                                        user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                                        user_registration_token__is_approved: user_registration_token.is_approved,
                                        user_registration_token__expires_at: user_registration_token.expires_at,
                                    },
                                    By1 {
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
                        let user_registration_token = PostgresqlRepository::<UserRegistrationToken<'_>>::create_1(
                            &database_2_postgresql_client,
                            Insert1 {
                                user__email: incoming.user__email.as_str(),
                                user_device__id: incoming.user_device__id.as_str(),
                                user_registration_token__value: Generator::<UserRegistrationToken_Value>::generate(),
                                user_registration_token__wrong_enter_tries_quantity: 0,
                                user_registration_token__is_approved: false,
                                user_registration_token__expires_at: Generator::<UserRegistrationToken_ExpiresAt>::generate()?,
                                user_registration_token__can_be_resent_from: Generator::<UserRegistrationToken_CanBeResentFrom>::generate()?,
                            },
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
                let environment_configuration_ = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserRegistrationToken<'_>>::repeatable_send(
                            environment_configuration_,
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
