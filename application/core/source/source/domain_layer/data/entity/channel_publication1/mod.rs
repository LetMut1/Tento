mod field;
use {
    self::field::{
        CreatedAt,
        ImagesPathes,
        Text,
        Id,
    },
    std::marker::PhantomData,
    super::channel::Channel_Id,
};
pub struct ChannelPublication1 {
    id: PhantomData<(i64, Id)>,
    channel__id: PhantomData<(i64, Channel_Id)>,
    images_pathes: PhantomData<(Vec<String>, ImagesPathes)>,
    text_: PhantomData<(String, Text)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}