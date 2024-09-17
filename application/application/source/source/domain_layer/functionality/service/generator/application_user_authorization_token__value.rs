use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value,
    infrastructure_layer::{
        functionality::service::generator::Generator as Generator_,
    },
};
use crate::infrastructure_layer::functionality::service::generator::number_row::NumberRow;
impl Generator<ApplicationUserAuthorizationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
