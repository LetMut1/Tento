use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;

impl Generator<ApplicationUserAuthorizationToken_Value> {
    pub fn generate() -> ApplicationUserAuthorizationToken_Value {
        return ApplicationUserAuthorizationToken_Value::new(Generator_::<NumberRow>::generate_6());
    }
}
