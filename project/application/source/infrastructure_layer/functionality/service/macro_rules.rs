macro_rules! r#enum {
    ($($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)?) => {
        const _: () = {
            $(
                let _ = |e: $enum| -> () {
                    match e {
                        $enum :: $enum_variant => (),
                        _ => (),
                    }
                };
            )*

            ()
        };

        #[derive(Debug)]
        enum Name {
            $($enum_variant $({ $($enum_variant_field: $enum_variant_field_type,)* })?,)*
        }
    }
}