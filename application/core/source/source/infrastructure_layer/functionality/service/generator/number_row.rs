use {
    super::Generator,
    rand::{
        distributions::Uniform,
        prelude::Distribution,
    },
};
pub struct NumberRow;
impl Generator<NumberRow> {
    pub fn generate_6() -> String {
        let mut thread_rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive::<u8, _>(0, 9);
        return format!(
            "{}{}{}{}{}{}",
            uniform.sample(&mut thread_rng),
            uniform.sample(&mut thread_rng),
            uniform.sample(&mut thread_rng),
            uniform.sample(&mut thread_rng),
            uniform.sample(&mut thread_rng),
            uniform.sample(&mut thread_rng),
        );
    }
}