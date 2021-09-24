use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;

pub trait BaseTrait {
    fn create<'outer_a>(
        json_access_web_token_id: &'outer_a str
    ) -> JsonAccessWebTokenBlackList<'_> {
        return JsonAccessWebTokenBlackList::new(json_access_web_token_id);
    }
}