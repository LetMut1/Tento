use {
    super::Validator,
    crate::domain_layer::data::entity::channel::Channel_Orientation,
};
impl Validator<Channel_Orientation> {
    pub fn is_valid<'a>(_channel__orientation: &'a [i16]) -> bool {
        return true; // TODO;
    }
}
