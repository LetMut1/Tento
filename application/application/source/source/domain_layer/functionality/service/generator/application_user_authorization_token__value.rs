use super::Generator;
use crate::{
    domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_Value,
    infrastructure_layer::functionality::service::generator::{
        number_row::NumberRow,
        Generator as Generator_,
    },
};
impl Generator<UserAuthorizationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
