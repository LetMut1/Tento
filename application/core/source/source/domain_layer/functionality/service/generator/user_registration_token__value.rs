use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user_registration_token::UserRegistrationToken_Value,
        infrastructure_layer::functionality::service::generator::{
            Generator as Generator_,
            NumberRow,
        },
    },
};
impl Generator<UserRegistrationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
