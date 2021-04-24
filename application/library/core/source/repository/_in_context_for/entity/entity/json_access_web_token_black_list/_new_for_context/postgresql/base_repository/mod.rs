use crate::diesel_component::schema::public::json_access_web_token_black_list as json_access_web_token_black_list_schema;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer ConnectionManager, json_access_web_token_black_list: &'outer JsonAccessWebTokenBlackList<'outer>
    ) -> Result<(), ResourceErrorKind> {
        diesel::insert_into(json_access_web_token_black_list_schema::table).values(New::new(json_access_web_token_black_list))
        .execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_json_access_token_id(connection_manager: &'outer ConnectionManager, json_access_web_token_id: &'outer UuidV4) -> Result<bool, ResourceErrorKind> {
        return Ok(
            diesel::select(
                dsl::exists(
                    json_access_web_token_black_list_schema::table
                    .filter(json_access_web_token_black_list_schema::json_access_web_token_id.eq(json_access_web_token_id.get_value()))
                )
            )
            .get_result::<bool>(connection_manager.get_connection())?
        );
    }
}

// При переходе на Редис  делать срок экспирации// TODO // TODO обратить внимение на Транзакции, в которых используется методы ( то есть, пройти по РекуэстХэндлерам для Authentication)