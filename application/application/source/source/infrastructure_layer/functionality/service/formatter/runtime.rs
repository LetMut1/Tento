use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::aggregate_error::Runtime;
impl Formatter<Runtime> {
    pub fn format<'a>(runtime: &'a Runtime) -> String {
        return Formatter_::<Runtime>::format(runtime);
    }
}
