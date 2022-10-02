pub struct Insert {
    application_user_id: i64,
    value: String,
    wrong_enter_tries_quantity: u8,
    is_approved: bool,
}

impl<'a> Insert {
    pub fn new(
        application_user_id: i64,
        value: String,
        wrong_enter_tries_quantity: u8,
        is_approved: bool,
    ) -> Self {
        return Self {
            application_user_id,
            value,
            wrong_enter_tries_quantity,
            is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (i64, String, u8, bool) {
        return (
            self.application_user_id,
            self.value,
            self.wrong_enter_tries_quantity,
            self.is_approved
        );
    }
}