pub mod postgresql;
use std::marker::PhantomData;
pub struct Repository<E> {
    _entity: PhantomData<E>,
}
