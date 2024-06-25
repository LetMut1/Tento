use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
    },
    error::Internal,
};
impl Formatter<Auditor<Internal>> {
    pub fn format<'a>(internal_auditor: &'a Auditor<Internal>) -> String {
        return Formatter_::<Auditor<Internal>>::format(internal_auditor);
    }
}
