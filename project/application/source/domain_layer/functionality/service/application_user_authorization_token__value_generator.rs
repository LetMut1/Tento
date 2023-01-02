use crate::infrastructure_layer::functionality::service::number_row_generator::NumberRowGenerator;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAuthorizationToken_ValueGenerator;

impl ApplicationUserAuthorizationToken_ValueGenerator {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}