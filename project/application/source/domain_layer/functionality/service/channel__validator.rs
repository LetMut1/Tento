#[allow(non_camel_case_types)]
pub struct Channel_Validator;

impl Channel_Validator {
    #[allow(non_upper_case_globals)]
    const CHANNEL__NAME_MAXIMUM_LENGTH: usize = 75;
    #[allow(non_upper_case_globals)]
    const CHANNEL__DESCRIPTION_MAXIMUM_LENGTH: usize = 500;

    pub fn is_valid_name<'a>(channel_name: &'a str) -> bool {
        return channel_name.chars().count() <= Self::CHANNEL__NAME_MAXIMUM_LENGTH;
    }

    pub fn is_valid_description<'a>(channel_description: &'a str) -> bool {
        return channel_description.chars().count() <= Self::CHANNEL__DESCRIPTION_MAXIMUM_LENGTH;
    }
}