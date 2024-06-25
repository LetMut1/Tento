use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
    },
    error::External,
};
impl Formatter<Auditor<External>> {
    pub fn format<'a>(external_auditor: &'a Auditor<External>) -> String {
        return Formatter_::<Auditor<External>>::format(external_auditor);
    }
}
