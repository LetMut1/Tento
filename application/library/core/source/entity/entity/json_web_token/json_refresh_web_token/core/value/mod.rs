use std::clone::Clone;

pub struct Value {
    value: String
}

impl<'this> Value {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}


// TODO эту структуру переделать на JAWTserializationForm. Хранить сам токен в бд (кеше) в открытом виде. Значение JAWTserializationForm хешировать для хранения. Слать токен в захешированном виде 
