pub mod action_processor;
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Common1 {
    pub channel__id: i64,
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
    pub channel__cover_image_path: Option<String>,
    pub channel__background_image_path: Option<String>,
    pub is_user_subscribed: bool,
}