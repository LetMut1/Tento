use std::borrow::Cow;

pub struct Channel<'a> {
    id: i64,
    /// application_user_id
    owner: i64,
    name: Cow<'a, str>,
    linked_name: String,
    description: Option<String>,
    is_private: bool,
    orientation: Vec<i16>,
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
        is_private: bool,
        orientation: Vec<i16>,
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
            is_private,
            orientation,
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
        bool,
        Vec<i16>,
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
            self.is_private,
            self.orientation,
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

    pub fn get_is_private<'b>(&'b self) -> bool {
        return self.is_private;
    }
}