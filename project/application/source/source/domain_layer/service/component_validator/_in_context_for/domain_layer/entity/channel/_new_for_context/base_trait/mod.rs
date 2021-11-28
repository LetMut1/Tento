pub trait BaseTrait {
    const NAME_MAXIMUM_LENGTH: u8 = 75;
    const DESCRIPTION_MAXIMUM_LENGTH: u16 = 500;

    fn is_valid_name<'a>(
        name: &'a str
    ) -> bool {
        return name.chars().count() <= (Self::NAME_MAXIMUM_LENGTH as usize);
    }
    
    fn is_valid_description<'a>(
        description: &'a str
    ) -> bool {
        return description.chars().count() <= (Self::DESCRIPTION_MAXIMUM_LENGTH as usize);
    }
}