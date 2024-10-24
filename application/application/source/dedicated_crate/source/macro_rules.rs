macro_rules! enum_from {
    ($visability:vis enum $enum_name:ident { $($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)? }) => {
        const _: () = {
            $(
                let _ = move |r#enum: $enum| -> () {
                    match r#enum {
                        $enum :: $enum_variant => (),
                        _ => (),
                    }
                };
            )*
            ()
        };
        #[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
        #[derive(bitcode::Encode, bitcode::Decode)]
        $visability enum $enum_name {
            $($enum_variant $({ $($enum_variant_field: $enum_variant_field_type,)* })?,)*
        }
    };
}
pub(crate) use enum_from;