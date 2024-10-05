use skype::emoticon_data::{Item, Media};

#[derive(Debug)]
pub struct EmoticonMediaInfo {
    pub first_frame: u32,
    pub frame_count: u32,
    pub fps: u32,
}

#[derive(Debug)]
pub struct Emoticon {
    pub id: String,
    pub media: EmoticonMediaInfo,
}

impl From<Item> for Emoticon {
    fn from(item: Item) -> Self {
        Emoticon {
            id: item.id,
            media: item
                .media
                .expect("Emoticons should have media info!")
                .into(),
        }
    }
}

impl From<Media> for EmoticonMediaInfo {
    fn from(media: Media) -> Self {
        EmoticonMediaInfo {
            first_frame: media.default.first_frame as u32,
            frame_count: media.default.frames_count as u32,
            fps: media.default.fps as u32,
        }
    }
}
