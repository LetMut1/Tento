use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::entity::entity::apllication_user_log_in_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer mut ConnectionManager, application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().set_ex::<String, String, ()>(
                RedisStorageKeyResolver::get_first_for_application_user_log_in_token_base_repository(
                    application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                ), 
                serde_json::to_string(&Common::new(application_user_log_in_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
                (DateExpirationCreator::QUANTITY_OF_MINUTES * 60) as usize
            )?
        );
    }

    pub fn delete(
        connection_manager: &'outer mut ConnectionManager, application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().del::<String, ()>(
                RedisStorageKeyResolver::get_first_for_application_user_log_in_token_base_repository(
                    application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                )
            )?
        );
    }

    pub fn update(
        connection_manager: &'outer mut ConnectionManager, application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(Self::create(connection_manager, application_user_log_in_token)?);
    }

    pub fn get_by_application_user_id_and_device_id(
        connection_manager: &'outer mut ConnectionManager, application_user_id: &'outer UuidV4, device_id: &'outer UuidV4,
    ) -> Result<Option<ApplicationUserLogInToken<'vague>>, ResourceErrorKind> {
        // match connection_manager.get_connection().get::<String, String>(
        //     RedisStorageKeyResolver::get_first_for_application_user_log_in_token_base_repository(application_user_id, device_id)
        // ) 
        // {
        //                     //     Ok(value) => {
        //                     //     return Ok(Some(ApplicationUserLogInToken::new_from_model(serde_json::from_str::<'_, Common>(s).unwrap())))
        //                     //     },
        //                     //     Err(error) => {
        //                     //         println!("error3");
        //                     //         println!("{:?}", error);
        //                     //     }
        //                     // }
        // }
        return Ok(None);
    }
}