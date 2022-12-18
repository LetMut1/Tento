use extern_crate::serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

// It is more correct to use Enam in the context of a unified server response.
// The Struct is used, but not Enam, because there are problems in synchronizing the Enum serialization
// and deserialization rules of the used encoding protocol on both sides.
//
// pub enum Data<S>
// {
//     Empty,
//     Filled {
//         data: S
//     }
// }

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Data<S>
{
    data: Option<S>
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
impl<S> Data<S>
where
    S: Serialize
{
    pub fn new(data: Option<S>) -> Self {
        return Self {
            data
        };
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
impl<S> Data<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new(data: Option<S>) -> Self {
        return Self {
            data
        };
    }
}