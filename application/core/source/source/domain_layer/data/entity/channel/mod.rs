mod field;
pub mod derivative;
use {
    self::field::{
        AccessModifier,
        BackgroundImagePath,
        CoverImagePath,
        CreatedAt,
        Description,
        Id,
        LinkedName,
        MarksQuantity,
        Name,
        Orientation,
        SubscribersQuantity,
        ViewingQuantity,
        VisabilityModifier,
        ObfuscationValue,
    },
    super::user::User_Id,
    std::{
        borrow::Cow,
        marker::PhantomData,
    },
};
pub struct Channel<'a> {
    pub id: i64,
    _id: PhantomData<Id>,
    pub owner: i64,
    _owner: PhantomData<User_Id>,
    pub name: Cow<'a, str>,
    _name: PhantomData<Name>,
    pub linked_name: String,
    _linked_name: PhantomData<LinkedName>,
    pub description: Option<String>,
    _description: PhantomData<Description>,
    pub access_modifier: i16,
    _access_modifier: PhantomData<AccessModifier>,
    pub visability_modifier: i16,
    _visability_modifier: PhantomData<VisabilityModifier>,
    pub orientation: Vec<i16>,
    _orientation: PhantomData<Orientation>,
    pub cover_image_path: Option<String>,
    _cover_image_path: PhantomData<CoverImagePath>,
    pub background_image_path: Option<String>,
    _background_image_path: PhantomData<BackgroundImagePath>,
    pub subscribers_quantity: i64,
    _subscribers_quantity: PhantomData<SubscribersQuantity>,
    pub marks_quantity: i64,
    _marks_quantity: PhantomData<MarksQuantity>,
    pub viewing_quantity: i64,
    _viewing_quantity: PhantomData<ViewingQuantity>,
    pub obfuscation_value: i64,
    _obfuscation_value: PhantomData<ObfuscationValue>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
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
        obfuscation_value: i64,
        created_at: i64,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            owner,
            _owner: PhantomData,
            name,
            _name: PhantomData,
            linked_name,
            _linked_name: PhantomData,
            description,
            _description: PhantomData,
            access_modifier,
            _access_modifier: PhantomData,
            visability_modifier,
            _visability_modifier: PhantomData,
            orientation,
            _orientation: PhantomData,
            cover_image_path,
            _cover_image_path: PhantomData,
            background_image_path,
            _background_image_path: PhantomData,
            subscribers_quantity,
            _subscribers_quantity: PhantomData,
            marks_quantity,
            _marks_quantity: PhantomData,
            viewing_quantity,
            _viewing_quantity: PhantomData,
            obfuscation_value,
            _obfuscation_value: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
pub type Channel_BackgroundImagePath = BackgroundImagePath;
pub type Channel_CoverImagePath = CoverImagePath;
pub type Channel_CreatedAt = CreatedAt;
pub type Channel_MarksQuantity = MarksQuantity;
pub type Channel_Orientation = Orientation;
pub type Channel_SubscribersQuantity = SubscribersQuantity;
pub type Channel_ViewingQuantity = ViewingQuantity;
pub type Channel_AccessModifier = AccessModifier;
pub type Channel_Description = Description;
pub type Channel_Id = Id;
pub type Channel_LinkedName = LinkedName;
pub type Channel_Name = Name;
pub type Channel_VisabilityModifier = VisabilityModifier;
pub type Channel_ObfuscationValue = ObfuscationValue;
