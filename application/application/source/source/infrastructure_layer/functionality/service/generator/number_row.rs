use super::Generator;
use crate::infrastructure_layer::data::control_type::NumberRow;
impl Generator<NumberRow> {
    pub fn generate_6() -> String {
        return "666666".to_string(); // TODOD format!("{}{}{}{}{}{}", rand....)
    }
}
