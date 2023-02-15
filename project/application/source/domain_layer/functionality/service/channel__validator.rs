pub struct Channel_Validator;

impl Channel_Validator {
    const CHANNEL__NAME_MAXIMUM_LENGTH: usize = 75;
    const CHANNEL__DESCRIPTION_MAXIMUM_LENGTH: usize = 500;

    pub fn is_valid_name<'a>(channel_name: &'a str) -> bool {
        return channel_name.chars().count() <= Self::CHANNEL__NAME_MAXIMUM_LENGTH;
    }

    pub fn is_valid_description<'a>(channel_description: &'a str) -> bool {
        return channel_description.chars().count() <= Self::CHANNEL__DESCRIPTION_MAXIMUM_LENGTH;
    }
}