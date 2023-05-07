pub trait GetterDELETE<S, I, O> {
    fn get(subject: S) -> O;
}

pub trait Getter<'a, T> {
    fn get(&'a self) -> T;
}