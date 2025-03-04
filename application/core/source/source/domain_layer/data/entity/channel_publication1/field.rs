
use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct ImagesPathes(PhantomData<Vec<String>>);
pub struct Text(PhantomData<Option<String>>);
pub struct CreatedAt(PhantomData<i64>);
