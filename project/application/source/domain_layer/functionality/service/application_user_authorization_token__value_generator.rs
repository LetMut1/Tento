use crate::infrastructure_layer::functionality::service::number_row_generator::NumberRowGenerator;

pub struct ApplicationUserAuthorizationToken_ValueGenerator;

impl ApplicationUserAuthorizationToken_ValueGenerator {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}