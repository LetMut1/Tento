pub trait Getter<'a, T> {
    fn get(&'a self) -> T;
}