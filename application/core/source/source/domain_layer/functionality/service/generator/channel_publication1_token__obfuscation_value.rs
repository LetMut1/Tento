use {
    super::Generator,
    crate::domain_layer::data::entity::channel_publication1_token::ChannelPublication1Token_ObfuscationValue,
    rand::Rng,
};
impl Generator<ChannelPublication1Token_ObfuscationValue> {
    pub fn generate() -> i64 {
        return rand::thread_rng().gen_range::<i64, _>(i64::MIN..=i64::MAX);
    }
}
