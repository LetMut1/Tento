use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By3;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_access_refresh_token::Insert1 as ApplicationUserAccessRefreshTokenInsert1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_device::Insert1 as ApplicationUserDeviceInsert1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_access_refresh_token::Update1;
use crate::infrastructure_layer::functionality::repository::postgresql::application_user_authorization_token::Update6;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::borrow::Cow;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use crate::application_layer::functionality::action_processor::ActionProcessor;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___AuthorizeByLastStep;

impl ActionProcessor<ApplicationUser__Authorization___AuthorizeByLastStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>, // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;

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

        if !Validator::<ApplicationUserAuthorizationToken_Value>::is_valid(incoming_.application_user_authorization_token_value.as_str())? {
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

        let by_4 = By4 {
            application_user_id: incoming_.application_user_id,
            application_user_device_id: incoming_.application_user_device_id.as_str(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = PostgresqlRepository::<ApplicationUserAuthorizationToken>::find_2(
            database_2_postgresql_connection,
            &by_4,
        )
        .await?;

        let mut application_user_authorization_token_ = match application_user_authorization_token {
            Some(application_user_authorization_token__) => application_user_authorization_token__,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_NotFound)));
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.expires_at) {
            PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                database_2_postgresql_connection,
                &by_4,
            )
            .await?;

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_AlreadyExpired)));
        }

        if application_user_authorization_token_.value != incoming_.application_user_authorization_token_value {
            application_user_authorization_token_.wrong_enter_tries_quantity = application_user_authorization_token_.wrong_enter_tries_quantity
                .checked_add(1)
                .convert_out_of_range(Backtrace::new(line!(), file!()))?;

            if application_user_authorization_token_.wrong_enter_tries_quantity < ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_4(
                    database_2_postgresql_connection,
                    &Update6 {
                        application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                    },
                    &by_4,
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    &by_4,
                )
                .await?;
            }

            return Ok(
                Ok(
                    UnifiedReport::precedent(
                        Precedent::ApplicationUserAuthorizationToken_WrongValue {
                            application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                        },
                    ),
                ),
            );
        }

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        if !PostgresqlRepository::<ApplicationUser<'_>>::is_exist_3(
            database_1_postgresql_connection,
            &By3 {
                application_user_id: incoming_.application_user_id,
            },
        )
        .await?
        {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound)));
        }

        let application_user_access_token = ApplicationUserAccessToken::new(
            Generator::<ApplicationUserAccessToken_Id>::generate(),
            incoming_.application_user_id,
            Cow::Borrowed(incoming_.application_user_device_id.as_str()),
            Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
        );

        let application_user_access_token_id = application_user_access_token.id.as_str();

        let application_user_access_refresh_token_obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();

        let application_user_access_refresh_token_expires_at = Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?;

        let application_user_access_refresh_token_updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();
        // TODO  TRANZACTION
        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await?
        {
            Some(mut application_user_access_refresh_token_) => {
                application_user_access_refresh_token_.application_user_access_token_id = Cow::Borrowed(application_user_access_token_id);

                application_user_access_refresh_token_.obfuscation_value = application_user_access_refresh_token_obfuscation_value;

                application_user_access_refresh_token_.expires_at = application_user_access_refresh_token_expires_at;

                application_user_access_refresh_token_.updated_at = application_user_access_refresh_token_updated_at;

                PostgresqlRepository::<ApplicationUserAccessRefreshToken>::update_1(
                    database_2_postgresql_connection,
                    &Update1 {
                        application_user_access_token_id: application_user_access_refresh_token_.application_user_access_token_id.as_ref(),
                        application_user_access_refresh_token_obfuscation_value: application_user_access_refresh_token_.obfuscation_value.as_str(),
                        application_user_access_refresh_token_expires_at: application_user_access_refresh_token_.expires_at,
                        application_user_access_refresh_token_updated_at: application_user_access_refresh_token_.updated_at,
                    },
                    &by_4,
                )
                .await?;

                application_user_access_refresh_token_
            }
            None => {
                let application_user_access_refresh_token_ = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create_1(
                    database_2_postgresql_connection,
                    ApplicationUserAccessRefreshTokenInsert1 {
                        application_user_id: incoming_.application_user_id,
                        application_user_device_id: incoming_.application_user_device_id.as_str(),
                        application_user_access_token_id,
                        application_user_access_refresh_token_obfuscation_value,
                        application_user_access_refresh_token_expires_at,
                        application_user_access_refresh_token_updated_at,
                    },
                )
                .await?;

                application_user_access_refresh_token_
            }
        };

// TODO  TRANZACTION
        let application_user_access_token_encrypted = FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(environment_configuration, &application_user_access_token)?;

        let application_user_access_refresh_token_encrypted = FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(environment_configuration, &application_user_access_refresh_token)?;

        let database_1_postgresql_connection_pool_ = database_1_postgresql_connection_pool.clone();

        let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                let database_1_postgresql_pooled_connection_ = database_1_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;

                let application_user_device = PostgresqlRepository::<ApplicationUserDevice>::create_1(
                    &*database_1_postgresql_pooled_connection_,
                    ApplicationUserDeviceInsert1 {
                        application_user_device_id: incoming_.application_user_device_id,
                        application_user_id: incoming_.application_user_id,
                    },
                )
                .await?;

                let database_2_postgresql_pooled_connection_ = database_2_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;

                PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                    &*database_2_postgresql_pooled_connection_,
                    &By4 {
                        application_user_id: application_user_device.application_user_id,
                        application_user_device_id: application_user_device.id.as_str(),
                    },
                )
                .await?;

                return Ok(());
            }
        );

        let outcoming = Outcoming {
            application_user_access_token_encrypted,
            application_user_access_refresh_token_encrypted,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}