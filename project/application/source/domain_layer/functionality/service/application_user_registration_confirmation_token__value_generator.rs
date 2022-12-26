use crate::infrastructure_layer::functionality::service::number_row_generator::NumberRowGenerator;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationConfirmationToken_ValueGenerator;

impl ApplicationUserRegistrationConfirmationToken_ValueGenerator {       // TODO Как-то нужно связать с Validator? (6 цифр)
    pub fn generate(
    ) -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}