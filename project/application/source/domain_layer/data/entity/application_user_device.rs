pub struct ApplicationUserDevice {
    id: String,
    _application_user_id: i64
}

impl ApplicationUserDevice {
    pub fn new(
        id: String,
        application_user_id: i64
    ) -> Self {
        return Self {
            id,
            _application_user_id: application_user_id
        };
    }

    pub fn get_id<'a>(&'a self) -> &'a str {
        return self.id.as_str();
    }
}