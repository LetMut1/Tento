use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct ImagesPathes(PhantomData<Vec<String>>);
pub struct Text(PhantomData<Option<String>>);
pub struct CommentariesQuantity(PhantomData<u32>);
pub struct MarksQuantity(PhantomData<u32>);
pub struct ViewQuantity(PhantomData<u32>);
pub struct CreatedAt(PhantomData<i64>);
