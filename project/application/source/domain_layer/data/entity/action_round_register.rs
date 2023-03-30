use std::marker::PhantomData;

pub struct ActionRoundRegister {
    route: String,
    route_: PhantomData<Route>,

    method: String,
    method_: PhantomData<Method>,

    status_code: i16,
    statuse_code_: PhantomData<StatusCode>,

    context: Option<String>,
    context_: PhantomData<Context>,

    created_at: String,
    created_at_: PhantomData<CreatedAt>,
}

pub struct Route;

pub struct Method;

pub struct StatusCode;

pub struct Context;

pub struct CreatedAt;