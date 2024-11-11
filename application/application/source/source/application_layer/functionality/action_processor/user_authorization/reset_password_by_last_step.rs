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
                User_Password,
            },
            user_access_refresh_token::UserAccessRefreshToken,
            user_device::UserDevice_Id,
            user_reset_password_token::{
                UserResetPasswordToken,
                UserResetPasswordToken_Value,
                UserResetPasswordToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            encoder::Encoder,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                OptionConverter,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                user::{
                    By3,
                    Update1,
                },
                user_access_refresh_token::By1,
                user_reset_password_token::{
                    By1 as By1_,
                    Update4,
                },
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    CloudMessage,
                    Expiration,
                    Resolver,
                },
                spawner::{
                    Spawner,
                    TokioBlockingTask,
                    TokioNonBlockingTask,
                },
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_last_step::{
        Incoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_ResetPasswordByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
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
            let database_2_postgresql_connection = inner.get_database_2_postgresql_client().await?;
            let mut user_reset_password_token = match PostgresqlRepository::<UserResetPasswordToken<'_>>::find_2(
                &database_2_postgresql_connection,
                By1_ {
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
                PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                    &database_2_postgresql_connection,
                    By1_ {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyExpired));
            }
            if !user_reset_password_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_IsNotApproved));
            }
            if user_reset_password_token.value != incoming.user_reset_password_token__value {
                user_reset_password_token.wrong_enter_tries_quantity = user_reset_password_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::update_4(
                        &database_2_postgresql_connection,
                        Update4 {
                            user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                        },
                        By1_ {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                        &database_2_postgresql_connection,
                        By1_ {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_WrongValue));
            }
            let database_1_postgresql_connection = inner.get_database_1_postgresql_client().await?;
            let mut user = match PostgresqlRepository::<User<'_>>::find_5(
                &database_1_postgresql_connection,
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
            if !Validator::<User_Password>::is_valid_part_2(
                incoming.user__password.as_str(),
                user.email.as_str(),
                user.nickname.as_str(),
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
            let user__password_hash___join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::encode(incoming.user__password.as_str());
                },
            );
            user.password_hash = user__password_hash___join_handle.await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )??;
            PostgresqlRepository::<User<'_>>::update_1(
                &database_1_postgresql_connection,
                Update1 {
                    user__password_hash: user.password_hash.as_str(),
                },
                By3 {
                    user__id: incoming.user__id,
                },
            )
            .await?;
            PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_2(
                &database_2_postgresql_connection,
                By1 {
                    user__id: incoming.user__id,
                },
            )
            .await?;
            Resolver::<CloudMessage>::deauthorize_user_from_all_devices();
            let database_2_postgresql_connection_pool = inner.database_2_postgresql_connection_pool.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                        &database_2_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        By1_ {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
