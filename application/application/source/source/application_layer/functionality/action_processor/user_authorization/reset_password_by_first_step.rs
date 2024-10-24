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
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                user::By2,
                user_reset_password_token::{
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
                    TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_first_step::{
    Incoming,
    Outcoming,
    Precedent,
};
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct UserAuthorization_ResetPasswordByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordByFirstStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
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
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let user = PostgresqlRepository::<User>::find_4(
                &*database_1_postgresql_pooled_connection,
                By2 {
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let (
                user_reset_password_token__value,
                user_reset_password_token__can_be_resent_from,
                user_reset_password_token__wrong_enter_tries_quantity,
                can_send,
            ) = match PostgresqlRepository::<UserResetPasswordToken>::find_1(
                database_2_postgresql_connection,
                By1 {
                    user__id: user_.id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut user_reset_password_token) => {
                    let (can_send_, need_to_update_1) = if Resolver::<Expiration>::is_expired(user_reset_password_token.can_be_resent_from) {
                        user_reset_password_token.can_be_resent_from = Generator::<UserResetPasswordToken_CanBeResentFrom>::generate()?;
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
                    let need_to_update_2 =
                        if Resolver::<Expiration>::is_expired(user_reset_password_token.expires_at) || user_reset_password_token.is_approved {
                            user_reset_password_token.value = Generator::<UserResetPasswordToken_Value>::generate();
                            user_reset_password_token.wrong_enter_tries_quantity = 0;
                            user_reset_password_token.is_approved = false;
                            user_reset_password_token.expires_at = Generator::<UserResetPasswordToken_ExpiresAt>::generate()?;
                            true
                        } else {
                            false
                        };
                    if need_to_update_1 && need_to_update_2 {
                        PostgresqlRepository::<UserResetPasswordToken>::update_1(
                            database_2_postgresql_connection,
                            Update1 {
                                user_reset_password_token__value: user_reset_password_token.value.as_str(),
                                user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                                user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                                user_reset_password_token__expires_at: user_reset_password_token.expires_at,
                                user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                            },
                            By1 {
                                user__id: user_.id,
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        if need_to_update_1 {
                            PostgresqlRepository::<UserResetPasswordToken>::update_2(
                                database_2_postgresql_connection,
                                Update2 {
                                    user_reset_password_token__can_be_resent_from: user_reset_password_token.can_be_resent_from,
                                },
                                By1 {
                                    user__id: user_.id,
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                        if need_to_update_2 {
                            PostgresqlRepository::<UserResetPasswordToken>::update_3(
                                database_2_postgresql_connection,
                                Update3 {
                                    user_reset_password_token__value: user_reset_password_token.value.as_str(),
                                    user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                                    user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                                    user_reset_password_token__expires_at: user_reset_password_token.expires_at,
                                },
                                By1 {
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
                    let user_reset_password_token = PostgresqlRepository::<UserResetPasswordToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        Insert1 {
                            user__id: user_.id,
                            user_device__id: incoming.user_device__id.as_str(),
                            user_reset_password_token__value: Generator::<UserResetPasswordToken_Value>::generate(),
                            user_reset_password_token__wrong_enter_tries_quantity: 0,
                            user_reset_password_token__is_approved: false,
                            user_reset_password_token__expires_at: Generator::<UserResetPasswordToken_ExpiresAt>::generate()?,
                            user_reset_password_token__can_be_resent_from: Generator::<UserResetPasswordToken_CanBeResentFrom>::generate()?,
                        },
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
                let environment_configuration_ = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserResetPasswordToken<'_>>::repeatable_send(
                            environment_configuration_,
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
