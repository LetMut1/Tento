use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct Email(PhantomData<String>);
impl Email {
    pub const MAXIMUM_LENGTH: usize = 320;
    pub const REGULAR_EXPRESSION: &'static str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
}
pub struct Nickname(PhantomData<String>);
impl Nickname {
    pub const MAXIMUM_LENGTH: usize = 55;
}
pub struct Password(PhantomData<String>);
impl Password {
    pub const MAXIMUM_LENGTH: usize = 65;
    pub const MINIMUM_LENGTH: usize = 7; // TODO Нужна ли максимальная длина? // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
}
pub struct PasswordHash(PhantomData<String>);
pub struct CreatedAt(PhantomData<i64>);
