pub struct ListOfMembers {
    value: String
}

impl ListOfMembers { // TODO Разбить на части для сериализации десиарлизации
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}