use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct ImagesPathes(PhantomData<Vec<String>>);
pub struct Text(PhantomData<Option<String>>);
pub struct CommentariesQuantity(PhantomData<u32>);
pub struct MarksQuantity(PhantomData<i64>);
pub struct ViewQuantity(PhantomData<i64>);
pub struct CreatedAt(PhantomData<i64>);
