pub struct Insert<'a> {
    application_user_id: i64,
    device_id: &'a str,
    value: String,
    wrong_enter_tries_quantity: u8
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_id: i64,
        device_id: &'a str,
        value: String,
        wrong_enter_tries_quantity: u8
    ) -> Self {
        return Self {
            application_user_id,
            device_id,
            value,
            wrong_enter_tries_quantity
        }
    }

    pub fn into_inner(
        self
    ) -> (i64, &'a str, String, u8) {
        return (
            self.application_user_id,
            self.device_id,
            self.value,
            self.wrong_enter_tries_quantity
        );
    }
}