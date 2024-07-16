use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::ApplicationUser_Email,
            application_user_device::ApplicationUserDevice_Id,
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_CanBeResentFrom,
            },
        },
        functionality::service::{
            email_sender::EmailSender,
            generator::Generator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            alternative_workflow::{
                AlternativeWorkflow,
                OptionConverter,
                ResultConverter,
            },
            auditor::Backtrace,
            control_type::{
                ApplicationUser__Authorization___SendEmailForRegister,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::{
            repository::postgresql::{
                application_user_registration_token::{
                    By1,
                    Update2,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::ExpirationTimeChecker,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_register::{
    Incoming,
    Outcoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
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
impl ActionProcessor<ApplicationUser__Authorization___SendEmailForRegister> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, AlternativeWorkflow>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.into_internal_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user__email.as_str())? {
            return Err(
                AlternativeWorkflow::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device__id.as_str()) {
            return Err(
                AlternativeWorkflow::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_3(
            database_2_postgresql_connection,
            By1 {
                application_user__email: incoming_.application_user__email.as_str(),
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(application_user_registration_token_) => application_user_registration_token_,
            None => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at) {
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                database_2_postgresql_connection,
                By1 {
                    application_user__email: incoming_.application_user__email.as_str(),
                    application_user_device__id: incoming_.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired));
        }
        if application_user_registration_token.is_approved {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyApproved));
        }
        if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.can_be_resent_from) {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_TimeToResendHasNotCome));
        }
        application_user_registration_token.can_be_resent_from = Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate()?;
        PostgresqlRepository::<ApplicationUserRegistrationToken>::update_2(
            database_2_postgresql_connection,
            Update2 {
                application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
            },
            By1 {
                application_user__email: incoming_.application_user__email.as_str(),
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?;
        EmailSender::<ApplicationUserRegistrationToken<'_>>::send(
            environment_configuration,
            application_user_registration_token.value.as_str(),
            incoming_.application_user__email.as_str(),
            incoming_.application_user_device__id.as_str(),
        )?;
        let outcoming = Outcoming {
            application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
