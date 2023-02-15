use crate::infrastructure_layer::functionality::service::number_row_generator::NumberRowGenerator;

pub struct ApplicationUserResetPasswordToken_ValueGenerator;

impl ApplicationUserResetPasswordToken_ValueGenerator {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}