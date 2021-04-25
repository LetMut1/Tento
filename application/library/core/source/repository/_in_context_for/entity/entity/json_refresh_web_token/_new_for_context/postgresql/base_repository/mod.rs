use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::entity::entity::json_refresh_web_token::_new_context_for::date_expiration_creator::DateExpirationCreator;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer mut ConnectionManager, json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        // TODO // TODO // TODO Класть в храгилище девайс-айди.
        return Ok(
            connection_manager.get_connection().set_ex::<String, String, ()>(
                RedisStorageKeyResolver::get_first_for_json_refresh_web_token_base_repository(
                    json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
                ), 
                serde_json::to_string(&Common::new(json_refresh_web_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
                (DateExpirationCreator::QUANTITY_OF_MINUTES * 60) as usize
            )?
        );
    }

    pub fn update(
        connection_manager: &'outer mut ConnectionManager, json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Self::create(connection_manager, json_refresh_web_token);
    }


    pub fn delete(
        connection_manager: &'outer mut ConnectionManager, json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        // TODO // TODO // TODO УДалять из хранилища девайс айди
        return Ok(
            connection_manager.get_connection().del::<String, ()>(
                RedisStorageKeyResolver::get_first_for_json_refresh_web_token_base_repository(
                    json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
                )
            )?
        );
    }

    pub fn get_by_application_user_id(  // TODO rename?     // TODO удаление массовое сделать внутри метода Репозетория
        connection_manager: &'outer mut ConnectionManager, 
        application_user_id: &'outer UuidV4
    ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, ResourceErrorKind> {
        // let existing_registry = json_refresh_web_token_schema::table
        // .filter(json_refresh_web_token_schema::application_user_id.eq(application_user_id.get_value()))
        // .get_results::<Existing>(connection_manager.get_connection())?;
        
        // if !existing_registry.is_empty() {
        //     return Ok(
        //         Some(
        //             existing_registry.into_iter().map(
        //                 |existing: Existing| -> JsonRefreshWebToken<'vague> { 
        //                     return JsonRefreshWebToken::new_from_model(existing); 
        //                 }
        //             ).collect::<Vec<JsonRefreshWebToken<'vague>>>()
        //         )
        //     ); 
        // }
        
        return Ok(None); 
    }

    pub fn get_by_application_user_id_and_application_user_log_in_token_device_id(
        connection_manager: &'outer mut ConnectionManager, 
        application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4,
    ) -> Result<Option<JsonRefreshWebToken<'vague>>, ResourceErrorKind> {
        match connection_manager.get_connection().get::<String, Option<String>>(
            RedisStorageKeyResolver::get_first_for_json_refresh_web_token_base_repository(application_user_id, application_user_log_in_token_device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(JsonRefreshWebToken::new_from_model(
                    serde_json::from_str::<'_, Common>(json_encoded_common.as_str()).unwrap()   // TODO error 
                ).unwrap()));    // TODO error 
            },
            None => {
                return Ok(None);
            }
        }
    }
}
// TODO какой срок ескпирации для ключа, хранящего все значения девайсов 


// TODO При переходе на РЕдис:
// хранить рефреш-токен по ключу "апплтикэйшн-юзер-айди + логин-девайс_айди".
// для Выхода со всех устройств храниь все ДевайсыАди по ключу "АппликэйшнЮзерАйди"
// ставить срок экспирации кеша, равный сроку экспирации токена ( так же и в БлэкЛист)
// // TODO // TODO обратить внимение на Транзакции, в которых используется методы ( то есть, пройти по РекуэстХэндлерам для Authentication)
// TODO УЗнать подробно про Транзакции в Редисе !