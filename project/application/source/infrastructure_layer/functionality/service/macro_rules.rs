macro_rules! create_enum {
    ($(($e:ident :: $var:ident $(, { $($field:ident : $typ:ty),* $(,)? })?)),* $(,)? ) => {
        const _: () = { {$(
            let _ = |e: $e| {
                match e {
                    $e :: $var => (),
                    _ => (),
                }
            };
        )*}};

        #[derive(Debug)]
        enum Name {
            $($var $({ $(
                $field: $typ,
            )*})?,)*
        }
    }
}
