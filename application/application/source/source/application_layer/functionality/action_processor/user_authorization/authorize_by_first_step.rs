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
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                user::{
                    By1,
                    By2,
                },
                user_authorization_token::{
                    By1 as By1_,
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
                    TokioBlockingTask,
                    TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use forced_crate::action_processor_incoming_outcoming::action_processor::user_authorization::authorize_by_first_step::{
    Incoming,
    Outcoming,
    Precedent,
};
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use forced_crate::unified_report::UnifiedReport;
use forced_crate::void::Void;
pub struct UserAuthorization_AuthorizeByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByFirstStep> {
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
            if !Validator::<User_Password>::is_valid_part_1(incoming.user__password.as_str()) {
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
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            let (user__id, user__email, user__nickname, user__password_hash) =
                if Validator::<User_Email>::is_valid(incoming.user__email___or___user__nickname.as_str())? {
                    let user_ = PostgresqlRepository::<User>::find_3(
                        database_1_postgresql_connection,
                        By2 {
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
                        let user_ = PostgresqlRepository::<User>::find_2(
                            database_1_postgresql_connection,
                            By1 {
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
                        return Result::Err(
                            AggregateError::new_invalid_argument(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };
            if !Validator::<User_Password>::is_valid_part_2(
                incoming.user__password.as_str(),
                user__email.as_str(),
                user__nickname.as_str(),
            ) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let is_valid_join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::is_valid(
                        incoming.user__password.as_str(),
                        user__password_hash.as_str(),
                    );
                },
            );
            if !is_valid_join_handle.await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?? {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_WrongEmailOrNicknameOrPassword));
            }
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let (
                user_authorization_token__value,
                user_authorization_token__can_be_resent_from,
                user_authorization_token__wrong_enter_tries_quantity,
                can_send,
            ) = match PostgresqlRepository::<UserAuthorizationToken>::find_1(
                database_2_postgresql_connection,
                By1_ {
                    user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut user_authorization_token) => {
                    let (can_send_, need_to_update_1) = if Resolver::<Expiration>::is_expired(user_authorization_token.can_be_resent_from) {
                        user_authorization_token.can_be_resent_from = Generator::<UserAuthorizationToken_CanBeResentFrom>::generate()?;
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
                    let need_to_update_2 = if Resolver::<Expiration>::is_expired(user_authorization_token.expires_at) {
                        user_authorization_token.value = Generator::<UserAuthorizationToken_Value>::generate();
                        user_authorization_token.wrong_enter_tries_quantity = 0;
                        user_authorization_token.expires_at = Generator::<UserAuthorizationToken_ExpiresAt>::generate()?;
                        true
                    } else {
                        false
                    };
                    if need_to_update_1 && need_to_update_2 {
                        PostgresqlRepository::<UserAuthorizationToken>::update_1(
                            database_2_postgresql_connection,
                            Update1 {
                                user_authorization_token__value: user_authorization_token.value.as_str(),
                                user_authorization_token__wrong_enter_tries_quantity: user_authorization_token.wrong_enter_tries_quantity,
                                user_authorization_token__expires_at: user_authorization_token.expires_at,
                                user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
                            },
                            By1_ {
                                user__id,
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        if need_to_update_1 {
                            PostgresqlRepository::<UserAuthorizationToken>::update_3(
                                database_2_postgresql_connection,
                                Update3 {
                                    user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
                                },
                                By1_ {
                                    user__id,
                                    user_device__id: incoming.user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                        if need_to_update_2 {
                            PostgresqlRepository::<UserAuthorizationToken>::update_2(
                                database_2_postgresql_connection,
                                Update2 {
                                    user_authorization_token__value: user_authorization_token.value.as_str(),
                                    user_authorization_token__wrong_enter_tries_quantity: user_authorization_token.wrong_enter_tries_quantity,
                                    user_authorization_token__expires_at: user_authorization_token.expires_at,
                                },
                                By1_ {
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
                    let user_authorization_token = PostgresqlRepository::<UserAuthorizationToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        Insert1 {
                            user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                            user_authorization_token__value: Generator::<UserAuthorizationToken_Value>::generate(),
                            user_authorization_token__wrong_enter_tries_quantity: 0,
                            user_authorization_token__expires_at: Generator::<UserAuthorizationToken_ExpiresAt>::generate()?,
                            user_authorization_token__can_be_resent_from: Generator::<UserAuthorizationToken_CanBeResentFrom>::generate()?,
                        },
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
                let environment_configuration_ = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserAuthorizationToken<'_>>::repeatable_send(
                            environment_configuration_,
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
