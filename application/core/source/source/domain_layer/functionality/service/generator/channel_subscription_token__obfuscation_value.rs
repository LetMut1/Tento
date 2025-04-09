use {
    super::Generator,
    crate::domain_layer::data::entity::channel_subscription_token::ChannelSubscriptionToken_ObfuscationValue,
    rand::Rng,
};
impl Generator<ChannelSubscriptionToken_ObfuscationValue> {
    pub fn generate() -> i64 {
        return rand::thread_rng().gen_range::<i64, _>(i64::MIN..=i64::MAX);
    }
}
