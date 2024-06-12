use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_access_refresh_token::Update1;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RefreshAccessToken;

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
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;

        let application_user_access_token = match FormResolver::<ApplicationUserAccessToken<'_>>::from_encrypted(environment_configuration, incoming_.application_user_access_token_encrypted.as_str())? {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(invalid_argument_auditor) => {
                return Ok(Err(invalid_argument_auditor));
            }
        };

        let by_4 = By4 {
            application_user_id: application_user_access_token.application_user_id,
            application_user_device_id: application_user_access_token.application_user_device_id.as_ref(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let mut application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            &by_4,
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

        if !is_valid || application_user_access_token.id != application_user_access_refresh_token.application_user_access_token_id.as_ref() {
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
                &by_4,
            )
            .await?;

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_AlreadyExpired)));
        }

        let application_user_access_token_new = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user_access_token.application_user_id,
            Cow::Borrowed(application_user_access_token.application_user_device_id.as_ref()),
            Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
        );

        application_user_access_refresh_token.application_user_access_token_id = Cow::Borrowed(application_user_access_token_new.id.as_str());

        application_user_access_refresh_token.obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();

        application_user_access_refresh_token.expires_at = Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?;

        application_user_access_refresh_token.updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();

        PostgresqlRepository::<ApplicationUserAccessRefreshToken>::update_1(
            database_2_postgresql_connection,
            &Update1 {
                application_user_access_token_id: application_user_access_refresh_token.application_user_access_token_id.as_ref(),
                application_user_access_refresh_token_obfuscation_value: application_user_access_refresh_token.obfuscation_value.as_str(),
                application_user_access_refresh_token_expires_at: application_user_access_refresh_token.expires_at,
                application_user_access_refresh_token_updated_at: application_user_access_refresh_token.updated_at,
            },
            &by_4,
        )
        .await?;

        let outcoming = Outcoming {
            application_user_access_token_encrypted: FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(environment_configuration, &application_user_access_token_new)?,
            application_user_access_refresh_token_encrypted: FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(environment_configuration, &application_user_access_refresh_token)?,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}