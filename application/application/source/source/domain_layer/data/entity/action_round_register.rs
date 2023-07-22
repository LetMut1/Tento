use std::borrow::Cow;

pub use self::Context as ActionRoundRegister_Context;
pub use self::CreatedAt as ActionRoundRegister_CreatedAt;
pub use self::Method as ActionRoundRegister_Method;
pub use self::Route as ActionRoundRegister_Route;
pub use self::StatusCode as ActionRoundRegister_StatusCode;

pub struct Route<'a>(pub Cow<'a, str>);

pub struct Method<'a>(pub Cow<'a, str>);

#[derive(Clone, Copy)]
pub struct StatusCode(pub i16);

pub struct Context(pub String);

pub struct CreatedAt(pub String);

pub struct ActionRoundRegister<'a> {
    route: Route<'a>,
    method: Method<'a>,
    status_code: StatusCode,
    context: Option<Context>,
    created_at: CreatedAt,
}
