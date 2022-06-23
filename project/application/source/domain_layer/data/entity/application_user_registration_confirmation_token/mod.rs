pub struct ApplicationUserRegistrationConfirmationToken<'a> {
    application_user_email: &'a str,
    value: String,
    wrong_enter_tries_quantity: u8,
    created_at: Option<String>
}

impl<'a> ApplicationUserRegistrationConfirmationToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u16 = 60 * 3;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 10;

    pub fn new(
        application_user_email: &'a str,
        value: String,
        wrong_enter_tries_quantity: u8,
        created_at: Option<String>
    ) -> Self {
        return Self {
            application_user_email,
            value,
            wrong_enter_tries_quantity,
            created_at
        };
    }
    
    pub fn get_application_user_email<'b>(
        &'b self
    ) -> &'a str {
        return self.application_user_email;
    }

    pub fn get_value<'b>(
        &'b self
    ) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(
        &'b self
    ) -> u8 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_created_at<'b>(
        &'b self
    ) -> &'b str {
       todo!() // TODO 
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: u8
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }
}