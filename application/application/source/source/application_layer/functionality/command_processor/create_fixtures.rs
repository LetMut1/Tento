use super::CommandProcessor;
use crate::{
    domain_layer::{
        data::entity::{
            channel::{
                Channel,
                Channel_AccessModifier,
                Channel_Description,
                Channel_LinkedName,
                Channel_Name,
                Channel_Orientation,
                Channel_VisabilityModifier,
            },
            user::{
                User,
                User_Email,
                User_Nickname,
                User_Password,
            },
            user_device::{
                UserDevice,
                UserDevice_Id,
            },
        },
        functionality::service::{
            encoder::Encoder,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::{
            repository::{
                postgresql::{
                    ChannelBy2,
                    ChannelInsert1,
                    Postgresql,
                    UserBy1,
                    UserDeviceInsert1,
                    UserInsert1,
                },
                Repository,
            },
            service::{
                creator::{
                    Creator,
                    PostgresqlConnectionPool,
                },
                loader::Loader,
                resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
};
use dedicated_crate::void::Void;
use rand::{
    thread_rng,
    Rng,
};
use std::future::Future;
use tokio::runtime::{
    Builder,
    Runtime,
};
use tokio_postgres::NoTls;
pub struct CreateFixtures;
impl CommandProcessor<CreateFixtures> {
    const APPLICATION_USER_DEVICE__ID_PART: &'static str = "device";
    const APPLICATION_USER__PASSWORD: &'static str = "passworD1";
    const ASCII_CHARACTER_REGISTRY: [char; 26] = [
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
        'q',
        'r',
        's',
        't',
        'u',
        'v',
        'w',
        'x',
        'y',
        'z',
    ];
    const QUANTITY_OF_APPLICATION_USERS: u16 = 10_000;
    const QUANTITY_OF_CHANNELS: u8 = 5;
    const STUB: &'static str = "s_t_u_b";
    pub fn process<'a>(environment_configuration_file_directory: &'a str) -> Result<(), AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_directory)?;
        let runtime = Self::initialize_runtime()?;
        runtime.block_on(Self::create_fixtures(&environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_runtime() -> Result<Runtime, AggregateError> {
        return Builder::new_current_thread().enable_all().build().into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    fn create_fixtures<'a>(environment_configuration: &'a EnvironmentConfiguration) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let database_1_postgresql_connection_pool = Creator::<PostgresqlConnectionPool>::create(
                &environment_configuration.resource.postgresql.database_1,
                NoTls,
            )
            .await?;
            let user__password = Self::APPLICATION_USER__PASSWORD.to_string();
            let user__password_hash = Encoder::<User_Password>::encode(user__password.as_str())?;
            let database_1_postgresql_client = database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            '_a: for _ in 1..=Self::QUANTITY_OF_APPLICATION_USERS {
                let mut user__nickname = String::new();
                '_b: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=User_Nickname::MAXIMUM_LENGTH) {
                    let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];
                    user__nickname = format!("{}{}", user__nickname.as_str(), character);
                }
                if !Validator::<User_Nickname>::is_valid(user__nickname.as_str()) {
                    return Result::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                let user__email = format!("{}@fixture.com", user__nickname.as_str());
                if !Validator::<User_Email>::is_valid(user__email.as_str())? {
                    return Result::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                if !Validator::<User_Password>::is_valid(
                    user__password.as_str(),
                    user__email.as_str(),
                    user__nickname.as_str(),
                ) {
                    return Result::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                let user = match Repository::<Postgresql<User<'_>>>::find_1(
                    &database_1_postgresql_client,
                    UserBy1 {
                        user__nickname: user__nickname.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(user_) => user_,
                    Option::None => {
                        Repository::<Postgresql<User<'_>>>::create_1(
                            &database_1_postgresql_client,
                            UserInsert1 {
                                user__email,
                                user__nickname,
                                user__password_hash: user__password_hash.clone(),
                                user__created_at: Resolver::<UnixTime>::get_now(),
                            },
                        )
                        .await?
                    }
                };
                let user_device__id = format!(
                    "{}_{}",
                    user.nickname.as_ref(),
                    Self::APPLICATION_USER_DEVICE__ID_PART
                );
                if !Validator::<UserDevice_Id>::is_valid(&user_device__id) {
                    return Result::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                Repository::<Postgresql<UserDevice>>::create_1(
                    &database_1_postgresql_client,
                    UserDeviceInsert1 {
                        user_device__id,
                        user__id: user.id,
                    },
                )
                .await?;
                'b: for _ in 1..=Self::QUANTITY_OF_CHANNELS {
                    let mut channel__name = String::new();
                    '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Name::MAXIMUM_LENGTH) {
                        let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];
                        channel__name = format!("{}{}", channel__name.as_str(), character,);
                    }
                    if !Validator::<Channel_Name>::is_valid(channel__name.as_str()) {
                        return Result::Err(
                            AggregateError::new_invalid_argument(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                    let channel__linked_name = channel__name.clone();
                    if !Validator::<Channel_LinkedName>::is_valid(channel__linked_name.as_str()) {
                        return Result::Err(
                            AggregateError::new_invalid_argument(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                    let channel__description = if thread_rng().gen_range::<i8, _>(0..=1) == 1 {
                        let mut channel__description_ = String::new();
                        '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Description::MAXIMUM_LENGTH) {
                            let character = Self::ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..Self::ASCII_CHARACTER_REGISTRY.len())];
                            channel__description_ = format!("{}{}", channel__description_.as_str(), character,);
                        }
                        if !Validator::<Channel_Description>::is_valid(channel__description_.as_str()) {
                            return Result::Err(
                                AggregateError::new_invalid_argument(
                                    Backtrace::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            );
                        }
                        Option::Some(channel__description_)
                    } else {
                        Option::None
                    };
                    let channel__orientation: Vec<i16> = vec![
                        0, 1, 2,
                    ];
                    if !Validator::<Channel_Orientation>::is_valid(channel__orientation.as_slice()) {
                        return Result::Err(
                            AggregateError::new_invalid_argument(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                    let channel = Repository::<Postgresql<Channel<'_>>>::find_2(
                        &database_1_postgresql_client,
                        ChannelBy2 {
                            channel__name: channel__name.as_str(),
                        },
                    )
                    .await?;
                    match channel {
                        Option::Some(_) => {
                            continue 'b;
                        }
                        Option::None => {
                            Repository::<Postgresql<Channel<'_>>>::create_1(
                                &database_1_postgresql_client,
                                ChannelInsert1 {
                                    channel__owner: user.id,
                                    channel__name,
                                    channel__linked_name,
                                    channel__description,
                                    channel__access_modifier: const { Channel_AccessModifier::Open as i16 },
                                    channel__visability_modifier: const { Channel_VisabilityModifier::Public as i16 },
                                    channel__orientation,
                                    channel__cover_image_path: Option::Some(Self::STUB.to_string()),
                                    channel__background_image_path: Option::Some(Self::STUB.to_string()),
                                    channel__subscribers_quantity: 0,
                                    channel__marks_quantity: 0,
                                    channel__viewing_quantity: 0,
                                    channel__created_at: Resolver::<UnixTime>::get_now(),
                                },
                            )
                            .await?;
                        }
                    }
                }
            }
            return Result::Ok(());
        };
    }
}
