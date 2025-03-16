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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                        UserAccessRefreshTokenBy1,
                        UserBy3,
                        UserResetPasswordTokenBy,
                        UserUpdate,
                    },
                },
                service::{
                    resolver::{
                        CloudMessage,
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
        action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_last_step::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct UserAuthorization_ResetPasswordByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordByLastStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Password>::is_valid_part_1(incoming.user__password) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            {
                let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
                let (
                    user_reset_password_token__value,
                    mut user_reset_password_token__wrong_enter_tries_quantity,
                    user_reset_password_token__is_approved,
                    user_reset_password_token__expires_at,
                ) = match Repository::<Postgresql<UserResetPasswordToken>>::find_2(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__NotFound)),
                };
                if user_reset_password_token__expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
                    if !Repository::<Postgresql<UserResetPasswordToken>>::delete(
                        &postgresql_database_2_client,
                        UserResetPasswordTokenBy {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await? {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__AlreadyExpired));
                }
                if !user_reset_password_token__is_approved {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__IsNotApproved));
                }
                if user_reset_password_token__value != incoming.user_reset_password_token__value {
                    if user_reset_password_token__wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                        user_reset_password_token__wrong_enter_tries_quantity += 1;
                    }
                    if user_reset_password_token__wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                        if !Repository::<Postgresql<UserResetPasswordToken>>::update_4(
                            &postgresql_database_2_client,
                            UserResetPasswordTokenBy {
                                user__id: incoming.user__id,
                                user_device__id: incoming.user_device__id,
                            },
                        )
                        .await? {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                        }
                    } else {
                        if !Repository::<Postgresql<UserResetPasswordToken>>::delete(
                            &postgresql_database_2_client,
                            UserResetPasswordTokenBy {
                                user__id: incoming.user__id,
                                user_device__id: incoming.user_device__id,
                            },
                        )
                        .await? {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                        }
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken__WrongValue));
                }
            }
            let (user__email, user__nickname, mut user__password_hash) = match Repository::<Postgresql<User>>::find_5(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User__NotFound)),
            };
            if !Validator::<User_Password>::is_valid_part_2(
                incoming.user__password,
                user__email.as_str(),
                user__nickname.as_str(),
            ) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user__password_hash___old = user__password_hash;
            let user__password = incoming.user__password.to_string();
            user__password_hash = crate::result_return_runtime!(
                Spawner::<TokioBlockingTask>::spawn_processed(
                    move || -> _ {
                        return Encoder::<User_Password>::encode(user__password.as_str());
                    },
                )
                .await
            )?;
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let mut postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let mut is_deleted = match Repository::<Postgresql<UserAccessRefreshToken>>::delete_2(
                transaction.get_client(),
                UserAccessRefreshTokenBy1 {
                    user__id: incoming.user__id,
                },
            )
            .await
            {
                Result::Ok(is_deleted_) => is_deleted_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_deleted {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            is_deleted = match Repository::<Postgresql<UserResetPasswordToken>>::delete(
                transaction.get_client(),
                UserResetPasswordTokenBy {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await
            {
                Result::Ok(is_deleted_) => is_deleted_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_deleted {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let is_updated = match Repository::<Postgresql<User>>::update(
                &postgresql_database_1_client,
                UserUpdate {
                    user__password_hash: user__password_hash.as_str(),
                },
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await
            {
                Result::Ok(is_updated_) => is_updated_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_updated {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            if let Result::Err(aggregate_error) = Resolver_::<Transaction<'_>>::commit(transaction).await {
                if !Repository::<Postgresql<User>>::update(
                    &postgresql_database_1_client,
                    UserUpdate {
                        user__password_hash: user__password_hash___old.as_str(),
                    },
                    UserBy3 {
                        user__id: incoming.user__id,
                    },
                )
                .await? {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Err(aggregate_error);
            }
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    Resolver::<CloudMessage>::deauthorize_user_from_all_devices();
                    return Result::Ok(());
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
