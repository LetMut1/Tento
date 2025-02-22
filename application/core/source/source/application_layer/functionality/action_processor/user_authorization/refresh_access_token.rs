use {
    crate::{
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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserAccessRefreshTokenBy2,
                        UserAccessRefreshTokenUpdate1,
                    },
                    Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::refresh_access_token::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        borrow::Cow,
        future::Future,
    },
};
pub struct UserAuthorization_RefreshAccessToken;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RefreshAccessToken> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user_access_token = Encoder::<UserAccessToken<'_>>::decode(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_encoded,
            )?;
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let mut user_access_refresh_token = match Repository::<Postgresql<UserAccessRefreshToken<'_>>>::find_1(
                &postgresql_database_2_client,
                UserAccessRefreshTokenBy2 {
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
                &inner.environment_configuration.subject.encryption.private_key,
                user_access_refresh_token.user__id,
                user_access_refresh_token.user_device__id,
                user_access_refresh_token.user_access_token__id.as_ref(),
                user_access_refresh_token.obfuscation_value.as_str(),
                user_access_refresh_token.expires_at,
                user_access_refresh_token.updated_at,
                &incoming.user_access_refresh_token_encoded,
            )?;
            if !is_valid || user_access_token.id.as_str() != user_access_refresh_token.user_access_token__id.as_ref() {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user_access_refresh_token.expires_at <= now {
                Repository::<Postgresql<UserAccessRefreshToken<'_>>>::delete_1(
                    &postgresql_database_2_client,
                    UserAccessRefreshTokenBy2 {
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
                Generator::<UserAccessToken_ExpiresAt>::generate(now)?,
            );
            user_access_refresh_token.user_access_token__id = Cow::Borrowed(user_access_token_new.id.as_str());
            user_access_refresh_token.obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            user_access_refresh_token.expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            user_access_refresh_token.updated_at = now;
            Repository::<Postgresql<UserAccessRefreshToken<'_>>>::update_1(
                &postgresql_database_2_client,
                UserAccessRefreshTokenUpdate1 {
                    user_access_token__id: user_access_refresh_token.user_access_token__id.as_ref(),
                    user_access_refresh_token__obfuscation_value: user_access_refresh_token.obfuscation_value.as_str(),
                    user_access_refresh_token__expires_at: user_access_refresh_token.expires_at,
                    user_access_refresh_token__updated_at: user_access_refresh_token.updated_at,
                },
                UserAccessRefreshTokenBy2 {
                    user__id: user_access_token.user__id,
                    user_device__id: user_access_token.user_device__id,
                },
            )
            .await?;
            let outcoming = Outcoming {
                user_access_token_encoded: Encoder::<UserAccessToken<'_>>::encode(
                    &inner.environment_configuration.subject.encryption.private_key,
                    &user_access_token_new,
                )?,
                user_access_refresh_token_encoded: Encoder::<UserAccessRefreshToken<'_>>::encode(
                    &inner.environment_configuration.subject.encryption.private_key,
                    user_access_refresh_token.user__id,
                    user_access_refresh_token.user_device__id,
                    user_access_refresh_token.user_access_token__id.as_ref(),
                    user_access_refresh_token.obfuscation_value.as_str(),
                    user_access_refresh_token.expires_at,
                    user_access_refresh_token.updated_at,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
