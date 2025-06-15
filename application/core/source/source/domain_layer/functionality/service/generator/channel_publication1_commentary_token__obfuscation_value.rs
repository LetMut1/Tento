use {
    super::Generator,
    crate::domain_layer::data::entity::channel_publication1_commentary_token::ChannelPublication1CommentaryToken_ObfuscationValue,
    rand::Rng,
};
impl Generator<ChannelPublication1CommentaryToken_ObfuscationValue> {
    pub fn generate() -> i64 {
        return rand::thread_rng().gen_range::<i64, _>(i64::MIN..=i64::MAX);
    }
}
