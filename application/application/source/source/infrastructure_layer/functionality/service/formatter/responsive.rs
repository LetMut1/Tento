use super::{
    Format,
    Formatter,
};
use crate::infrastructure_layer::data::server_workflow_error::Responsive;
use aggregate_error::{
    Context,
    IndefiniteArgument,
    InvalidArgument,
};
use formatter::Formatter as Formatter_;
use std::marker::PhantomData;
impl Format<Responsive> for Formatter<Responsive> {
    fn format<'a>(subject: &'a Responsive) -> String {
        return match *subject {
            Responsive::IndefiniteArgument {
                ref indefinite_argument_context,
            } => Formatter_::<Context<PhantomData<IndefiniteArgument>>>::format(indefinite_argument_context),
            Responsive::InvalidArgument {
                invalid_argument: _,
            } => Formatter_::<PhantomData<InvalidArgument>>::format(),
        };
    }
}
