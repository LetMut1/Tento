use {
    super::Validator,
    crate::domain_layer::data::entity::channel::Channel_Name,
};
impl Validator<Channel_Name> {
    pub fn is_valid<'a>(channel__name: &'a str) -> bool {
        return channel__name.chars().count() <= Channel_Name::MAXIMUM_LENGTH && !channel__name.is_empty();
    }
}
