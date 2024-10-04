use cbindgen::{
    Builder,
    Language,
};
use std::{
    env::var,
    error::Error,
    path::Path,
};
use uuid::Uuid;
#[allow(clippy::panic)]
fn main() -> () {
    if let Result::Err(error) = Processor::process() {
        panic!(
            "{}",
            error
        );
    }
    return ();
}
struct Processor;
impl Processor {
    fn process() -> Result<(), Box<dyn Error + 'static>> {
        Self::create_rerun_instruction()?;
        Self::create_c_bindings()?;
        return Result::Ok(());
    }
    fn create_rerun_instruction() -> Result<(), Box<dyn Error + 'static>> {
        let file_name = Uuid::new_v4().to_string();
        let file_path = format!(
            "{}/{}.txt",
            var("OUT_DIR")?.as_str(),
            file_name.as_str(),
        );
        cargo_emit::rerun_if_changed!(file_path.as_str());
        return Result::Ok(());
    }
    fn create_c_bindings() -> Result<(), Box<dyn Error + 'static>> {
        let crate_path = var("CARGO_MANIFEST_DIR")?;
        let crate_path_ = Path::new(crate_path.as_str());
        let file_path = format!(
            "{}/../target_/c_bindings.h",
            crate_path.as_str(),
        );
        let file_path_ = Path::new(file_path.as_str());
        Builder::new().with_crate(crate_path_).with_language(Language::C).generate()?.write_to_file(file_path_);
        return Result::Ok(());
    }
}
