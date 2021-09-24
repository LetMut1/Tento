use uuid::Uuid;

pub trait ValueGeneratorTrait {
    fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}