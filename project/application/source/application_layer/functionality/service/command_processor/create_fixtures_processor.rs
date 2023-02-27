use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_device__validator::ApplicationUserDevice_Validator;
use crate::domain_layer::functionality::service::channel__validator::Channel_Validator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::environment_configuration::ENVIRONMENT_CONFIGURATION_FILE_PATH;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::Insert as ApplicationUserInsert;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::ApplicationUserDevice_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_device__postgresql_repository::Insert as ApplicationUserDeviceInsert;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Insert as ChannelInsert;
use crate::infrastructure_layer::functionality::service::environment_configuration__creator::EnvironmentConfiguration_Creator;
use crate::infrastructure_layer::functionality::service::postgresql_connection_pool_creator::PostgresqlConnectionPoolCreator;
use extern_crate::rand::Rng;
use extern_crate::rand::thread_rng;
use extern_crate::tokio_postgres::NoTls;
use extern_crate::tokio::runtime::Builder;
use std::clone::Clone;

pub struct CreateFixturesProcessor;

impl CreateFixturesProcessor {
    const QUANTITY_OF_APPLICATION_USERS: u8 = 100;
    const QUANTITY_OF_CHANNELS: u8 = 15;
    const APPLICATION_USER__PASSWORD: &'static str = "passworD1";
    const APPLICATION_USER_DEVICE__ID: &'static str = "device";
    const ASCII_CHARACTER_REGISTRY: [char; 26] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z',
    ];


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
            .enable_all()
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

        if !ApplicationUser_Validator::is_valid_password(Self::APPLICATION_USER__PASSWORD) {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { message: "Application_user_password should be valid." },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        if !ApplicationUserDevice_Validator::is_valid_id(Self::APPLICATION_USER_DEVICE__ID) {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { message: "Application_user_device id should be valid." },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let application_user_password_hash = match ApplicationUser_PasswordHashResolver::create(Self::APPLICATION_USER__PASSWORD) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        '_a: for _i in 1..=Self::QUANTITY_OF_APPLICATION_USERS {
            let mut application_user_nickname = String::new();

            '_b: for _j in 1..=thread_rng().gen_range::<usize, _>(3..=ApplicationUser_Validator::APPLICATION_USER__NICKNAME_MAXIMUM_LENGTH) {
                let character = Self::ASCII_CHARACTER_REGISTRY[
                    thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())
                ];

                application_user_nickname = format!("{}{}", application_user_nickname.as_str(), character);
            }

            if !ApplicationUser_Validator::is_valid_nickname(application_user_nickname.as_str()) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Application_user nickname should be valid." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            let application_user_email = format!("{}@fixture.com", application_user_nickname.as_str());

            let is_valid_email = match ApplicationUser_Validator::is_valid_email(application_user_email.as_str()) {
                Ok(is_valid_email_) => is_valid_email_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            if !is_valid_email {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Application_user email should be valid." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            let application_user = match ApplicationUser_PostgresqlRepository::find_1(
                database_1_postgresql_connection, application_user_nickname.as_str()
            ).await {
                Ok(application_user_) => application_user_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            let application_user_ = match application_user {
                Some(application_user__) => application_user__,
                None => {
                    let application_user_insert = ApplicationUserInsert {
                        application_user_email,
                        application_user_nickname,
                        application_user_password_hash: application_user_password_hash.clone(),
                    };

                    let application_user__ = match ApplicationUser_PostgresqlRepository::create(database_1_postgresql_connection, application_user_insert).await {
                        Ok(application_user_) => application_user_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user__
                }
            };

            let application_user_device_insert = ApplicationUserDeviceInsert {
                application_user_device_id: format!("{}_{}", application_user_.get_nickname(), Self::APPLICATION_USER_DEVICE__ID),
                application_user_id: application_user_.get_id()
            };

            if let Err(mut error) = ApplicationUserDevice_PostgresqlRepository::create(database_1_postgresql_connection, application_user_device_insert).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            };

            'b: for _t in 1..=Self::QUANTITY_OF_CHANNELS {
                let mut channel_name = String::new();

                '_c: for _j in 1..=thread_rng().gen_range::<usize, _>(3..=Channel_Validator::CHANNEL__NAME_MAXIMUM_LENGTH) {
                    let character = Self::ASCII_CHARACTER_REGISTRY[
                        thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())
                    ];

                    channel_name = format!("{}{}", channel_name.as_str(), character);
                }

                if !Channel_Validator::is_valid_name(channel_name.as_str()) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError { message: "Channel name should be valid." },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }

                let channel_description = if thread_rng().gen_range::<i8, _>(0..=1) == 1 {
                    let mut channel_description_ = String::new();

                    '_c: for _j in 1..=thread_rng().gen_range::<usize, _>(3..=Channel_Validator::CHANNEL__DESCRIPTION_MAXIMUM_LENGTH) {
                        let character = Self::ASCII_CHARACTER_REGISTRY[
                            thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())
                        ];

                        channel_description_ = format!("{}{}", channel_description_.as_str(), character);
                    }

                    if !Channel_Validator::is_valid_description(channel_description_.as_str()) {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError { message: "Channel description should be valid." },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }

                    Some(channel_description_)
                } else {
                    None
                };

                let channel_orientation: Vec<i16> = vec![0, 1, 2];

                if !Channel_Validator::is_valid_orientation(&channel_orientation) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError { message: "Channel orientation email should be valid." },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }

                let channel = match Channel_PostgresqlRepository::find_2(
                    database_1_postgresql_connection, channel_name.as_str()
                ).await {
                    Ok(channel_) => channel_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                match channel {
                    Some(_) => {
                        continue 'b;
                    }
                    None => {
                        let channel_insert = ChannelInsert {
                            application_user_id: application_user_.get_id(),
                            channel_name,
                            channel_description,
                            channel_is_private: false,
                            channel_orientation,
                            channel_personalization_image_path: "personalization_image_path stab".to_string(),
                            channel_subscribers_quantity: 0,
                            channel_marks_quantity: 0,
                            channel_viewing_quantity: 0
                        };

                        if let Err(mut error) = Channel_PostgresqlRepository::create(
                            database_1_postgresql_connection, channel_insert
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }
            }
        }

        return Ok(());
    }
}