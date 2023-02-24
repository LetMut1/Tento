use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::environment_configuration::ENVIRONMENT_CONFIGURATION_FILE_PATH;
use crate::infrastructure_layer::functionality::service::environment_configuration__creator::EnvironmentConfiguration_Creator;
use crate::infrastructure_layer::functionality::service::postgresql_connection_pool_creator::PostgresqlConnectionPoolCreator;
use extern_crate::tokio_postgres::NoTls;
use extern_crate::tokio::runtime::Builder;
use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_authorization_token__expiration_time_resolver::ApplicationUserAuthorizationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__value_generator::ApplicationUserAuthorizationToken_ValueGenerator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::ApplicationUserAuthorizationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Update;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct CreateFixturesProcessor;

impl CreateFixturesProcessor {
    const APPLICATION_USER__EMAIL: &'static str = "fixture_user@fixture.fi";

    pub fn process() -> Result<(), ErrorAuditor> {
        let environment_configuration = match EnvironmentConfiguration_Creator::create_from_configuration_file(ENVIRONMENT_CONFIGURATION_FILE_PATH) {
            Ok(environment_configuration_) => environment_configuration_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if let Environment::Production = *environment_configuration.get_environment() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { message: "CreateFixturesProcessor should process only not in production environment." },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let runtime = match Builder::new_current_thread()
            .enable_io()
            .build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(mut error) = runtime.block_on(
            Self::create_fixtures(&environment_configuration)
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    async fn create_fixtures<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<(), ErrorAuditor> {
        let database_1_postgresql_connection_pool = match PostgresqlConnectionPoolCreator::<NoTls>::create(
            environment_configuration.get_environment(),
            environment_configuration.get_database_1_postgresql_configuration()
        ).await {
            Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let is_valid_email = match ApplicationUser_Validator::is_valid_email(Self::APPLICATION_USER__EMAIL) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_email {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { message: "Application user email should be valid." },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        // let application_user = match ApplicationUser_PostgresqlRepository::find_2(
        //     database_1_postgresql_connection, Self::APPLICATION_USER__EMAIL
        // ).await {
        //     Ok(application_user_) => application_user_,
        //     Err(mut error) => {
        //         error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

        //         return Err(error);
        //     }
        // };
        // let application_user_ = match application_user {
        //     Some(application_user__) => application_user__,
        //     None => {

        //         //cсозздать
        //     }
        // };

        return Ok(());
    }
}