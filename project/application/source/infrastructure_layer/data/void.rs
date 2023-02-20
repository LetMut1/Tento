use extern_crate::serde::Serialize;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[serde(crate = "extern_crate::serde")]
pub enum Void {}