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
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By7;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert7;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool::PostgresqlConnectionPoolNoTls;
use rand::thread_rng;
use rand::Rng;
use std::str::FromStr;
use tokio::runtime::Builder;
use tokio_postgres::Config as PostgresqlConfiguration;
use super::CommandProcessor;

pub use crate::infrastructure_layer::data::control_type::CreateFixtures;

impl CommandProcessor<CreateFixtures> {
    const STUB: &'static str = "s_t_u_b";
    const QUANTITY_OF_APPLICATION_USERS: u16 = 10_000;
    const QUANTITY_OF_CHANNELS: u8 = 5;
    const APPLICATION_USER__PASSWORD: &'static str = "passworD1";
    const APPLICATION_USER_DEVICE__ID_PART: &'static str = "device";
    const ASCII_CHARACTER_REGISTRY: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    pub fn process() -> Result<(), ErrorAuditor_> {
        if let Environment::Production = ENVIRONMENT_CONFIGURATION.environment {
            return Err(
                ErrorAuditor_::new(
                    Error::Logic {
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
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

    async fn create_fixtures<'a>() -> Result<(), ErrorAuditor_> {
        let database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(ENVIRONMENT_CONFIGURATION.resource.postgresql.database_1_url.0) {
            Ok(database_1_postgresql_configuration_) => database_1_postgresql_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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

        let application_user_password = ApplicationUser_Password(Self::APPLICATION_USER__PASSWORD.to_string());

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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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
            let mut application_user_nickname = ApplicationUser_Nickname(String::new());

            '_b: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=ApplicationUser_Nickname::MAXIMUM_LENGTH) {
                let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                application_user_nickname = ApplicationUser_Nickname(
                    format!(
                        "{}{}",
                        application_user_nickname.0.as_str(),
                        character
                    ),
                );
            }

            if !Validator::<ApplicationUser_Nickname>::is_valid(&application_user_nickname) {
                return Err(
                    ErrorAuditor_::new(
                        Error::Logic {
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

            let application_user_email = ApplicationUser_Email(
                format!(
                    "{}@fixture.com",
                    application_user_nickname.0.as_str()
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
                    ErrorAuditor_::new(
                        Error::Logic {
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
                    ErrorAuditor_::new(
                        Error::Logic {
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
                    let application_user__ = match PostgresqlRepository::<ApplicationUser<'_>>::create(
                        database_1_postgresql_connection,
                        Insert1 {
                            application_user_email,
                            application_user_nickname,
                            application_user_password_hash: application_user_password_hash.clone(),
                        },
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

            let application_user_device_id = ApplicationUserDevice_Id(
                format!(
                    "{}_{}",
                    application_user_.nickname.0.as_str(),
                    Self::APPLICATION_USER_DEVICE__ID_PART
                ),
            );

            if !Validator::<ApplicationUserDevice_Id>::is_valid(&application_user_device_id) {
                return Err(
                    ErrorAuditor_::new(
                        Error::Logic {
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

            if let Err(mut error) = PostgresqlRepository::<ApplicationUserDevice>::create(
                database_1_postgresql_connection,
                Insert4 {
                    application_user_device_id,
                    application_user_id: application_user_.id,
                },
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
                let mut channel_name = Channel_Name(String::new());

                '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Name::MAXIMUM_LENGTH) {
                    let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                    channel_name = Channel_Name(
                        format!(
                            "{}{}",
                            channel_name.0.as_str(),
                            character
                        ),
                    );
                }

                if !Validator::<Channel_Name>::is_valid(&channel_name) {
                    return Err(
                        ErrorAuditor_::new(
                            Error::Logic {
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

                let channel_linked_name = Channel_LinkedName(channel_name.0.clone());

                if !Validator::<Channel_LinkedName>::is_valid(&channel_linked_name) {
                    return Err(
                        ErrorAuditor_::new(
                            Error::Logic {
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
                    let mut channel_description_ = Channel_Description(String::new());

                    '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Description::MAXIMUM_LENGTH) {
                        let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                        channel_description_ = Channel_Description(
                            format!(
                                "{}{}",
                                channel_description_.0.as_str(),
                                character
                            ),
                        );
                    }

                    if !Validator::<Channel_Description>::is_valid(&channel_description_) {
                        return Err(
                            ErrorAuditor_::new(
                                Error::Logic {
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

                let channel_orientation = Channel_Orientation(
                    vec![
                        0, 1, 2,
                    ],
                );

                if !Validator::<Channel_Orientation>::is_valid(&channel_orientation) {
                    return Err(
                        ErrorAuditor_::new(
                            Error::Logic {
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
                    &By7 {
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
                        if let Err(mut error) = PostgresqlRepository::<Channel<'_>>::create(
                            database_1_postgresql_connection,
                            Insert7 {
                                channel_owner: application_user_.id,
                                channel_name,
                                channel_linked_name,
                                channel_description,
                                channel_access_modifier: FormResolver::<AccessModifier>::from_representation(Channel_AccessModifier_::Open),
                                channel_visability_modifier: FormResolver::<Channel_VisabilityModifier>::from_representation(Channel_VisabilityModifier_::Public),
                                channel_orientation,
                                channel_cover_image_path: Some(Channel_CoverImagePath(Self::STUB.to_string())),
                                channel_background_image_path: Some(Channel_BackgroundImagePath(Self::STUB.to_string())),
                                channel_subscribers_quantity: Channel_SubscribersQuantity(0),
                                channel_marks_quantity: Channel_MarksQuantity(0),
                                channel_viewing_quantity: Channel_ViewingQuantity(0),
                            },
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
