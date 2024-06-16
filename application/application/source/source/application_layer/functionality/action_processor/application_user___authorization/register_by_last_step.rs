use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::{
                ApplicationUser,
                ApplicationUser_Email,
                ApplicationUser_Nickname,
                ApplicationUser_Password,
            },
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
            application_user_device::{
                ApplicationUserDevice,
                ApplicationUserDevice_Id,
            },
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_Value,
                ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            encoder::Encoder,
            form_resolver::FormResolver,
            generator::Generator,
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
                ApplicationUser__Authorization___RegisterByLastStep,
                TokioBlockingTask,
                TokioNonBlockingTask,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
        },
        functionality::{
            repository::postgresql::{
                application_user::{
                    By1,
                    By2,
                    Insert1 as ApplicationUserInsert1,
                },
                application_user_access_refresh_token::Insert1 as ApplicationUserAccessRefreshTokenInsert1,
                application_user_device::Insert1,
                application_user_registration_token::{
                    By1 as By1_,
                    Update4,
                },
                PostgresqlRepository,
            },
            service::{
                expiration_time_checker::ExpirationTimeChecker,
                spawner::Spawner,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::{
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
impl ActionProcessor<ApplicationUser__Authorization___RegisterByLastStep> {
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
        if !Validator::<ApplicationUser_Password>::is_valid(
            incoming_.application_user_password.as_str(),
            incoming_.application_user__email.as_str(),
            incoming_.application_user__nickname.as_str(),
        ) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUser_Nickname>::is_valid(incoming_.application_user__nickname.as_str()) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user__email.as_str())? {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(incoming_.application_user_registration_token_value.as_str())? {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device_id.as_str()) {
            return Ok(Err(Auditor::<InvalidArgument>::new(
                InvalidArgument,
                Backtrace::new(line!(), file!()),
            )));
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
            database_1_postgresql_connection,
            By1 {
                application_user__nickname: incoming_.application_user__nickname.as_str(),
            },
        )
        .await?
        {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUser_NicknameAlreadyExist,
            )));
        }
        if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            database_1_postgresql_connection,
            By2 {
                application_user__email: incoming_.application_user__email.as_str(),
            },
        )
        .await?
        {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUser_EmailAlreadyExist,
            )));
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_2(
            database_2_postgresql_connection,
            By1_ {
                application_user__email: incoming_.application_user__email.as_str(),
                application_user_device_id: incoming_.application_user_device_id.as_str(),
            },
        )
        .await?
        {
            Some(application_user_registration_token_) => application_user_registration_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(
                    Precedent::ApplicationUserRegistrationToken_NotFound,
                )));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at) {
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                database_2_postgresql_connection,
                By1_ {
                    application_user__email: incoming_.application_user__email.as_str(),
                    application_user_device_id: incoming_.application_user_device_id.as_str(),
                },
            )
            .await?;
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_AlreadyExpired,
            )));
        }
        if !application_user_registration_token.is_approved {
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_IsNotApproved,
            )));
        }
        if application_user_registration_token.value != incoming_.application_user_registration_token_value {
            application_user_registration_token.wrong_enter_tries_quantity =
                application_user_registration_token.wrong_enter_tries_quantity.checked_add(1).convert_out_of_range(Backtrace::new(line!(), file!()))?;
            if application_user_registration_token.wrong_enter_tries_quantity < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserRegistrationToken>::update_4(
                    database_2_postgresql_connection,
                    Update4 {
                        application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                    },
                    By1_ {
                        application_user__email: incoming_.application_user__email.as_str(),
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                    },
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1_ {
                        application_user__email: incoming_.application_user__email.as_str(),
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                    },
                )
                .await?;
            }
            return Ok(Ok(UnifiedReport::precedent(
                Precedent::ApplicationUserRegistrationToken_WrongValue,
            )));
        }
        let closure = move || -> _ {
            return Encoder::<ApplicationUser_Password>::encode(incoming_.application_user_password.as_str());
        };
        let join_handle = Spawner::<TokioBlockingTask>::spawn_processed(closure);
        let application_user = PostgresqlRepository::<ApplicationUser<'_>>::create_1(
            database_1_postgresql_connection,
            ApplicationUserInsert1 {
                application_user__email: incoming_.application_user__email,
                application_user__nickname: incoming_.application_user__nickname,
                application_user__password_hash: join_handle.await.convert(Backtrace::new(line!(), file!()))??,
            },
        )
        .await?;
        let application_user_access_token = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user.id,
            Cow::Borrowed(incoming_.application_user_device_id.as_str()),
            Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
        );
        // TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
        let application_user_access_refresh_token = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create_1(
            database_2_postgresql_connection,
            ApplicationUserAccessRefreshTokenInsert1 {
                application_user__id: application_user.id,
                application_user_device_id: incoming_.application_user_device_id.as_str(),
                application_user_access_token_id: application_user_access_token.id.as_str(),
                application_user_access_refresh_token_obfuscation_value: Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate(),
                application_user_access_refresh_token_expires_at: Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?,
                application_user_access_refresh_token_updated_at: Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate(),
            },
        )
        .await?;
        let application_user_access_token_encrypted = FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(environment_configuration, &application_user_access_token)?;
        let application_user_access_refresh_token_encrypted = FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(
            environment_configuration,
            &application_user_access_refresh_token,
        )?;
        let database_1_postgresql_connection_pool_ = database_1_postgresql_connection_pool.clone();
        let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();
        let future = async move {
            let database_1_postgresql_pooled_connection_ = database_1_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;
            let application_user_device = PostgresqlRepository::<ApplicationUserDevice>::create_1(
                &*database_1_postgresql_pooled_connection_,
                Insert1 {
                    application_user_device_id: incoming_.application_user_device_id,
                    application_user__id: application_user.id,
                },
            )
            .await?;
            let database_2_postgresql_pooled_connection_ = database_2_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                &*database_2_postgresql_pooled_connection_,
                By1_ {
                    application_user__email: application_user.email.as_str(),
                    application_user_device_id: application_user_device.id.as_str(),
                },
            )
            .await?;
            return Ok(());
        };
        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);
        let outcoming = Outcoming {
            application_user_access_token_encrypted,
            application_user_access_refresh_token_encrypted,
        };
        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
