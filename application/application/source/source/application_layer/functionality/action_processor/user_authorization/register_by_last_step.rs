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
            user_access_refresh_token::{
                UserAccessRefreshToken,
                UserAccessRefreshToken_ExpiresAt,
                UserAccessRefreshToken_ObfuscationValue,
                UserAccessRefreshToken_UpdatedAt,
            },
            user_access_token::{
                UserAccessToken,
                UserAccessToken_ExpiresAt,
                UserAccessToken_Id,
            },
            user_device::{
                UserDevice,
                UserDevice_Id,
            },
            user_registration_token::{
                UserRegistrationToken,
                UserRegistrationToken_Value,
                UserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            encoder::Encoder,
            generator::Generator,
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
                UserBy1,
                UserBy2,
                UserInsert1,
                UserAccessRefreshTokenInsert1,
                UserDeviceInsert1,
                UserRegistrationTokenBy1,
                UserRegistrationTokenUpdate4,
                Postgresql,
            },
            service::{
                resolver::{
                    Expiration,
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
};
use crate::infrastructure_layer::functionality::repository::Repository;
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_last_step::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_RegisterByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Password>::is_valid(
                incoming.user__password.as_str(),
                incoming.user__email.as_str(),
                incoming.user__nickname.as_str(),
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
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
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
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value.as_str())? {
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
            let database_1_postgresql_client = inner.database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            if Repository::<Postgresql<User<'_>>>::is_exist_1(
                &database_1_postgresql_client,
                UserBy1 {
                    user__nickname: incoming.user__nickname.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_NicknameAlreadyExist));
            }
            if Repository::<Postgresql<User<'_>>>::is_exist_2(
                &database_1_postgresql_client,
                UserBy2 {
                    user__email: incoming.user__email.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_EmailAlreadyExist));
            }
            let database_2_postgresql_client = inner.database_2_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let mut user_registration_token = match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_2(
                &database_2_postgresql_client,
                UserRegistrationTokenBy1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_registration_token_) => user_registration_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(user_registration_token.expires_at) {
                Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                    &database_2_postgresql_client,
                    UserRegistrationTokenBy1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyExpired));
            }
            if !user_registration_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_IsNotApproved));
            }
            if user_registration_token.value != incoming.user_registration_token__value {
                user_registration_token.wrong_enter_tries_quantity = user_registration_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                if user_registration_token.wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::update_4(
                        &database_2_postgresql_client,
                        UserRegistrationTokenUpdate4 {
                            user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                        },
                        UserRegistrationTokenBy1 {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                        &database_2_postgresql_client,
                        UserRegistrationTokenBy1 {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_WrongValue));
            }
            let user__password_hash___join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::encode(incoming.user__password.as_str());
                },
            );
            let user = Repository::<Postgresql<User<'_>>>::create_1(
                &database_1_postgresql_client,
                UserInsert1 {
                    user__email: incoming.user__email,
                    user__nickname: incoming.user__nickname,
                    user__password_hash: user__password_hash___join_handle.await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )??,
                    user__created_at: Resolver::<UnixTime>::get_now(),
                },
            )
            .await?;
            let user_access_token = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                user.id,
                incoming.user_device__id.as_str(),
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            // TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
            let user_access_refresh_token = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::create_1(
                &database_2_postgresql_client,
                UserAccessRefreshTokenInsert1 {
                    user__id: user.id,
                    user_device__id: incoming.user_device__id.as_str(),
                    user_access_token__id: user_access_token.id.as_str(),
                    user_access_refresh_token__obfuscation_value: Generator::<UserAccessRefreshToken_ObfuscationValue>::generate(),
                    user_access_refresh_token__expires_at: Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?,
                    user_access_refresh_token__updated_at: Generator::<UserAccessRefreshToken_UpdatedAt>::generate(),
                },
            )
            .await?;
            let user_access_token_encoded = Encoder::<UserAccessToken<'_>>::encode(
                inner.environment_configuration,
                &user_access_token,
            )?;
            let user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken<'_>>::encode(
                inner.environment_configuration,
                &user_access_refresh_token,
            )?;
            let database_1_postgresql_connection_pool = inner.database_1_postgresql_connection_pool.clone();
            let database_2_postgresql_connection_pool = inner.database_2_postgresql_connection_pool.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let user_device = Repository::<Postgresql<UserDevice>>::create_1(
                        &database_1_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        UserDeviceInsert1 {
                            user_device__id: incoming.user_device__id,
                            user__id: user.id,
                        },
                    )
                    .await?;
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                        &database_2_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        UserRegistrationTokenBy1 {
                            user__email: user.email.as_str(),
                            user_device__id: user_device.id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_access_token_encoded,
                user_access_refresh_token_encoded,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
