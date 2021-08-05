use std::clone::Clone;

#[derive(Clone)]
pub struct OwnerApplicationUserAdministratorId {
    value: i64
}

impl OwnerApplicationUserAdministratorId {
    pub fn new(value: i64) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> i64 {
        return self.value;
    }
}

// TODO TODO TODO Когда будет гтов application_user_administrator, удалить этту структуру и использовать созданну.