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
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserAccessRefreshTokenBy2,
                        UserAccessRefreshTokenUpdate,
                    }, Repository
                },
                service::{resolver::{
                    Resolver,
                    UnixTime,
                }, task_spawner::TaskSpawner},
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::refresh_access_token::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct RefreshAccessToken;
impl ActionProcessor_ for ActionProcessor<RefreshAccessToken> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            let sended = Sended_::new(&raw const incoming as *const Self::Incoming<'static>);
            if let Option::Some(precedent) = crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> Result<Option<Precedent>, AggregateError> {
                        let incoming_ = unsafe { sended.read_() };
                        if !Encoder::<UserAccessToken>::is_valid(
                            private_key,
                            &incoming_.user_access_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        const QUANTITY_OF_MICROSECONDS_UNTIL_TIME: i64 = 1000000 * 60 * 5;
                        const QUANTITY_OF_MICROSECONDS_FOR_CLIENT_TIME_DESYNCHRONIZATION_CASE: i64 = 1000000 * 10;
                        const QUANTITY_OF_MICROSECONDS: i64 = QUANTITY_OF_MICROSECONDS_UNTIL_TIME + QUANTITY_OF_MICROSECONDS_FOR_CLIENT_TIME_DESYNCHRONIZATION_CASE;
                        static_assertions::const_assert!(
                            (UserAccessToken_ExpiresAt::QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION / QUANTITY_OF_MICROSECONDS) >= 30
                        );
                        if (incoming_.user_access_token_signed.user_access_token__expires_at - QUANTITY_OF_MICROSECONDS) > now {
                            return Result::Ok(Option::Some(Precedent::UserAccessToken__NotReadyToRefresh));
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_access_token__obfuscation_value_, user_access_refresh_token__obfuscation_value, user_access_refresh_token__expires_at, user_access_refresh_token__updated_at) =
                match Repository::<Postgresql<UserAccessRefreshToken>>::find(
                    &postgresql_client_database_2,
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
            if user_access_refresh_token__expires_at <= now {
                if !Repository::<Postgresql<UserAccessRefreshToken>>::delete_1(
                    &postgresql_client_database_2,
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
            if !crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> _ {
                        let incoming_ = unsafe { sended.read_() };
                        return Encoder::<UserAccessRefreshToken>::is_valid(
                            private_key,
                            incoming_.user_access_token_signed.user__id,
                            incoming_.user_access_token_signed.user_device__id,
                            incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                            user_access_refresh_token__obfuscation_value,
                            user_access_refresh_token__expires_at,
                            user_access_refresh_token__updated_at,
                            &incoming_.user_access_refresh_token_signed,
                        )
                    },
                ).await
            )? || incoming.user_access_token_signed.user_access_token__obfuscation_value != user_access_token__obfuscation_value_ {
                return Result::Err(crate::new_invalid_argument!());
            }
            let new___user_access_token__obfuscation_value = Generator::<UserAccessToken_ObfuscationValue>::generate();
            let new___user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let new___user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let new___user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            if !Repository::<Postgresql<UserAccessRefreshToken>>::update(
                &postgresql_client_database_2,
                UserAccessRefreshTokenUpdate {
                    user_access_token__obfuscation_value: new___user_access_token__obfuscation_value,
                    user_access_refresh_token__obfuscation_value: new___user_access_refresh_token__obfuscation_value,
                    user_access_refresh_token__expires_at: new___user_access_refresh_token__expires_at,
                    user_access_refresh_token__updated_at: now,
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
            let (
                user_access_token_signed,
                user_access_refresh_token_signed,
            ) = crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> _ {
                        let incoming_ = unsafe { sended.read_() };
                        return Ok(
                            (
                                Encoder::<UserAccessToken>::encode(
                                    private_key,
                                    incoming_.user_access_token_signed.user__id,
                                    incoming_.user_access_token_signed.user_device__id,
                                    new___user_access_token__obfuscation_value,
                                    new___user_access_token__expires_at,
                                )?,
                                Encoder::<UserAccessRefreshToken>::encode(
                                    private_key,
                                    incoming_.user_access_token_signed.user__id,
                                    incoming_.user_access_token_signed.user_device__id,
                                    new___user_access_token__obfuscation_value,
                                    new___user_access_refresh_token__obfuscation_value,
                                    new___user_access_refresh_token__expires_at,
                                    now,
                                )?,
                            ),
                        );
                    },
                ).await
            )?;
            let outcoming = Outcoming {
                user_access_token_signed,
                user_access_refresh_token_signed,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
