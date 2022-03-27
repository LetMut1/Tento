use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub enum Data<S>
where
    S: Serialize
{
    Empty,
    Filled {
        data: S
    }
}

// TODO TOD OTODO  https://github.com/rust-lang/rust-clippy/issues/1689 