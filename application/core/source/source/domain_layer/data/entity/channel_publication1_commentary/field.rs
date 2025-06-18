use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct Text(PhantomData<String>);
pub struct MarksQuantity(PhantomData<u32>);
pub struct CreatedAt(PhantomData<i64>);
