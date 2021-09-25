use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;

pub trait BaseTrait {
    fn create<'a>(
        json_access_web_token_id: &'a str
    ) -> JsonAccessWebTokenBlackList<'_> {
        return JsonAccessWebTokenBlackList::new(json_access_web_token_id);
    }
}