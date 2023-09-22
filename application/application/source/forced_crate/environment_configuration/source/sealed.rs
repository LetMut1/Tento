pub struct String_(pub String);

pub struct StringLiteral(pub &'static str);

pub trait Sealed {}

impl Sealed for StringLiteral {}

impl Sealed for String_ {}