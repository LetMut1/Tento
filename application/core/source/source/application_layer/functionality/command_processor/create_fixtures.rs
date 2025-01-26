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
            aggregate_error::AggregateError,
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
use dedicated::void::Void;
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
pub use crate::infrastructure_layer::data::environment_configuration::create_fixtures::CreateFixtures;
impl CommandProcessor<CreateFixtures> {
    pub fn process<'a>(environment_configuration_file_path: &'a str) -> Result<(), AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<CreateFixtures>>::load_from_file(environment_configuration_file_path)?;
        let runtime = Self::initialize_runtime()?;
        runtime.block_on(Self::create_fixtures(&environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_runtime() -> Result<Runtime, AggregateError> {
        return crate::result_into_result_runtime!(
            Builder::new_current_thread().enable_all().build()
        );
    }
    fn create_fixtures<'a>(environment_configuration: &'a EnvironmentConfiguration<CreateFixtures>) -> impl Future<Output = Result<(), AggregateError>> + Send + Capture<&'a Void> {
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
        return async move {
            let postgresql_connection_pool_database_1 = Creator::<PostgresqlConnectionPool>::create(
                &environment_configuration.subject.resource.postgresql.database_1,
                NoTls,
            )
            .await?;
            let user__password = APPLICATION_USER__PASSWORD.to_string();
            let user__password_hash = Encoder::<User_Password>::encode(user__password.as_str())?;
            let postgresql_database_1_client = crate::result_return_result_runtime!(
                postgresql_connection_pool_database_1.get().await
            );
            '_a: for _ in 1..=QUANTITY_OF_APPLICATION_USERS {
                let mut user__nickname = String::new();
                '_b: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=User_Nickname::MAXIMUM_LENGTH) {
                    let character = ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..ASCII_CHARACTER_REGISTRY.len())];
                    user__nickname = format!("{}{}", user__nickname.as_str(), character);
                }
                if !Validator::<User_Nickname>::is_valid(user__nickname.as_str()) {
                    return Result::Err(crate::new_invalid_argument!());
                }
                let user__email = format!("{}@fixture.com", user__nickname.as_str());
                if !Validator::<User_Email>::is_valid(user__email.as_str())? {
                    return Result::Err(crate::new_invalid_argument!());
                }
                if !Validator::<User_Password>::is_valid(
                    user__password.as_str(),
                    user__email.as_str(),
                    user__nickname.as_str(),
                ) {
                    return Result::Err(crate::new_invalid_argument!());
                }
                let user = match Repository::<Postgresql<User<'_>>>::find_1(
                    &postgresql_database_1_client,
                    UserBy1 {
                        user__nickname: user__nickname.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(user_) => user_,
                    Option::None => {
                        Repository::<Postgresql<User<'_>>>::create_1(
                            &postgresql_database_1_client,
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
                    APPLICATION_USER_DEVICE__ID_PART
                );
                if !Validator::<UserDevice_Id>::is_valid(&user_device__id) {
                    return Result::Err(crate::new_invalid_argument!());
                }
                Repository::<Postgresql<UserDevice>>::create_1(
                    &postgresql_database_1_client,
                    UserDeviceInsert1 {
                        user_device__id,
                        user__id: user.id,
                    },
                )
                .await?;
                'b: for _ in 1..=QUANTITY_OF_CHANNELS {
                    let mut channel__name = String::new();
                    '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Name::MAXIMUM_LENGTH) {
                        let character = ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..ASCII_CHARACTER_REGISTRY.len())];
                        channel__name = format!("{}{}", channel__name.as_str(), character,);
                    }
                    if !Validator::<Channel_Name>::is_valid(channel__name.as_str()) {
                        return Result::Err(crate::new_invalid_argument!());
                    }
                    let channel__linked_name = channel__name.clone();
                    if !Validator::<Channel_LinkedName>::is_valid(channel__linked_name.as_str()) {
                        return Result::Err(crate::new_invalid_argument!());
                    }
                    let channel__description = if thread_rng().gen_range::<i8, _>(0..=1) == 1 {
                        let mut channel__description_ = String::new();
                        '_c: for _ in 1..=thread_rng().gen_range::<usize, _>(1..=Channel_Description::MAXIMUM_LENGTH) {
                            let character = ASCII_CHARACTER_REGISTRY[thread_rng().gen_range::<usize, _>(0..ASCII_CHARACTER_REGISTRY.len())];
                            channel__description_ = format!("{}{}", channel__description_.as_str(), character,);
                        }
                        if !Validator::<Channel_Description>::is_valid(channel__description_.as_str()) {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        Option::Some(channel__description_)
                    } else {
                        Option::None
                    };
                    let channel__orientation: Vec<i16> = vec![
                        0, 1, 2,
                    ];
                    if !Validator::<Channel_Orientation>::is_valid(channel__orientation.as_slice()) {
                        return Result::Err(crate::new_invalid_argument!());
                    }
                    let channel = Repository::<Postgresql<Channel<'_>>>::find_2(
                        &postgresql_database_1_client,
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
                                &postgresql_database_1_client,
                                ChannelInsert1 {
                                    channel__owner: user.id,
                                    channel__name,
                                    channel__linked_name,
                                    channel__description,
                                    channel__access_modifier: const { Channel_AccessModifier::Open as i16 },
                                    channel__visability_modifier: const { Channel_VisabilityModifier::Public as i16 },
                                    channel__orientation,
                                    channel__cover_image_path: Option::Some(STUB.to_string()),
                                    channel__background_image_path: Option::Some(STUB.to_string()),
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
