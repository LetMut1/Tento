use uuid::Uuid;

pub struct ValueGenerator;

impl ValueGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}