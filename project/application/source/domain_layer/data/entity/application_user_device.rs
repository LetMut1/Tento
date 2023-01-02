pub struct ApplicationUserDevice {
    id: String,
    application_user_id: i64
}

impl ApplicationUserDevice {

    pub fn new(
        id: String,
        application_user_id: i64
    ) -> Self {
        return Self {
            id,
            application_user_id
        };
    }
}