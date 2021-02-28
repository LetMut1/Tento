use std::error::Error;

#[derive(Debug)]
pub struct Context<E> 
where 
    E: Error
{
    previous: Option<E>,
    message: String
}

impl<'this, E> Context<E> where E: Error
{
    pub fn new(previous: Option<E>, message: String) -> Self {
        return Self {
            previous,
            message
        };
    }

    pub fn get_previous(&'this self) -> &'this Option<E> {
        return &self.previous;
    }

    pub fn get_message(&'this self) -> &'this String {
        return &self.message;
    }
}