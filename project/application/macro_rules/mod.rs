#[macro_export]
macro_rules! r#enum {
    ($visability:vis enum $enum_name:ident { $($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)? }) => {
        const _: () = {
            $(
                let _ = |r#enum: $enum| -> () {
                    match r#enum {
                        $enum :: $enum_variant => (),
                        _ => (),
                    }
                };
            )*

            ()
        };

        #[cfg_attr(feature = "manual_testing", derive(Deserialize))]
        #[derive(Serialize)]
        #[serde(crate = "extern_crate::serde")]
        $visability enum $enum_name {
            $($enum_variant $({ $($enum_variant_field: $enum_variant_field_type,)* })?,)*
        }
    }
}
