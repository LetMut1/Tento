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
                    postgresql::{
                        Postgresql,
                        UserAccessRefreshTokenBy1,
                        UserBy3,
                        UserResetPasswordTokenBy1,
                        UserUpdate1,
                        Resolver as Resolver_,
                        Transaction,
                        IsolationLevel,
                    },
                    Repository,
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
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Password>::is_valid_part_1(incoming.user__password.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            {
                let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
                let mut user_reset_password_token = match Repository::<Postgresql<UserResetPasswordToken<'_>>>::find_2(
                    &postgresql_database_2_client,
                    UserResetPasswordTokenBy1 {
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
                if user_reset_password_token.expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
                    Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                        &postgresql_database_2_client,
                        UserResetPasswordTokenBy1 {
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
                    if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                        user_reset_password_token.wrong_enter_tries_quantity += 1;
                    }
                    if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                        Repository::<Postgresql<UserResetPasswordToken<'_>>>::update_4(
                            &postgresql_database_2_client,
                            UserResetPasswordTokenBy1 {
                                user__id: incoming.user__id,
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                            &postgresql_database_2_client,
                            UserResetPasswordTokenBy1 {
                                user__id: incoming.user__id,
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_WrongValue));
                }
            }
            let mut user = match Repository::<Postgresql<User<'_>>>::find_5(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy3 {
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
                return Result::Err(crate::new_invalid_argument!());
            }
            let user__password_hash___old = user.password_hash;
            user.password_hash = crate::result_return_runtime!(
                Spawner::<TokioBlockingTask>::spawn_processed(
                    move || -> _ {
                        return Encoder::<User_Password>::encode(incoming.user__password.as_str());
                    },
                )
                .await
            )?;
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let mut postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            ).await?;
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::delete_2(
                transaction.get_client(),
                UserAccessRefreshTokenBy1 {
                    user__id: incoming.user__id,
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserResetPasswordToken<'_>>>::delete_2(
                transaction.get_client(),
                UserResetPasswordTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<User<'_>>>::update_1(
                &postgresql_database_1_client,
                UserUpdate1 {
                    user__password_hash: user.password_hash.as_str(),
                },
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Resolver_::<Transaction<'_>>::commit(transaction).await {
                Repository::<Postgresql<User<'_>>>::update_1(
                    &postgresql_database_1_client,
                    UserUpdate1 {
                        user__password_hash: user__password_hash___old.as_str(),
                    },
                    UserBy3 {
                        user__id: incoming.user__id,
                    },
                )
                .await?;
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
