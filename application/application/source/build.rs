use cargo_emit::rerun_if_changed;
use std::{
    env::var,
    error::Error,
};
use uuid::Uuid;
fn main() -> () {
    if let Result::Err(error) = Processor::process() {
        panic!("{}", error);
    }
    return ();
}
struct Processor;
impl Processor {
    fn process() -> Result<(), Box<dyn Error + 'static>> {
        Self::create_rerun_instruction()?;
        return Result::Ok(());
    }
    // It is necessary that the build-script be run on each compilation command,
    // so we specify in the instructions that the Cargo watch for a non-existent
    // file with `cargo:rerun-if-changed=non_existent_file` command.
    fn create_rerun_instruction() -> Result<(), Box<dyn Error + 'static>> {
        let file_name = Uuid::new_v4().to_string();
        let file_path = format!("{}/{}.txt", var("OUT_DIR")?.as_str(), file_name.as_str(),);
        rerun_if_changed!(file_path.as_str());
        return Result::Ok(());
    }
}
