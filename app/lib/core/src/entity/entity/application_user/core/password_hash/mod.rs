use maybe_owned::MaybeOwned;

pub struct PasswordHash<'b> {
    value: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> PasswordHash<'b> {
    pub fn new(value: MaybeOwned<'b, String>) -> Self {     // TODO create Hash with Util HAher (Нужен ли он имеео здесь как свойство )
        return Self {
            value
        };
    }

    pub fn get_value(&'a self) -> &'a String {
        return &self.value;
    }
}