#![allow(clippy::unused_unit)]

use application::application_layer::functionality::command_processor::create_fixtures::CreateFixtures;
use application::infrastructure_layer::functionality::service::formatter::Format;
use application::infrastructure_layer::functionality::service::formatter::Formatter;

fn main() -> () {
    'a: loop {
        match CreateFixtures::process() {
            Ok(_) => {
                println!("\n Graceful shutdown.");

                break 'a;
            }
            Err(error) => {
                println!(
                    "{}",
                    Formatter::prepare(&error)
                );

                continue 'a;
            }
        }
    }

    return ();
}
