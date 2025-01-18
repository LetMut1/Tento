mod application_layer;
mod domain_layer;
mod infrastructure_layer;
mod presentation_layer;
use self::infrastructure_layer::{
    data::aggregate_error::AggregateError,
    functionality::service::formatter::Formatter,
};
use self::presentation_layer::functionality::command::Command;
// The type is 'Result<(), ()>' but not '()' to return a success/error exit code but not only success exit code.
fn main() -> Result<(), ()> {
    if let Result::Err(aggregate_error) = Command::process() {
        println!("{}", Formatter::<AggregateError>::format(&aggregate_error));
        return Result::Err(());
    }
    return Result::Ok(());
}
