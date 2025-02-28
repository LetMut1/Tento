use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_Value,
        infrastructure_layer::functionality::service::generator::{
            Generator as Generator_,
            NumberRow,
        },
    },
};
impl Generator<UserAuthorizationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
