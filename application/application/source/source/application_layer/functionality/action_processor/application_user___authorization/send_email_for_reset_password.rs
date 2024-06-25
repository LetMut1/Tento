use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::{
                ApplicationUser,
                ApplicationUser_Id,
            },
            application_user_device::ApplicationUserDevice_Id,
            application_user_reset_password_token::{
                ApplicationUserResetPasswordToken,
                ApplicationUserResetPasswordToken_CanBeResentFrom,
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
            auditor::{
                Backtrace,
            },
            control_type::{
                ApplicationUser__Authorization___SendEmailForResetPassword,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::{
                Error,
                OptionConverter,
                ResultConverter,
            },
        },
        functionality::{
            repository::postgresql::{
                application_user::By3,
                application_user_reset_password_token::{
                    By1,
                    Update2,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::ExpirationTimeChecker,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::{
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
impl ActionProcessor<ApplicationUser__Authorization___SendEmailForResetPassword> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, Error>
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
        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user__id) {
            return Err(
                Error::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    )
                )
            );
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device__id.as_str()) {
            return Err(
                Error::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    )
                )
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_user = match PostgresqlRepository::<ApplicationUser>::find_6(
            &*database_1_postgresql_pooled_connection,
            By3 {
                application_user__id: incoming_.application_user__id,
            },
        )
        .await?
        {
            Some(application_user_) => application_user_,
            None => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound));
            }
        };
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let mut application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_3(
            database_2_postgresql_connection,
            By1 {
                application_user__id: incoming_.application_user__id,
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(application_user_reset_password_token_) => application_user_reset_password_token_,
            None => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_NotFound));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.expires_at) {
            PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                database_2_postgresql_connection,
                By1 {
                    application_user__id: incoming_.application_user__id,
                    application_user_device__id: incoming_.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired));
        }
        if application_user_reset_password_token.is_approved {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyApproved));
        }
        if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.can_be_resent_from) {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome));
        }
        application_user_reset_password_token.can_be_resent_from = Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate()?;
        PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_2(
            database_2_postgresql_connection,
            Update2 {
                application_user_reset_password_token__can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
            },
            By1 {
                application_user__id: incoming_.application_user__id,
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?;
        EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
            environment_configuration,
            application_user_reset_password_token.value.as_str(),
            application_user.email.as_str(),
            incoming_.application_user_device__id.as_str(),
        )?;
        let outcoming = Outcoming {
            application_user_reset_password_token__can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
