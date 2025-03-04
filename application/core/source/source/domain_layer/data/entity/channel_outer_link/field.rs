use std::marker::PhantomData;
pub struct Alias(PhantomData<String>);
pub struct Address(PhantomData<String>);
pub struct CreatedAt(PhantomData<i64>);
