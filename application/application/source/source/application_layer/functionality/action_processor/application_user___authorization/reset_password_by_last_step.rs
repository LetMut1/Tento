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
                ApplicationUser_Password,
            },
            application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
            application_user_device::ApplicationUserDevice_Id,
            application_user_reset_password_token::{
                ApplicationUserResetPasswordToken,
                ApplicationUserResetPasswordToken_Value,
                ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            encoder::Encoder,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
                ErrorConverter,
                OptionConverter,
            },
            control_type::{
                ApplicationUser__Authorization___ResetPasswordByLastStep,
                CloudMessage,
                TokioBlockingTask,
                TokioNonBlockingTask,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
            void::Void,
        },
        functionality::{
            repository::postgresql::{
                application_user::{
                    By3,
                    Update1,
                },
                application_user_access_refresh_token::By1,
                application_user_reset_password_token::{
                    By1 as By1_,
                    Update4,
                },
                PostgresqlRepository,
            },
            service::{
                expiration_time_checker::ExpirationTimeChecker,
                resolver::Resolver,
                spawner::Spawner,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::{
    Incoming,
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
impl ActionProcessor<ApplicationUser__Authorization___ResetPasswordByLastStep> {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Void, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;
        if !Validator::<ApplicationUserResetPasswordToken_Value>::is_valid(incoming_.application_user_reset_password_token__value.as_str())? {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user__id) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUser_Password>::is_valid_part_1(incoming_.application_user_password.as_str()) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device__id.as_str()) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let mut application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_2(
            database_2_postgresql_connection,
            By1_ {
                application_user__id: incoming_.application_user__id,
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(application_user_reset_password_token_) => application_user_reset_password_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(
                    Precedent::ApplicationUserResetPasswordToken_NotFound,
                )));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.expires_at) {
            PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                database_2_postgresql_connection,
                By1_ {
                    application_user__id: incoming_.application_user__id,
                    application_user_device__id: incoming_.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserResetPasswordToken_AlreadyExpired,
            )));
        }
        if !application_user_reset_password_token.is_approved {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserResetPasswordToken_IsNotApproved,
            )));
        }
        if application_user_reset_password_token.value != incoming_.application_user_reset_password_token__value {
            application_user_reset_password_token.wrong_enter_tries_quantity =
                application_user_reset_password_token.wrong_enter_tries_quantity.checked_add(1).convert_out_of_range(Backtrace::new(line!(), file!()))?;
            if application_user_reset_password_token.wrong_enter_tries_quantity < ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_4(
                    database_2_postgresql_connection,
                    Update4 {
                        application_user_reset_password_token__wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                    },
                    By1_ {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                    },
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1_ {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                    },
                )
                .await?;
            }
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserResetPasswordToken_WrongValue,
            )));
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        let mut application_user = match PostgresqlRepository::<ApplicationUser>::find_5(
            database_1_postgresql_connection,
            By3 {
                application_user__id: incoming_.application_user__id,
            },
        )
        .await?
        {
            Some(application_user_) => application_user_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(
                    Precedent::ApplicationUser_NotFound,
                )));
            }
        };
        if !Validator::<ApplicationUser_Password>::is_valid_part_2(
            incoming_.application_user_password.as_str(),
            application_user.email.as_str(),
            application_user.nickname.as_str(),
        ) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        let closure = move || -> _ {
            return Encoder::<ApplicationUser_Password>::encode(incoming_.application_user_password.as_str());
        };
        let join_handle = Spawner::<TokioBlockingTask>::spawn_processed(closure);
        application_user.password_hash = join_handle.await.convert(Backtrace::new(line!(), file!()))??;
        PostgresqlRepository::<ApplicationUser>::update_1(
            database_1_postgresql_connection,
            Update1 {
                application_user__password_hash: application_user.password_hash.as_str(),
            },
            By3 {
                application_user__id: incoming_.application_user__id,
            },
        )
        .await?;
        PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
            &*database_2_postgresql_pooled_connection,
            By1 {
                application_user__id: incoming_.application_user__id,
            },
        )
        .await?;
        Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();
        let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();
        let future = async move {
            let database_2_postgresql_pooled_connection_ = database_2_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;
            PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                &*database_2_postgresql_pooled_connection_,
                By1_ {
                    application_user__id: incoming_.application_user__id,
                    application_user_device__id: incoming_.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(());
        };
        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);
        return Ok(Ok(UnifiedReport::target_empty()));
    }
}
