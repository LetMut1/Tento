use std::marker::PhantomData;
pub struct Text(PhantomData<Option<String>>);
pub struct MarksQuantity(PhantomData<i64>);
pub struct CreatedAt(PhantomData<i64>);
