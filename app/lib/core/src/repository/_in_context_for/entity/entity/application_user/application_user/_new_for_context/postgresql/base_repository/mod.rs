use crate::diesel_component::schema::public::application_user as application_user_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::application_user::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::application_user::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user_schema::table).values(New::new(application_user)).execute(connection_manager.get_connection()) {   // TODO нужно ли обработать количество вернувшихся строк
            Ok(_) => { 
                return Ok(()); 
            },
            Err(error) => { 
                return Err(DieselErrorKind::new_any(error, None));
             }
        };
    }

    pub fn is_exist_by_nickanme(connection_manager: &'outer ConnectionManager, nickname: &'outer str) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(
            dsl::exists(application_user_schema::table.filter(application_user_schema::nickname.eq(nickname)))
        ).get_result::<bool>(connection_manager.get_connection()) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(is_exist) => { 
                return Ok(is_exist); 
            },
            Err(error) => {
                return Err(DieselErrorKind::new_any(error, None)); 
            }
        };
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer str) -> Result<bool, DieselErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        match diesel::select(
            dsl::exists(application_user_schema::table.filter(application_user_schema::email.eq(email)))
        ).get_result::<bool>(connection_manager.get_connection()) { // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            Ok(is_exist) => { 
                return Ok(is_exist); 
            },
            Err(error) => { 
                return Err(DieselErrorKind::new_any(error, None)); 
            }
        };
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer str) -> Result<Option<ApplicationUser<'vague>>, DieselErrorKind> {
        match application_user_schema::table.filter(application_user_schema::email.eq(email))
        .get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(existing) => {
                match existing {
                    Some(existing) => { 
                        return Ok(Some(ApplicationUser::new_from_model(existing))); 
                    },
                    None => {
                        return Ok(None); 
                    }
                };
            },
            Err(error) => { 
                return Err(DieselErrorKind::new_any(error, None)); 
            }
        };
    }

    pub fn get_by_id(connection_manager: &'outer ConnectionManager, id: &'outer UuidV4) -> Result<Option<ApplicationUser<'vague>>, DieselErrorKind> {
        match application_user_schema::table.filter(application_user_schema::id.eq(id.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(existing) => {
                match existing {
                    Some(existing) => { 
                        return Ok(Some(ApplicationUser::new_from_model(existing))); 
                    },
                    None => { 
                        return Ok(None); 
                    }
                };
            },
            Err(error) => { 
                return Err(DieselErrorKind::new_any(error, None)); 
            }
        };
    }
}