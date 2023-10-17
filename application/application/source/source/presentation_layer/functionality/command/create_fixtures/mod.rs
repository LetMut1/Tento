#![allow(clippy::unused_unit)]

use application::application_layer::functionality::command_processor::CommandProcessor;
use application::application_layer::functionality::command_processor::CreateFixtures;
use application::infrastructure_layer::functionality::service::formatter::Format;
use application::infrastructure_layer::functionality::service::formatter::Formatter;

fn main() -> () {
    'a: loop {
        match CommandProcessor::<CreateFixtures>::process() {
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
