use {
    super::Validator,
    crate::domain_layer::data::entity::channel_publication1::ChannelPublication1_Id,
};
impl Validator<ChannelPublication1_Id> {
    pub fn is_valid(channel_publication1__id: i64) -> bool {
        return channel_publication1__id >= 0;
    }
}
