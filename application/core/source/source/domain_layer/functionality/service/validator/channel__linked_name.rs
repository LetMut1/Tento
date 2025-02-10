use {
    super::Validator,
    crate::domain_layer::data::entity::channel::Channel_LinkedName,
};
impl Validator<Channel_LinkedName> {
    pub fn is_valid<'a>(channel__linked_name: &'a str) -> bool {        // TODO TODO TODO
        '_a: for r#char in channel__linked_name.chars() {
            if r#char.is_uppercase() {
                return false;
            }
        }
        return true;
    }
}
