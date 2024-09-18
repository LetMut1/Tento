use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
};
use application::application_layer::functionality::command_processor::CommandProcessor;
use application::application_layer::functionality::command_processor::run_server::RunServer;
use application::application_layer::functionality::command_processor::remove_incomplite_state::RemoveIncompliteState;
use application::application_layer::functionality::command_processor::create_fixtures::CreateFixtures;
use clap::{
    command,
    Command,
};
use formatter::Formatter;
// The type is 'Result<(), ()>' but not '()' to return a success/error exit code but not only success exit code.
fn main() -> Result<(), ()> {
    if let Result::Err(aggregate_error) = Processor::process() {
        println!("{}", Formatter::<AggregateError>::format(&aggregate_error));
        return Result::Err(());
    }
    return Result::Ok(());
}
struct Processor;
impl Processor {
    fn process() -> Result<(), AggregateError> {
        const RUN_SERVER: &'static str = "run_server";
        const CREATE_FIXTURES: &'static str = "create_fixtures";
        const REMOVE_INCOMPLITE_STATE: &'static str = "remove_incomplite_state";
        let arg_matches = command!()
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new(RUN_SERVER))
            .subcommand(Command::new(CREATE_FIXTURES))
            .get_matches();
        let subcommand_arg_matches = match arg_matches.subcommand() {
            Option::Some(subcommand_arg_matches_) => subcommand_arg_matches_,
            Option::None => {
                return Result::Err(
                    AggregateError::new_logic_(
                        Common::UnreachableState,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };
        return match subcommand_arg_matches {
            (RUN_SERVER, _) => CommandProcessor::<RunServer>::process(),
            (CREATE_FIXTURES, _) => CommandProcessor::<CreateFixtures>::process(),
            (REMOVE_INCOMPLITE_STATE, _) => CommandProcessor::<RemoveIncompliteState>::process(),
            _ => {
                Result::Err(
                    AggregateError::new_logic(
                        "Unexpected subcommand.".into(),
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                )
            }
        };
    }
}
