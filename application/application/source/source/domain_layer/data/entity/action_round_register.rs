use std::borrow::Cow;

pub use self::Context as ActionRoundRegister_Context;
pub use self::CreatedAt as ActionRoundRegister_CreatedAt;
pub use self::Method as ActionRoundRegister_Method;
pub use self::Route as ActionRoundRegister_Route;
pub use self::StatusCode as ActionRoundRegister_StatusCode;

pub struct Route<'a>(Cow<'a, str>);

impl<'a> Route<'a> {
    pub fn new(inner: Cow<'a, str>) -> Self {
        return Self(inner);
    }

    pub fn get<'b>(&'b self) -> &'b str {
        return self.0.as_ref();
    }
}

pub struct Method<'a>(Cow<'a, str>);

impl<'a> Method<'a> {
    pub fn new(inner: Cow<'a, str>) -> Self {
        return Self(inner);
    }

    pub fn get<'b>(&'b self) -> &'b str {
        return self.0.as_ref();
    }
}

#[derive(Clone, Copy)]
pub struct StatusCode(i16);

impl StatusCode {
    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

pub struct Context(String);

impl Context {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct CreatedAt;

pub struct ActionRoundRegister<'a> {
    route: Route<'a>,
    method: Method<'a>,
    status_code: StatusCode,
    context: Option<Context>,
    created_at: CreatedAt,
}
