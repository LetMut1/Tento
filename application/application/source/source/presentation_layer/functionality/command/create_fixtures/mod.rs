#![allow(clippy::unused_unit)]

use application::application_layer::functionality::command_processor::create_fixtures::CreateFixtures;
use application::infrastructure_layer::functionality::service::formatter::Format;
use application::infrastructure_layer::functionality::service::formatter::Formatter;

fn main() -> () {
    if let Err(error) = CreateFixtures::process() {
        println!(
            "{}",
            Formatter::prepare(&error)
        );
    }

    return ();
}
