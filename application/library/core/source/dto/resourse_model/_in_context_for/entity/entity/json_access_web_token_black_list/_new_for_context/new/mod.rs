use crate::diesel_component::schema::public::json_access_web_token_black_list;
use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "json_access_web_token_black_list"]
pub struct New<'outer> {
    json_refresh_web_token_id: &'outer Uuid
}

impl<'outer> New<'outer> {
    pub fn new(json_access_web_token_black_list: &'outer JsonAccessWebTokenBlackList<'outer>) -> Self {
        return Self {
            json_refresh_web_token_id: json_access_web_token_black_list.get_json_refresh_web_token_id().get_value()
        };
    }
}