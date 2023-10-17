use super::Generator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::number_row::NumberRow;

impl Generator<ApplicationUserRegistrationToken_Value> {
    pub fn generate() -> ApplicationUserRegistrationToken_Value {
        return ApplicationUserRegistrationToken_Value(Generator_::<NumberRow>::generate_6());
    }
}
