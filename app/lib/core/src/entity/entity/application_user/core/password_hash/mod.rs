use maybe_owned::MaybeOwned;

pub struct PasswordHash<'outer> {
    value: MaybeOwned<'outer, String>
}

impl<'this, 'outer: 'this> PasswordHash<'outer> {
    pub fn new(value: MaybeOwned<'outer, String>) -> Self {     // TODO create Hash with Util HAher (Нужен ли он имеео здесь как свойство )
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this String {
        return &self.value;
    }
}