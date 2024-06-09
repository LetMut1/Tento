use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser5;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::derivative::ApplicationUserAuthorizationToken3;
use crate::domain_layer::data::entity::application_user_authorization_token::derivative::ApplicationUserAuthorizationToken5;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By3;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update5;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___SendEmailForAuthorize;

impl ActionProcessor<ApplicationUser__Authorization___SendEmailForAuthorize> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
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

        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device_id.as_str()) {
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

        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user_id) {
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

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let application_user = match PostgresqlRepository::<ApplicationUser5>::find_1(
            &*database_1_postgresql_pooled_connection,
            &By3 {
                application_user_id: incoming_.application_user_id,
            },
        )
        .await?
        {
            Some(application_user_) => application_user_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound)));
            }
        };

        let by_4 = By4 {
            application_user_id: incoming_.application_user_id,
            application_user_device_id: incoming_.application_user_device_id.as_str(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let mut application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken5>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await?
        {
            Some(application_user_authorization_token_) => application_user_authorization_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_NotFound)));
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.expires_at) {
            PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_4,
            )
            .await?;

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_AlreadyExpired)));
        }

        if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.can_be_resent_from) {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_TimeToResendHasNotCome)));
        }

        application_user_authorization_token.can_be_resent_from = Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?;

        PostgresqlRepository::<ApplicationUserAuthorizationToken3>::update(
            database_2_postgresql_connection,
            &Update5 {
                application_user_authorization_token_can_be_resent_from: application_user_authorization_token.can_be_resent_from,
            },
            &by_4,
        )
        .await?;

        EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
            environment_configuration,
            application_user_authorization_token.value.as_str(),
            application_user.email.as_str(),
            incoming_.application_user_device_id.as_str(),
        )?;

        let outcoming = Outcoming {
            application_user_authorization_token_can_be_resent_from: application_user_authorization_token.can_be_resent_from,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}