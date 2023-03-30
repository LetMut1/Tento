use std::marker::PhantomData;

pub struct ActionRoundRegister {
    route: String,
    _route: PhantomData<Route>,

    method: String,
    _method: PhantomData<Method>,

    status_code: i16,
    _statuse_code: PhantomData<StatusCode>,

    context: Option<String>,
    _context: PhantomData<Context>,

    created_at: String,
    _created_at: PhantomData<CreatedAt>,
}

pub struct Route;

pub struct Method;

pub struct StatusCode;

pub struct Context;

pub struct CreatedAt;