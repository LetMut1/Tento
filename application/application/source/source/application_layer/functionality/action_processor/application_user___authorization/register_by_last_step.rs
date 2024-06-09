use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By1;
use crate::infrastructure_layer::data::control_type::TokioBlockingTask;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By2;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By5;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert1;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert2;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert4;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update10;
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

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterByLastStep;

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
            incoming_.application_user_email.as_str(),
            incoming_.application_user_nickname.as_str(),
        ) {
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

        if !Validator::<ApplicationUser_Nickname>::is_valid(incoming_.application_user_nickname.as_str()) {
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

        if !Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user_email.as_str())? {
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

        if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(incoming_.application_user_registration_token_value.as_str())? {
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

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
            database_1_postgresql_connection,
            &By1 {
                application_user_nickname: incoming_.application_user_nickname.as_str(),
            },
        )
        .await?
        {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NicknameAlreadyExist)));
        }

        if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            database_1_postgresql_connection,
            &By2 {
                application_user_email: incoming_.application_user_email.as_str(),
            },
        )
        .await?
        {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_EmailAlreadyExist)));
        }

        let by_5 = By5 {
            application_user_email: incoming_.application_user_email.as_str(),
            application_user_device_id: incoming_.application_user_device_id.as_str(),
        };

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_2(
            database_2_postgresql_connection,
            &by_5,
        )
        .await?
        {
            Some(application_user_registration_token_) => application_user_registration_token_,
            None => {
                return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound)));
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token.expires_at) {
            PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_5,
            )
            .await?;

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired)));
        }

        if !application_user_registration_token.is_approved {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_IsNotApproved)));
        }

        if application_user_registration_token.value != incoming_.application_user_registration_token_value {
            application_user_registration_token.wrong_enter_tries_quantity = application_user_registration_token.wrong_enter_tries_quantity
            .checked_add(1)
            .convert_out_of_range(Backtrace::new(line!(), file!()))?;

            if application_user_registration_token.wrong_enter_tries_quantity < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                PostgresqlRepository::<ApplicationUserRegistrationToken>::update_4(
                    database_2_postgresql_connection,
                    &Update10 {
                        application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                    },
                    &by_5,
                )
                .await?;
            } else {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                    database_2_postgresql_connection,
                    &by_5,
                )
                .await?;
            }

            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_WrongValue)));
        }

        let join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
            move || -> _ {
                return Encoder::<ApplicationUser_Password>::encode(incoming_.application_user_password.as_str());
            }
        );

        let application_user = PostgresqlRepository::<ApplicationUser<'_>>::create(
            database_1_postgresql_connection,
            Insert1 {
                application_user_email: incoming_.application_user_email,
                application_user_nickname: incoming_.application_user_nickname,
                application_user_password_hash: join_handle.await.convert(Backtrace::new(line!(), file!()))??,
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
        let application_user_access_refresh_token = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create(
            database_2_postgresql_connection,
            Insert2 {
                application_user_id: application_user.id,
                application_user_device_id: incoming_.application_user_device_id.as_str(),
                application_user_access_token_id: application_user_access_token.id.as_str(),
                application_user_access_refresh_token_obfuscation_value: Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate(),
                application_user_access_refresh_token_expires_at: Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?,
                application_user_access_refresh_token_updated_at: Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate(),
            },
        )
        .await?;

        let application_user_access_token_encrypted = FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(environment_configuration, &application_user_access_token)?;

        let application_user_access_refresh_token_encrypted = FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(environment_configuration, &application_user_access_refresh_token)?;

        let database_1_postgresql_connection_pool_ = database_1_postgresql_connection_pool.clone();

        let database_2_postgresql_connection_pool_ = database_2_postgresql_connection_pool.clone();

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                let database_1_postgresql_pooled_connection_ = database_1_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;

                let application_user_device = PostgresqlRepository::<ApplicationUserDevice>::create(
                    &*database_1_postgresql_pooled_connection_,
                    Insert4 {
                        application_user_device_id: incoming_.application_user_device_id,
                        application_user_id: application_user.id,
                    },
                )
                .await?;

                let database_2_postgresql_pooled_connection_ = database_2_postgresql_connection_pool_.get().await.convert(Backtrace::new(line!(), file!()))?;

                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                    &*database_2_postgresql_pooled_connection_,
                    &By5 {
                        application_user_email: application_user.email.as_str(),
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
