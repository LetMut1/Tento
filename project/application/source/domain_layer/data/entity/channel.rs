use std::borrow::Cow;

pub struct Channel<'a> {
    id: i64,
    /// application_user_id
    owner: i64,
    name: Cow<'a, str>,
    linked_name: String,
    description: Option<String>,
    access_modifier: i16,
    visability_modifier: i16,
    orientation: Vec<i16>,
    cover_image_path: Option<String>,
    background_image_path: Option<String>,
    subscribers_quantity: i64,
    marks_quantity: i64,
    viewing_quantity: i64,
    created_at: String
}

impl<'a> Channel<'a> {
    pub fn new(
        id: i64,
        owner: i64,
        name: Cow<'a, str>,
        linked_name: String,
        description: Option<String>,
        access_modifier: i16,
        visability_modifier: i16,
        orientation: Vec<i16>,
        cover_image_path: Option<String>,
        background_image_path: Option<String>,
        subscribers_quantity: i64,
        marks_quantity: i64,
        viewing_quantity: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            owner,
            name,
            linked_name,
            description,
            access_modifier,
            visability_modifier,
            orientation,
            cover_image_path,
            background_image_path,
            subscribers_quantity,
            marks_quantity,
            viewing_quantity,
            created_at
        };
    }

    pub fn into_inner(self) -> (
        i64,
        i64,
        Cow<'a, str>,
        String,
        Option<String>,
        i16,
        i16,
        Vec<i16>,
        Option<String>,
        Option<String>,
        i64,
        i64,
        i64,
        String
    ) {
        return (
            self.id,
            self.owner,
            self.name,
            self.linked_name,
            self.description,
            self.access_modifier,
            self.visability_modifier,
            self.orientation,
            self.cover_image_path,
            self.background_image_path,
            self.subscribers_quantity,
            self.marks_quantity,
            self.viewing_quantity,
            self.created_at
        );
    }

    pub fn get_id<'b>(&'b self) -> i64 {
        return self.id;
    }

    pub fn get_owner<'b>(&'b self) -> i64 {
        return self.owner;
    }

    pub fn get_access_modifier<'b>(&'b self) -> i16 {
        return self.access_modifier;
    }

    pub fn get_visability_modifier<'b>(&'b self) -> i16 {
        return self.visability_modifier;
    }
}

pub enum AccessModifier {
    /// 0 in integer representation.
    Open,
    /// 1 in integer representation.
    Close
}

pub enum VisabilityModifier {
    /// 0 in integer representation.
    Public,
    /// 1 in integer representation.
    Private
}