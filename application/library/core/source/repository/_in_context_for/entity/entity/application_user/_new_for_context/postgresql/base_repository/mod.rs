use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::insert::Insert;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::select::Select;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::update::Update;
use crate::diesel_component::schema::public::application_user as application_user_schema;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::data_transfer_object::resource_model::_new_for_context::update_resolver::_in_context_for::_in_context_for::entity::entity::application_user::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer_a, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer_a ConnectionManager, application_user: &'outer_a ApplicationUser<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        diesel::insert_into(application_user_schema::table).values(Insert::new(application_user))
        .execute(connection_manager.get_connection())?;  // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer_a ConnectionManager, application_user: &'outer_a ApplicationUser<'outer_a>, update_resolver: UpdateResolver
    ) -> Result<(), ResourceErrorKind> {
        diesel::update(application_user_schema::table.filter(application_user_schema::id.eq(application_user.get_id().get_value())))
        .set(&Update::new(application_user, update_resolver)).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_nickanme(connection_manager: &'outer_a ConnectionManager, nickname: &'outer_a Nickname) -> Result<bool, ResourceErrorKind> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::nickname.eq(nickname.get_value()))))
            .get_result::<bool>(connection_manager.get_connection())?
        );// TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn is_exist_by_email(connection_manager: &'outer_a ConnectionManager, email: &'outer_a Email) -> Result<bool, ResourceErrorKind> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))))
            .get_result::<bool>(connection_manager.get_connection())?
        );      // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_email(connection_manager: &'outer_a ConnectionManager, email: &'outer_a Email) -> Result<Option<ApplicationUser<'vague>>, ResourceErrorKind> {
        if let Some(existing) = application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }

    pub fn get_by_id(connection_manager: &'outer_a ConnectionManager, id: &'outer_a UuidV4) -> Result<Option<ApplicationUser<'vague>>, ResourceErrorKind> {
        if let Some(existing) = application_user_schema::table.filter(application_user_schema::id.eq(id.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }
}

// TODO // TODO // TODO // TODO // TODO // TODO // TODO
// delete this after writing same query for another entity (Exampe of multyrow Select)
// pub fn get_by_application_user_id(
//     connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a UuidV4
// ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, ResourceErrorKind> {
//     let existing_registry = json_refresh_web_token_schema::table
//     .filter(json_refresh_web_token_schema::application_user_id.eq(application_user_id.get_value()))
//     .get_results::<Existing>(connection_manager.get_connection())?;
    
//     if !existing_registry.is_empty() {
//         return Ok(
//             Some(
//                 existing_registry.into_iter().map(
//                     |existing: Existing| -> JsonRefreshWebToken<'vague> { 
//                         return JsonRefreshWebToken::new_from_model(existing); 
//                     }
//                 ).collect::<Vec<JsonRefreshWebToken<'vague>>>()
//             )
//         ); 
//     }
    
//     return Ok(None); 
// }
// TODO // TODO // TODO // TODO // TODO // TODO // TODO