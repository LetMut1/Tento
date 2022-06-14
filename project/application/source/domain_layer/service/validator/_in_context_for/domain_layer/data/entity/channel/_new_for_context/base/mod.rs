pub struct Base;

impl Base {
    const NAME_MAXIMUM_LENGTH: u8 = 75;
    const DESCRIPTION_MAXIMUM_LENGTH: u16 = 500;

    pub fn is_valid_name<'a>(
        name: &'a str
    ) -> bool {
        return name.chars().count() <= (Self::NAME_MAXIMUM_LENGTH as usize);
    }
    
    pub fn is_valid_description<'a>(
        description: &'a str
    ) -> bool {
        return description.chars().count() <= (Self::DESCRIPTION_MAXIMUM_LENGTH as usize);
    }
}