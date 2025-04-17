#[cfg(not(feature = "token_value_666666"))]
use crate::infrastructure_layer::functionality::service::generator::{
    Generator as Generator_,
    NumberRow,
};
use {
    super::Generator,
    crate::domain_layer::data::entity::user_registration_token::UserRegistrationToken_Value,
};
impl Generator<UserRegistrationToken_Value> {
    pub fn generate() -> String {
        #[cfg(feature = "token_value_666666")]
        {
            return "666666".to_string();
        }
        #[cfg(not(feature = "token_value_666666"))]
        {
            return Generator_::<NumberRow>::generate_6();
        }
    }
}
