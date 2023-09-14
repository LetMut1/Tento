use super::StringLiteral;
use super::String_;

pub trait Sealed {}

impl Sealed for StringLiteral {}

impl Sealed for String_ {}