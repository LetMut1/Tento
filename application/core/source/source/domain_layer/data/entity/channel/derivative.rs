pub struct Channel2 {
    pub owner: i64,
    pub name: String,
    pub linked_name: String,
    pub description: Option<String>,
    pub access_modifier: i16,
    pub visability_modifier: i16,
    pub orientation: Vec<i16>,
    pub cover_image_path: Option<String>,
    pub background_image_path: Option<String>,
    pub subscribers_quantity: i64,
    pub marks_quantity: i64,
    pub viewing_quantity: i64,
}