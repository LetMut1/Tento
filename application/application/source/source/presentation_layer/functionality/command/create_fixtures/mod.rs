#![allow(clippy::unused_unit)]

use application::application_layer::functionality::command_processor::create_fixtures::CreateFixtures;
use application::infrastructure_layer::functionality::service::formatter::Format;
use application::infrastructure_layer::functionality::service::formatter::Formatter;
use application::infrastructure_layer::data::error_auditor::ErrorAuditor;

fn main() -> () {
    if let Err(error) = CreateFixtures::process() {
        println!(
            "{}",
            Formatter::<ErrorAuditor>::prepare(&error)
        );
    }

    return ();
}
