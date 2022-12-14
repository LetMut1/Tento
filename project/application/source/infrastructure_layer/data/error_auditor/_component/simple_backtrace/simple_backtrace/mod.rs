use super::_component::backtrace_part::BacktracePart;

#[derive(Debug)]
pub struct SimpleBacktrace {
    backtrace_part_registry: Vec<BacktracePart>
}

impl SimpleBacktrace {
    pub fn new(
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part]
        };
    }

    pub fn add<'a>(
        &'a mut self,
        backtrace_part: BacktracePart
    ) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }
}