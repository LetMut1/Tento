pub struct Formatter;

pub trait Format<T> {
    fn prepare<'a>(subject: &'a T) -> String;
}