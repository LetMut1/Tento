extern crate application;

use application::application_layer::functionality::service::command_processor::create_fixtures_processor::CreateFixturesProcessor;
use application::infrastructure_layer::functionality::service::formatter::Format;
use application::infrastructure_layer::functionality::service::formatter::Formatter;

fn main() -> () {
    if let Err(error) = CreateFixturesProcessor::process() {
        println!("{}", Formatter::prepare(&error));
    }

    return ();
}