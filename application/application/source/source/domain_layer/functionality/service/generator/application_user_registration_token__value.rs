use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value,
    infrastructure_layer::functionality::service::generator::{
        number_row::NumberRow,
        Generator as Generator_,
    },
};
impl Generator<ApplicationUserRegistrationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
