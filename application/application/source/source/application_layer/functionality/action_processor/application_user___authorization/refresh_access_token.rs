use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_refresh_token::{
                ApplicationUserAccessRefreshToken,
                ApplicationUserAccessRefreshToken_ExpiresAt,
                ApplicationUserAccessRefreshToken_ObfuscationValue,
                ApplicationUserAccessRefreshToken_UpdatedAt,
            },
            application_user_access_token::{
                ApplicationUserAccessToken,
                ApplicationUserAccessToken_ExpiresAt,
                ApplicationUserAccessToken_Id,
            },
        },
        functionality::service::{
            encoder::Encoder,
            generator::Generator,
        },
    },
    infrastructure_layer::{
        data::{
            capture::Capture,
            control_type::{
                ApplicationUser__Authorization___RefreshAccessToken,
            },
        },
        functionality::{
            repository::postgresql::{
                application_user_access_refresh_token::{
                    By2,
                    Update1,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::ExpirationTimeChecker,
        },
    },
};
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
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
            let application_user_access_token = Encoder::<ApplicationUserAccessToken<'_>>::decode(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )?;
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
                database_2_postgresql_connection,
                By2 {
                    application_user__id: application_user_access_token.application_user__id,
                    application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                },
            )
            .await?
            {
                Some(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_NotFound));
                }
            };
            let is_valid = Encoder::<ApplicationUserAccessRefreshToken<'_>>::is_valid(
                inner.environment_configuration,
                &application_user_access_refresh_token,
                &incoming.application_user_access_refresh_token_encoded,
            )?;
            if !is_valid || application_user_access_token.id.as_str() != application_user_access_refresh_token.application_user_access_token__id.as_ref() {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_refresh_token.expires_at) {
                PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By2 {
                        application_user__id: application_user_access_token.application_user__id,
                        application_user_device__id: application_user_access_token.application_user_device__id.as_ref(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_AlreadyExpired));
            }
            let application_user_access_token_new = ApplicationUserAccessToken::new(
                Generator::<ApplicationUserAccessToken_Id>::generate(),
                application_user_access_token.application_user__id,
                Cow::Borrowed(application_user_access_token.application_user_device__id.as_ref()),
                Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
            );
            application_user_access_refresh_token.application_user_access_token__id = Cow::Borrowed(application_user_access_token_new.id.as_str());
            application_user_access_refresh_token.obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();
            application_user_access_refresh_token.expires_at = Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?;
            application_user_access_refresh_token.updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();
            PostgresqlRepository::<ApplicationUserAccessRefreshToken>::update_1(
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
                application_user_access_token_encoded: Encoder::<ApplicationUserAccessToken<'_>>::encode(
                    inner.environment_configuration,
                    &application_user_access_token_new,
                )?,
                application_user_access_refresh_token_encoded: Encoder::<ApplicationUserAccessRefreshToken<'_>>::encode(
                    inner.environment_configuration,
                    &application_user_access_refresh_token,
                )?,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
