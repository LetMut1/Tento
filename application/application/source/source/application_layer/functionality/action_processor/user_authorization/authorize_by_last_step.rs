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
            user_authorization_token::{
                UserAuthorizationToken,
                UserAuthorizationToken_Value,
                UserAuthorizationToken_WrongEnterTriesQuantity,
            },
            user_device::{
                UserDevice,
                UserDevice_Id,
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
                UserBy3,
                UserAccessRefreshTokenBy2,
                UserAccessRefreshTokenInsert1,
                UserAccessRefreshTokenUpdate1,
                UserAuthorizationTokenBy1,
                UserAuthorizationTokenUpdate4,
                UserDeviceInsert1,
                Postgresql,
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
use crate::infrastructure_layer::functionality::repository::Repository;
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::authorize_by_last_step::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::{
    borrow::Cow,
    future::Future,
};
pub struct UserAuthorization_AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
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
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.user_authorization_token__value.as_str())? {
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
            let database_2_postgresql_client = inner.database_2_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let user_authorization_token = Repository::<Postgresql<UserAuthorizationToken<'_>>>::find_2(
                &database_2_postgresql_client,
                UserAuthorizationTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let mut user_authorization_token_ = match user_authorization_token {
                Option::Some(user_authorization_token__) => user_authorization_token__,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(user_authorization_token_.expires_at) {
                Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                    &database_2_postgresql_client,
                    UserAuthorizationTokenBy1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_AlreadyExpired));
            }
            if user_authorization_token_.value != incoming.user_authorization_token__value {
                user_authorization_token_.wrong_enter_tries_quantity = user_authorization_token_.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                if user_authorization_token_.wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_4(
                        &database_2_postgresql_client,
                        UserAuthorizationTokenUpdate4 {
                            user_authorization_token__wrong_enter_tries_quantity: user_authorization_token_.wrong_enter_tries_quantity,
                        },
                        UserAuthorizationTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                        &database_2_postgresql_client,
                        UserAuthorizationTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserAuthorizationToken_WrongValue {
                            user_authorization_token__wrong_enter_tries_quantity: user_authorization_token_.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            let database_1_postgresql_client = inner.database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            if !Repository::<Postgresql<User<'_>>>::is_exist_3(
                &database_1_postgresql_client,
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
            }
            let user_access_token = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                incoming.user__id,
                incoming.user_device__id.as_str(),
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            let user_access_token__id = user_access_token.id.as_str();
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?;
            let user_access_refresh_token__updated_at = Generator::<UserAccessRefreshToken_UpdatedAt>::generate();
            // TODO  TRANZACTION
            let user_access_refresh_token = match Repository::<Postgresql<UserAccessRefreshToken<'_>>>::find_1(
                &database_2_postgresql_client,
                UserAccessRefreshTokenBy2 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut user_access_refresh_token_) => {
                    user_access_refresh_token_.user_access_token__id = Cow::Borrowed(user_access_token__id);
                    user_access_refresh_token_.obfuscation_value = user_access_refresh_token__obfuscation_value;
                    user_access_refresh_token_.expires_at = user_access_refresh_token__expires_at;
                    user_access_refresh_token_.updated_at = user_access_refresh_token__updated_at;
                    Repository::<Postgresql<UserAccessRefreshToken<'_>>>::update_1(
                        &database_2_postgresql_client,
                        UserAccessRefreshTokenUpdate1 {
                            user_access_token__id: user_access_refresh_token_.user_access_token__id.as_ref(),
                            user_access_refresh_token__obfuscation_value: user_access_refresh_token_.obfuscation_value.as_str(),
                            user_access_refresh_token__expires_at: user_access_refresh_token_.expires_at,
                            user_access_refresh_token__updated_at: user_access_refresh_token_.updated_at,
                        },
                        UserAccessRefreshTokenBy2 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                    user_access_refresh_token_
                }
                Option::None => {
                    let user_access_refresh_token_ = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::create_1(
                        &database_2_postgresql_client,
                        UserAccessRefreshTokenInsert1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                            user_access_token__id,
                            user_access_refresh_token__obfuscation_value,
                            user_access_refresh_token__expires_at,
                            user_access_refresh_token__updated_at,
                        },
                    )
                    .await?;
                    user_access_refresh_token_
                }
            };
            // TODO  TRANZACTION
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
                            user__id: incoming.user__id,
                        },
                    )
                    .await?;
                    Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                        &database_2_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        UserAuthorizationTokenBy1 {
                            user__id: user_device.user__id,
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
