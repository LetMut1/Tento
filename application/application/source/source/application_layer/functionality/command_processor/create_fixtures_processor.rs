use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::channel::AccessModifier;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;
use crate::domain_layer::data::entity::channel::Channel_BackgroundImagePath;
use crate::domain_layer::data::entity::channel::Channel_CoverImagePath;
use crate::domain_layer::data::entity::channel::Channel_Description;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_MarksQuantity;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_Orientation;
use crate::domain_layer::data::entity::channel::Channel_SubscribersQuantity;
use crate::domain_layer::data::entity::channel::Channel_ViewingQuantity;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier_;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user___postgresql_repository::Insert as ApplicationUserInsert;
use crate::infrastructure_layer::functionality::repository::application_user_device___postgresql_repository::Insert as ApplicationUserDeviceInsert;
use crate::infrastructure_layer::functionality::repository::channel___postgresql_repository::Insert as ChannelInsert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::By1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::By7;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::PostgresqlConnectionPoolNoTls;
use extern_crate::rand::thread_rng;
use extern_crate::rand::Rng;
use extern_crate::tokio::runtime::Builder;
use extern_crate::tokio_postgres::Config as PostgresqlConfiguration;
use std::str::FromStr;

pub struct CreateFixturesProcessor;

impl CreateFixturesProcessor {
    const STUB: &'static str = "s_t_u_b";
    const QUANTITY_OF_APPLICATION_USERS: u16 = 10_000;
    const QUANTITY_OF_CHANNELS: u8 = 5;
    const APPLICATION_USER__PASSWORD: &'static str = "passworD1";
    const APPLICATION_USER_DEVICE__ID_PART: &'static str = "device";
    const ASCII_CHARACTER_REGISTRY: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    pub fn process() -> Result<(), ErrorAuditor> {
        if let Environment::Production = ENVIRONMENT_CONFIGURATION.environment {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError {
                        message: "CreateFixturesProcessor should process only not in production environment.",
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                ),
            );
        }

