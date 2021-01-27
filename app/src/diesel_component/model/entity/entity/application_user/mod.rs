use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::ApplicationUser;
use diesel::Insertable;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user"]
pub struct New<'a> {    // TODO описать id Аттрибутами
    id: &'a Uuid,
    email: &'a String,
    nickname: &'a String,
    password_hash: &'a String
}

impl<'a> New<'a> {
    pub fn from_entity(application_user: &'a ApplicationUser<'a>) -> Self {
        return Self {
            id: application_user.get_id().get_value(),
            email: application_user.get_emal().get_value(),
            nickname: application_user.get_nickname().get_value(),
            password_hash: application_user.get_passord_hash().get_value()
        };
    }
}

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    email: String,
    nickname: String,
    password_hash: String
}

impl<'a> Existing{
    pub fn get_id(&'a self) -> &'a Uuid {
        return &self.id;
    }

    pub fn get_emal(&'a self) -> &'a String {
        return &self.email;
    }

    pub fn get_nickname(&'a self) -> &'a String {
        return &self.nickname;
    }

    pub fn get_password_hash(&'a self) -> &'a String {
        return &self.password_hash;
    }
}