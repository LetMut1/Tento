use {
    super::Validator,
    crate::domain_layer::data::entity::channel_publication1::ChannelPublication1_Text,
};
impl Validator<ChannelPublication1_Text> {
    pub fn is_valid<'a>(channel_publication1__text: Option<&'a str>) -> bool {
        return true;
    }
}