        let runtime = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime_) => runtime_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        if let Err(mut error) = runtime.block_on(Self::create_fixtures()) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        return Ok(());
    }

    async fn create_fixtures<'a>() -> Result<(), ErrorAuditor> {
        let database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(ENVIRONMENT_CONFIGURATION.resource.postgresql.database_1_url.0) {
            Ok(database_1_postgresql_configuration_) => database_1_postgresql_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(
            &ENVIRONMENT_CONFIGURATION.environment,
            &database_1_postgresql_configuration,
        )
        .await
        {
            Ok(database_1_postgresql_connection_pool_) => database_1_postgresql_connection_pool_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_password = ApplicationUser_Password::new(Self::APPLICATION_USER__PASSWORD.to_string());

        let application_user_password_hash = match Encoder::<ApplicationUser_Password>::encode(&application_user_password) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        '_a: for _ in 1..=Self::QUANTITY_OF_APPLICATION_USERS {
            let mut application_user_nickname = ApplicationUser_Nickname::new(String::new());

            '_b: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Validator::<ApplicationUser_Nickname>::MAXIMUM_LENGTH) {
                let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                application_user_nickname = ApplicationUser_Nickname::new(
                    format!(
                        "{}{}",
                        application_user_nickname.get(),
                        character
                    ),
                );
            }

            if !Validator::<ApplicationUser_Nickname>::is_valid(&application_user_nickname) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Application_user nickname should be valid.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }

            let application_user_email = ApplicationUser_Email::new(
                format!(
                    "{}@fixture.com",
                    application_user_nickname.get()
                ),
            );

            let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&application_user_email) {
                Ok(is_valid_email_) => is_valid_email_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            if !is_valid_email {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Application_user email should be valid.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }

            if !Validator::<ApplicationUser_Password>::is_valid(
                &application_user_password,
                &application_user_email,
                &application_user_nickname,
            ) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Application_user_password should be valid.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }

            let by_1 = By1 {
                application_user_nickname: &application_user_nickname,
            };

            let application_user = match PostgresqlRepository::<ApplicationUser<'_>>::find_1(
                database_1_postgresql_connection,
                &by_1,
            )
            .await
            {
                Ok(application_user_) => application_user_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

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

                    let application_user__ = match PostgresqlRepository::<ApplicationUser<'_>>::create(
                        database_1_postgresql_connection,
                        application_user_insert,
                    )
                    .await
                    {
                        Ok(application_user_) => application_user_,
                        Err(mut error) => {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    };

                    application_user__
                }
            };

            let application_user_device_id = ApplicationUserDevice_Id::new(
                format!(
                    "{}_{}",
                    application_user_.get_nickname().get(),
                    Self::APPLICATION_USER_DEVICE__ID_PART
                ),
            );

            if !Validator::<ApplicationUserDevice_Id>::is_valid(&application_user_device_id) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Application_user_device id should be valid.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }

            let application_user_device_insert = ApplicationUserDeviceInsert {
                application_user_device_id,
                application_user_id: application_user_.get_id(),
            };

            if let Err(mut error) = PostgresqlRepository::<ApplicationUserDevice>::create(
                database_1_postgresql_connection,
                application_user_device_insert,
            )
            .await
            {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            };

            'b: for _ in 1..=Self::QUANTITY_OF_CHANNELS {
                let mut channel_name = Channel_Name::new(String::new());

                '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Validator::<Channel_Name>::MAXIMUM_LENGTH) {
                    let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                    channel_name = Channel_Name::new(
                        format!(
                            "{}{}",
                            channel_name.get(),
                            character
                        ),
                    );
                }

                if !Validator::<Channel_Name>::is_valid(&channel_name) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError {
                                message: "Channel name should be valid.",
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }

                let channel_linked_name = Channel_LinkedName::new(channel_name.get().to_string());

                if !Validator::<Channel_LinkedName>::is_valid(&channel_linked_name) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError {
                                message: "Channel linked name should be valid.",
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }

                let channel_description = if thread_rng().gen_range::<i8, _>(0..=1) == 1 {
                    let mut channel_description_ = Channel_Description::new(String::new());

                    '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Validator::<Channel_Description>::MAXIMUM_LENGTH) {
                        let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                        channel_description_ = Channel_Description::new(
                            format!(
                                "{}{}",
                                channel_description_.get(),
                                character
                            ),
                        );
                    }

                    if !Validator::<Channel_Description>::is_valid(&channel_description_) {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::LogicError {
                                    message: "Channel description should be valid.",
                                },
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            ),
                        );
                    }

                    Some(channel_description_)
                } else {
                    None
                };

                let channel_orientation = Channel_Orientation::new(
                    vec![
                        0, 1, 2,
                    ],
                );

                if !Validator::<Channel_Orientation>::is_valid(&channel_orientation) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError {
                                message: "Channel orientation email should be valid.",
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }

                let channel = match PostgresqlRepository::<Channel<'_>>::find_2(
                    database_1_postgresql_connection,
                    & By7 {
                        channel_name: &channel_name,
                    },
                )
                .await
                {
                    Ok(channel_) => channel_,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                match channel {
                    Some(_) => {
                        continue 'b;
                    }
                    None => {
                        let channel_insert = ChannelInsert {
                            channel_owner: application_user_.get_id(),
                            channel_name,
                            channel_linked_name,
                            channel_description,
                            channel_access_modifier: FormResolver::<AccessModifier>::from_representation(Channel_AccessModifier_::Open),
                            channel_visability_modifier: FormResolver::<Channel_VisabilityModifier>::from_representation(Channel_VisabilityModifier_::Public),
                            channel_orientation,
                            channel_cover_image_path: Some(Channel_CoverImagePath::new(Self::STUB.to_string())),
                            channel_background_image_path: Some(Channel_BackgroundImagePath::new(Self::STUB.to_string())),
                            channel_subscribers_quantity: Channel_SubscribersQuantity::new(0),
                            channel_marks_quantity: Channel_MarksQuantity::new(0),
                            channel_viewing_quantity: Channel_ViewingQuantity::new(0),
                        };

                        if let Err(mut error) = PostgresqlRepository::<Channel<'_>>::create(
                            database_1_postgresql_connection,
                            channel_insert,
                        )
                        .await
                        {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    }
                }
            }
        }

        return Ok(());
    }
}
