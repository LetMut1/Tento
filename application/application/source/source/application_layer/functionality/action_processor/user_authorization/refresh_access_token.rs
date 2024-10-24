use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
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
        },
        functionality::service::{
            encoder::Encoder,
            generator::Generator,
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
                user_access_refresh_token::{
                    By2,
                    Update1,
                },
                PostgresqlRepository,
            },
            service::resolver::{
                Expiration,
                Resolver,
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::refresh_access_token::{
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
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
pub struct UserAuthorization_RefreshAccessToken;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RefreshAccessToken> {
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
            let user_access_token = Encoder::<UserAccessToken<'_>>::decode(
                inner.environment_configuration,
                &incoming.user_access_token_encoded,
            )?;
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut user_access_refresh_token = match PostgresqlRepository::<UserAccessRefreshToken<'_>>::find_1(
                database_2_postgresql_connection,
                By2 {
                    user__id: user_access_token.user__id,
                    user_device__id: user_access_token.user_device__id,
                },
            )
            .await?
            {
                Option::Some(user_access_refresh_token_) => user_access_refresh_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessRefreshToken_NotFound));
                }
            };
            let is_valid = Encoder::<UserAccessRefreshToken<'_>>::is_valid(
                inner.environment_configuration,
                &user_access_refresh_token,
                &incoming.user_access_refresh_token_encoded,
            )?;
            if !is_valid || user_access_token.id.as_str() != user_access_refresh_token.user_access_token__id.as_ref() {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if Resolver::<Expiration>::is_expired(user_access_refresh_token.expires_at) {
                PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By2 {
                        user__id: user_access_token.user__id,
                        user_device__id: user_access_token.user_device__id,
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessRefreshToken_AlreadyExpired));
            }
            let user_access_token_new = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                user_access_token.user__id,
                user_access_token.user_device__id,
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            user_access_refresh_token.user_access_token__id = Cow::Borrowed(user_access_token_new.id.as_str());
            user_access_refresh_token.obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            user_access_refresh_token.expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?;
            user_access_refresh_token.updated_at = Generator::<UserAccessRefreshToken_UpdatedAt>::generate();
            PostgresqlRepository::<UserAccessRefreshToken>::update_1(
                database_2_postgresql_connection,
                Update1 {
                    user_access_token__id: user_access_refresh_token.user_access_token__id.as_ref(),
                    user_access_refresh_token__obfuscation_value: user_access_refresh_token.obfuscation_value.as_str(),
                    user_access_refresh_token__expires_at: user_access_refresh_token.expires_at,
                    user_access_refresh_token__updated_at: user_access_refresh_token.updated_at,
                },
                By2 {
                    user__id: user_access_token.user__id,
                    user_device__id: user_access_token.user_device__id,
                },
            )
            .await?;
            let outcoming = Outcoming {
                user_access_token_encoded: Encoder::<UserAccessToken<'_>>::encode(
                    inner.environment_configuration,
                    &user_access_token_new,
                )?,
                user_access_refresh_token_encoded: Encoder::<UserAccessRefreshToken<'_>>::encode(
                    inner.environment_configuration,
                    &user_access_refresh_token,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
