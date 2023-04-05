pub trait Getter<S, I, O> {
    fn get(subject: S) -> O;
}