use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
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
            form_resolver::FormResolver,
            generator::Generator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
                OptionConverter,
                ResultConverter,
            },
            control_type::{
                ApplicationUser__Authorization___RefreshAccessToken,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
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
use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::{
    Incoming,
    Outcoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    borrow::Cow,
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor<ApplicationUser__Authorization___RefreshAccessToken> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_user_access_token = match FormResolver::<ApplicationUserAccessToken<'_>>::from_encrypted(
            environment_configuration,
            incoming_.application_user_access_token_encrypted.as_str(),
        )? {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(invalid_argument_auditor) => {
                return Ok(Err(invalid_argument_auditor));
            }
        };
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
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
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_NotFound)));
            }
        };
        let is_valid = FormResolver::<ApplicationUserAccessRefreshToken<'_>>::is_valid(
            environment_configuration,
            &application_user_access_refresh_token,
            incoming_.application_user_access_refresh_token_encrypted.as_str(),
        )?;
        if !is_valid || application_user_access_token.id != application_user_access_refresh_token.application_user_access_token__id.as_ref() {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
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
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_AlreadyExpired)));
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
            application_user_access_token_encrypted: FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(
                environment_configuration,
                &application_user_access_token_new,
            )?,
            application_user_access_refresh_token_encrypted: FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(
                environment_configuration,
                &application_user_access_refresh_token,
            )?,
        };
        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
