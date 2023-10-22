#![allow(clippy::unused_unit)]

use application::application_layer::functionality::command_processor::CommandProcessor;
use application::application_layer::functionality::command_processor::create_fixtures::CreateFixtures;
use application::infrastructure_layer::functionality::service::formatter::Formatter;
use application::infrastructure_layer::data::error_auditor::ErrorAuditor;

fn main() -> () {
    match CommandProcessor::<CreateFixtures>::process() {
        Ok(_) => {
            println!("\n Graceful shutdown.");
        }
        Err(error) => {
            println!(
                "{}",
                Formatter::<ErrorAuditor>::format(&error)
            );
        }
    }

    return ();
}
