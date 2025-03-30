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
                    UserAccessToken_ObfuscationValue,
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
                    Repository,
                    postgresql::{
                        Postgresql,
                        UserAccessRefreshTokenBy2,
                        UserAccessRefreshTokenUpdate,
                    },
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
    std::future::Future,
};
pub struct UserAuthorization_RefreshAccessToken;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RefreshAccessToken> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_access_token__obfuscation_value_, user_access_refresh_token__obfuscation_value, user_access_refresh_token__expires_at, user_access_refresh_token__updated_at) =
                match Repository::<Postgresql<UserAccessRefreshToken>>::find(
                    &postgresql_database_2_client,
                    UserAccessRefreshTokenBy2 {
                        user__id: incoming.user_access_token_signed.user__id,
                        user_device__id: incoming.user_access_token_signed.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessRefreshToken__NotFound)),
                };
            let is_valid = Encoder::<UserAccessRefreshToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                incoming.user_access_token_signed.user_device__id,
                incoming.user_access_token_signed.user_access_token__obfuscation_value,
                user_access_refresh_token__obfuscation_value,
                user_access_refresh_token__expires_at,
                user_access_refresh_token__updated_at,
                &incoming.user_access_refresh_token_signed,
            )?;
            if !is_valid || incoming.user_access_token_signed.user_access_token__obfuscation_value != user_access_token__obfuscation_value_ {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if user_access_refresh_token__expires_at <= now {
                if !Repository::<Postgresql<UserAccessRefreshToken>>::delete_1(
                    &postgresql_database_2_client,
                    UserAccessRefreshTokenBy2 {
                        user__id: incoming.user_access_token_signed.user__id,
                        user_device__id: incoming.user_access_token_signed.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessRefreshToken__AlreadyExpired));
            }
            let new___user_access_token__obfuscation_value = Generator::<UserAccessToken_ObfuscationValue>::generate();
            let new___user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let new___user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let new___user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let new___user_access_refresh_token__updated_at = now;
            if !Repository::<Postgresql<UserAccessRefreshToken>>::update(
                &postgresql_database_2_client,
                UserAccessRefreshTokenUpdate {
                    user_access_token__obfuscation_value: new___user_access_token__obfuscation_value,
                    user_access_refresh_token__obfuscation_value: new___user_access_refresh_token__obfuscation_value,
                    user_access_refresh_token__expires_at: new___user_access_refresh_token__expires_at,
                    user_access_refresh_token__updated_at: new___user_access_refresh_token__updated_at,
                },
                UserAccessRefreshTokenBy2 {
                    user__id: incoming.user_access_token_signed.user__id,
                    user_device__id: incoming.user_access_token_signed.user_device__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let outcoming = Outcoming {
                user_access_token_signed: Encoder::<UserAccessToken>::encode(
                    &inner.environment_configuration.subject.encryption.private_key,
                    new___user_access_token__obfuscation_value,
                    incoming.user_access_token_signed.user__id,
                    incoming.user_access_token_signed.user_device__id,
                    new___user_access_token__expires_at,
                )?,
                user_access_refresh_token_signed: Encoder::<UserAccessRefreshToken>::encode(
                    &inner.environment_configuration.subject.encryption.private_key,
                    incoming.user_access_token_signed.user__id,
                    incoming.user_access_token_signed.user_device__id,
                    new___user_access_token__obfuscation_value,
                    new___user_access_refresh_token__obfuscation_value,
                    new___user_access_refresh_token__expires_at,
                    new___user_access_refresh_token__updated_at,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
