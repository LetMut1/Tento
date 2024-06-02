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
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By1;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By7;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert1;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert4;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert7;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool::PostgresqlConnectionPoolNoTls;
use rand::thread_rng;
use rand::Rng;
use crate::infrastructure_layer::functionality::service::loader::Loader;
use tokio::runtime::Builder;
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

    pub fn process() -> Result<(), Auditor<Error>> {
        let environment_configuration = Self::initialize_environment()?;

        if let Environment::Production = environment_configuration.environment {
            return Err(
                Auditor::<Error>::new(
                    Error::Logic {
                        message: "Should process only not in production environment.",
                    },
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        Self::run_runtime(&environment_configuration)?;

        return Ok(());
    }

    fn initialize_environment() -> Result<EnvironmentConfiguration, Auditor<Error>> {
        let environment_configuration_file_path = format!(
            "{}/environment_configuration",
            std::env::var("CARGO_MANIFEST_DIR").convert(Backtrace::new(line!(), file!()))?.as_str(),
        );

        return Ok(Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_path.as_str())?);
    }

    fn run_runtime<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .convert(Backtrace::new(line!(), file!()))?
            .block_on(Self::create_fixtures(environment_configuration))?;

        return Ok(());
    }

    async fn create_fixtures<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        let database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPoolNoTls>::create_database_1(
            environment_configuration,
        )
        .await?;

        let application_user_password = Self::APPLICATION_USER__PASSWORD.to_string();

        let application_user_password_hash = Encoder::<ApplicationUser_Password>::encode(application_user_password.as_str())?;

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        '_a: for _ in 1..=Self::QUANTITY_OF_APPLICATION_USERS {
            let mut application_user_nickname = String::new();

            '_b: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=ApplicationUser_Nickname::MAXIMUM_LENGTH) {
                let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                application_user_nickname = format!(
                    "{}{}",
                    application_user_nickname.as_str(),
                    character
                );
            }

            if !Validator::<ApplicationUser_Nickname>::is_valid(application_user_nickname.as_str()) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Application_user nickname should be valid.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            let application_user_email = format!(
                "{}@fixture.com",
                application_user_nickname.as_str()
            );

            if !Validator::<ApplicationUser_Email>::is_valid(application_user_email.as_str())? {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Application_user email should be valid.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            if !Validator::<ApplicationUser_Password>::is_valid(
                application_user_password.as_str(),
                application_user_email.as_str(),
                application_user_nickname.as_str(),
            ) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Application_user_password should be valid.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            let by_1 = By1 {
                application_user_nickname: application_user_nickname.as_str(),
            };

            let application_user = match PostgresqlRepository::<ApplicationUser<'_>>::find_1(
                database_1_postgresql_connection,
                &by_1,
            )
            .await?
            {
                Some(application_user_) => application_user_,
                None => {
                    PostgresqlRepository::<ApplicationUser<'_>>::create(
                        database_1_postgresql_connection,
                        Insert1 {
                            application_user_email,
                            application_user_nickname,
                            application_user_password_hash: application_user_password_hash.clone(),
                        },
                    )
                    .await?
                }
            };

            let application_user_device_id = format!(
                "{}_{}",
                application_user.nickname.as_ref(),
                Self::APPLICATION_USER_DEVICE__ID_PART
            );

            if !Validator::<ApplicationUserDevice_Id>::is_valid(&application_user_device_id) {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Application_user_device id should be valid.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }

            PostgresqlRepository::<ApplicationUserDevice>::create(
                database_1_postgresql_connection,
                Insert4 {
                    application_user_device_id,
                    application_user_id: application_user.id,
                },
            )
            .await?;

            'b: for _ in 1..=Self::QUANTITY_OF_CHANNELS {
                let mut channel_name = String::new();

                '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Name::MAXIMUM_LENGTH) {
                    let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                    channel_name = format!(
                        "{}{}",
                        channel_name.as_str(),
                        character,
                    );
                }

                if !Validator::<Channel_Name>::is_valid(channel_name.as_str()) {
                    return Err(
                        Auditor::<Error>::new(
                            Error::Logic {
                                message: "Channel name should be valid.",
                            },
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }

                let channel_linked_name = channel_name.clone();

                if !Validator::<Channel_LinkedName>::is_valid(channel_linked_name.as_str()) {
                    return Err(
                        Auditor::<Error>::new(
                            Error::Logic {
                                message: "Channel linked name should be valid.",
                            },
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }

                let channel_description = if thread_rng().gen_range::<i8, _>(0..=1) == 1 {
                    let mut channel_description_ = String::new();

                    '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Description::MAXIMUM_LENGTH) {
                        let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];

                        channel_description_ = format!(
                            "{}{}",
                            channel_description_.as_str(),
                            character,
                        );
                    }

                    if !Validator::<Channel_Description>::is_valid(channel_description_.as_str()) {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Logic {
                                    message: "Channel description should be valid.",
                                },
                                Backtrace::new(
                                    line!(),
                                    file!(),
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
                        Auditor::<Error>::new(
                            Error::Logic {
                                message: "Channel orientation email should be valid.",
                            },
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }

                let channel = PostgresqlRepository::<Channel<'_>>::find_2(
                    database_1_postgresql_connection,
                    &By7 {
                        channel_name: channel_name.as_str(),
                    },
                )
                .await?;

                match channel {
                    Some(_) => {
                        continue 'b;
                    }
                    None => {
                        PostgresqlRepository::<Channel<'_>>::create(
                            database_1_postgresql_connection,
                            Insert7 {
                                channel_owner: application_user.id,
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
                        .await?;
                    }
                }
            }
        }

        return Ok(());
    }
}
