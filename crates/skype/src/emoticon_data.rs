use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub tabs_root: String,
    pub packs_root: String,
    pub emoticons_root: String,
    pub items_root: String,
    pub tabs: Vec<Tab>,
    pub packs: Vec<Pack>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub sections: Vec<Section>,
    pub id: String,
    pub title: String,
    pub description: String,
    pub copyright: String,
    pub is_hidden: bool,
    pub price: String,
    pub expiry: Value,
    pub glyph_bg_color: String,
    pub is_discoverable: bool,
    #[serde(rename = "badgeETag")]
    pub badge_etag: String,
    pub keywords: Vec<String>,
    pub etag: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub pack: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pack {
    pub id: String,
    pub title: String,
    pub description: String,
    pub copyright: Value,
    pub is_hidden: bool,
    pub is_sponsored: bool,
    pub keywords: Value,
    pub price: String,
    pub expiry: Value,
    pub etag: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: ItemType,
    #[serde(default)]
    pub shortcuts: Vec<String>,
    pub visible: Option<bool>,
    pub use_in_sms: Option<bool>,
    pub media: Option<Media>,
    pub diverse: Option<bool>,
    pub fluent_diverse: Option<bool>,
    pub utf: Option<String>,
    pub description: String,
    pub keywords: Vec<String>,
    pub etag: String,
    #[serde(default)]
    pub group: Vec<String>,
    #[serde(default)]
    pub excluded_countries: Vec<String>,
    #[serde(default)]
    pub included_countries: Vec<String>,
    pub is_sponsored: Option<bool>,
    pub picker_title: Option<String>,
    pub auxiliary_text: Option<String>,
    pub auxiliary_url: Option<String>,
    pub transcript: Option<String>,
    pub copyright: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Emoticon,
    Flik,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub default: Default,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub first_frame: i64,
    pub frames_count: i64,
    pub frames_count_optimized: Option<i64>,
    pub fluent_frames_count: Option<i64>,
    pub fps: i64,
}
