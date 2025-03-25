use {
    super::Validator,
    crate::domain_layer::data::entity::channel_publication1_commentary::ChannelPublication1Commentary_Id,
};
impl Validator<ChannelPublication1Commentary_Id> {
    pub fn is_valid<'a>(channel_publication1_commentary__id: i64) -> bool {
        return channel_publication1_commentary__id >= 0;
    }
}
