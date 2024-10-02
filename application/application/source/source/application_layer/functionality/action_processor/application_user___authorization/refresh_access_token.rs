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
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                application_user_access_refresh_token::{
                    By2,
                    Update1,
                },
                PostgresqlRepository,
            },
            service::resolver::{
                expiration::Expiration,
                Resolver,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::{
    Incoming,
    Outcoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
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
use unified_report::UnifiedReport;
use void::Void;
pub struct ApplicationUser__Authorization___RefreshAccessToken;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___RefreshAccessToken> {
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
            let application_user_access_token = Encoder::<UserAccessToken<'_>>::decode(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )?;
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_access_refresh_token = match PostgresqlRepository::<UserAccessRefreshToken<'_>>::find_1(
                database_2_postgresql_connection,
                By2 {
                    application_user__id: application_user_access_token.application_user__id,
                    application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                },
            )
            .await?
            {
                Option::Some(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_NotFound));
                }
            };
            let is_valid = Encoder::<UserAccessRefreshToken<'_>>::is_valid(
                inner.environment_configuration,
                &application_user_access_refresh_token,
                &incoming.application_user_access_refresh_token_encoded,
            )?;
            if !is_valid || application_user_access_token.id.as_str() != application_user_access_refresh_token.application_user_access_token__id.as_ref() {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if Resolver::<Expiration>::is_expired(application_user_access_refresh_token.expires_at) {
                PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By2 {
                        application_user__id: application_user_access_token.application_user__id,
                        application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_AlreadyExpired));
            }
            let application_user_access_token_new = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                application_user_access_token.application_user__id,
                Cow::Borrowed(application_user_access_token.application_user_device__id.as_ref()),
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            application_user_access_refresh_token.application_user_access_token__id = Cow::Borrowed(application_user_access_token_new.id.as_str());
            application_user_access_refresh_token.obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            application_user_access_refresh_token.expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?;
            application_user_access_refresh_token.updated_at = Generator::<UserAccessRefreshToken_UpdatedAt>::generate();
            PostgresqlRepository::<UserAccessRefreshToken>::update_1(
                database_2_postgresql_connection,
                Update1 {
                    application_user_access_token__id: application_user_access_refresh_token.application_user_access_token__id.as_ref(),
                    application_user_access_refresh_token__obfuscation_value: application_user_access_refresh_token.obfuscation_value.as_str(),
                    application_user_access_refresh_token__expires_at: application_user_access_refresh_token.expires_at,
                    application_user_access_refresh_token__updated_at: application_user_access_refresh_token.updated_at,
                },
                By2 {
                    application_user__id: application_user_access_token.application_user__id,
                    application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                },
            )
            .await?;
            let outcoming = Outcoming {
                application_user_access_token_encoded: Encoder::<UserAccessToken<'_>>::encode(
                    inner.environment_configuration,
                    &application_user_access_token_new,
                )?,
                application_user_access_refresh_token_encoded: Encoder::<UserAccessRefreshToken<'_>>::encode(
                    inner.environment_configuration,
                    &application_user_access_refresh_token,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
