use crate::infrastructure_layer::data::{
    alternative_workflow::{
        AlternativeWorkflow,
        OptionConverter,
    },
    auditor::Backtrace,
};
pub struct Counter<T> {
    value: T,
    step_size: T,
}
impl Counter<i16> {
    const CLASSIC_STEP_SIZE: i16 = 1;
    pub fn new(value: i16, step_size: i16) -> Self {
        return Self {
            value,
            step_size,
        };
    }
    pub fn new_classic() -> Self {
        return Self {
            value: 0,
            step_size: Self::CLASSIC_STEP_SIZE,
        };
    }
    pub fn get_next_value<'a>(&'a mut self) -> Result<i16, AlternativeWorkflow> {
        self.value = self.value.checked_add(self.step_size).into_internal_error_logic_out_of_range(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(self.value);
    }
}
