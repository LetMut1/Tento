pub struct ApplicationUserDirectMessage {
    id: Option<i64>,
    list_of_members: String
}

impl ApplicationUserDirectMessage {
    pub fn new(
        id: Option<i64>,
        list_of_members: String
    ) -> Self {
        return Self {
            id,
            list_of_members
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> Option<i64> {
        return self.id;
    }

    pub fn get_list_of_members<'a>(
        &'a self
    ) -> &'a str {
        return self.list_of_members.as_str();
    }
}