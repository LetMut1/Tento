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
            application_user_authorization_token::{
                ApplicationUserAuthorizationToken,
                ApplicationUserAuthorizationToken_Value,
                ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
            },
            application_user_device::{
                ApplicationUserDevice,
                ApplicationUserDevice_Id,
            },
        },
        functionality::service::{
            form_resolver::FormResolver,
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
                ApplicationUser__Authorization___AuthorizeByLastStep,
                TokioNonBlockingTask,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::{
            repository::postgresql::{
                application_user::By3,
                application_user_access_refresh_token::{
                    By2,
                    Insert1 as ApplicationUserAccessRefreshTokenInsert1,
                    Update1,
                },
                application_user_authorization_token::{
                    By1,
                    Update4,
                },
                application_user_device::Insert1 as ApplicationUserDeviceInsert1,
                PostgresqlRepository,
            },
            service::{
                expiration_time_checker::ExpirationTimeChecker,
                spawner::Spawner,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::{
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
impl ActionProcessor<ApplicationUser__Authorization___AuthorizeByLastStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>, // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, AlternativeWorkflow>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.into_internal_error_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if !Validator::<ApplicationUser_Id>::is_valid(incoming_.application_user__id) {
            return Err(
                AlternativeWorkflow::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        if !Validator::<ApplicationUserAuthorizationToken_Value>::is_valid(incoming_.application_user_authorization_token__value.as_str())? {
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
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.into_internal_error_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
        let application_user_authorization_token = PostgresqlRepository::<ApplicationUserAuthorizationToken>::find_2(
            database_2_postgresql_connection,
            By1 {
                application_user__id: incoming_.application_user__id,
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?;
        let mut application_user_authorization_token_ = match application_user_authorization_token {
            Some(application_user_authorization_token__) => application_user_authorization_token__,
            None => {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_NotFound));
            }
        };
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.expires_at) {
            PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                database_2_postgresql_connection,
                By1 {
                    application_user__id: incoming_.application_user__id,
                    application_user_device__id: incoming_.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_AlreadyExpired));
        }
        if application_user_authorization_token_.value != incoming_.application_user_authorization_token__value {
            application_user_authorization_token_.wrong_enter_tries_quantity =
                application_user_authorization_token_.wrong_enter_tries_quantity.checked_add(1).into_internal_error_logic_out_of_range(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            if application_user_authorization_token_.wrong_enter_tries_quantity < ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_4(
                    database_2_postgresql_connection,
                    Update4 {
                        application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                    },
                    By1 {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                    },
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By1 {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                    },
                )
                .await?;
            }
            return Ok(
                UnifiedReport::precedent(
                    Precedent::ApplicationUserAuthorizationToken_WrongValue {
                        application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                    },
                ),
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_internal_error_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        if !PostgresqlRepository::<ApplicationUser<'_>>::is_exist_3(
            database_1_postgresql_connection,
            By3 {
                application_user__id: incoming_.application_user__id,
            },
        )
        .await?
        {
            return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound));
        }
        let application_user_access_token = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            incoming_.application_user__id,
            Cow::Borrowed(incoming_.application_user_device__id.as_str()),
            Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
        );
        let application_user_access_token__id = application_user_access_token.id.as_str();
        let application_user_access_refresh_token__obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();
        let application_user_access_refresh_token__expires_at = Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?;
        let application_user_access_refresh_token__updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();
        // TODO  TRANZACTION
        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            By2 {
                application_user__id: incoming_.application_user__id,
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(mut application_user_access_refresh_token_) => {
                application_user_access_refresh_token_.application_user_access_token__id = Cow::Borrowed(application_user_access_token__id);
                application_user_access_refresh_token_.obfuscation_value = application_user_access_refresh_token__obfuscation_value;
                application_user_access_refresh_token_.expires_at = application_user_access_refresh_token__expires_at;
                application_user_access_refresh_token_.updated_at = application_user_access_refresh_token__updated_at;
                PostgresqlRepository::<ApplicationUserAccessRefreshToken>::update_1(
                    database_2_postgresql_connection,
                    Update1 {
                        application_user_access_token__id: application_user_access_refresh_token_.application_user_access_token__id.as_ref(),
                        application_user_access_refresh_token__obfuscation_value: application_user_access_refresh_token_.obfuscation_value.as_str(),
                        application_user_access_refresh_token__expires_at: application_user_access_refresh_token_.expires_at,
                        application_user_access_refresh_token__updated_at: application_user_access_refresh_token_.updated_at,
                    },
                    By2 {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                    },
                )
                .await?;
                application_user_access_refresh_token_
            }
            None => {
                let application_user_access_refresh_token_ = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create_1(
                    database_2_postgresql_connection,
                    ApplicationUserAccessRefreshTokenInsert1 {
                        application_user__id: incoming_.application_user__id,
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
                        application_user_access_token__id,
                        application_user_access_refresh_token__obfuscation_value,
                        application_user_access_refresh_token__expires_at,
                        application_user_access_refresh_token__updated_at,
                    },
                )
                .await?;
                application_user_access_refresh_token_
            }
        };
        // TODO  TRANZACTION
        let application_user_access_token_encrypted = FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(
            environment_configuration,
            &application_user_access_token,
        )?;
        let application_user_access_refresh_token_encrypted = FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(
            environment_configuration,
            &application_user_access_refresh_token,
        )?;
        let database_1_postgresql_connection_pool_ = database_1_postgresql_connection_pool.clone();
        let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                let database_1_postgresql_pooled_connection_ = database_1_postgresql_connection_pool_.get().await.into_internal_error_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                let application_user_device = PostgresqlRepository::<ApplicationUserDevice>::create_1(
                    &*database_1_postgresql_pooled_connection_,
                    ApplicationUserDeviceInsert1 {
                        application_user_device__id: incoming_.application_user_device__id,
                        application_user__id: incoming_.application_user__id,
                    },
                )
                .await?;
                let database_2_postgresql_pooled_connection_ = database_2_postgresql_connection_pool_.get().await.into_internal_error_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                    &*database_2_postgresql_pooled_connection_,
                    By1 {
                        application_user__id: application_user_device.application_user__id,
                        application_user_device__id: application_user_device.id.as_str(),
                    },
                )
                .await?;
                return Ok(());
            },
        );
        let outcoming = Outcoming {
            application_user_access_token_encrypted,
            application_user_access_refresh_token_encrypted,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
