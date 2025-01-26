mod application_layer;
mod domain_layer;
mod infrastructure_layer;
mod presentation_layer;
use self::infrastructure_layer::{
    data::aggregate_error::AggregateError,
    functionality::service::formatter::Formatter,
};
pub(crate) use self::infrastructure_layer::data::aggregate_error::{
    new_invalid_argument,
    new_logic_unreachable_state,
    new_logic_value_already_exist,
    new_logic,
    new_runtime,
    option_into_result_logic_out_of_range,
    option_into_result_logic_value_does_not_exist,
    option_return_result_logic_invalid_socket_address,
    option_return_result_logic_out_of_range,
    option_return_result_logic_unreachable_state,
    option_return_result_logic_value_does_not_exist,
    result_into_result_indefinite_argument,
    result_into_result_logic,
    result_into_result_runtime,
    result_return_result_indefinite_argument,
    result_return_result_logic,
    result_return_result_runtime,
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
