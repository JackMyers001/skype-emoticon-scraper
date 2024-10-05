use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkypeConfigRoot {
    pub skype_personalization: SkypePersonalization,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SkypePersonalization {
    pub pes_config: String,
}
