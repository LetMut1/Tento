use std::marker::PhantomData;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>
}