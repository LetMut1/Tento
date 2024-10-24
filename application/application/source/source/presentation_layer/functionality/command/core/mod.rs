use application::{
    CreateFixtures,
    RemoveIncompliteState,
    RunServer,
    CommandProcessor,
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    Formatter,
};
use clap::{
    Command,
    Arg,
};
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
        const COMMAND_RUN_SERVER: &'static str = "run_server";
        const COMMAND_CREATE_FIXTURES: &'static str = "create_fixtures";
        const COMMAND_REMOVE_INCOMPLITE_STATE: &'static str = "remove_incomplite_state";
        const ARGUMENT_ENVIRONMENT_FILE_DIRECTORY: &'static str = "environment_configuration_file_directory";
        let arg_matches = clap::command!()
            .arg_required_else_help(true)
            .arg(
                Arg::new(ARGUMENT_ENVIRONMENT_FILE_DIRECTORY)
                .required(true)
                .long(ARGUMENT_ENVIRONMENT_FILE_DIRECTORY),
            )
            .subcommand_required(true)
            .subcommand(Command::new(COMMAND_RUN_SERVER))
            .subcommand(Command::new(COMMAND_CREATE_FIXTURES))
            .subcommand(Command::new(COMMAND_REMOVE_INCOMPLITE_STATE))
            .get_matches();
        let environment_configuration_file_directory = arg_matches.get_one::<String>(ARGUMENT_ENVIRONMENT_FILE_DIRECTORY).into_logic_unreachable_state(
            Backtrace::new(
                line!(),
                file!(),
            )
        )?;
        let subcommand_arg_matches = arg_matches.subcommand().into_logic_unreachable_state(
            Backtrace::new(
                line!(),
                file!(),
            )
        )?;
        return match subcommand_arg_matches {
            (COMMAND_RUN_SERVER, _) => CommandProcessor::<RunServer>::process(environment_configuration_file_directory.as_str()),
            (COMMAND_CREATE_FIXTURES, _) => CommandProcessor::<CreateFixtures>::process(environment_configuration_file_directory.as_str()),
            (COMMAND_REMOVE_INCOMPLITE_STATE, _) => CommandProcessor::<RemoveIncompliteState>::process(),
            _ => {
                Result::Err(
                    AggregateError::new_logic_(
                        Common::UnreachableState,
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
