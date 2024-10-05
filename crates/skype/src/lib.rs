use config::SkypeConfigRoot;
use thiserror::Error;

pub mod config;
pub mod emoticon_data;

#[derive(Error, Debug)]
pub enum DownloadEmoticonImageError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
}

pub struct SkypeClient {
    client: reqwest::Client,
    version: String,
}

impl Default for SkypeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SkypeClient {
    const CONFIG_API_BASE_URL: &'static str = "https://config.edge.skype.com/config/v1/Skype";
    const EMOTICON_API_BASE_URL: &'static str =
        "https://static-asm.secure.skypeassets.com/pes/v1/emoticons";
    pub const SKYPE_CLIENT_VERSION: &'static str = "1418_8.123.0.203";

    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            version: Self::SKYPE_CLIENT_VERSION.to_owned(),
        }
    }

    pub fn with_version(client_version: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            version: client_version.to_owned(),
        }
    }

    pub async fn get_config(&self) -> Result<config::SkypeConfigRoot, reqwest::Error> {
        let url = format!("{}/{}", Self::CONFIG_API_BASE_URL, self.version);

        self.client.get(&url).send().await?.json().await
    }

    pub async fn get_emoticon_config(
        &self,
        config: &SkypeConfigRoot,
    ) -> Result<emoticon_data::Root, reqwest::Error> {
        self.client
            .get(&config.skype_personalization.pes_config)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn download_emoticon_image(
        &self,
        emoticon_id: &str,
        resolution: u32,
    ) -> Result<image::DynamicImage, DownloadEmoticonImageError> {
        let url = format!(
            "{}/{emoticon_id}/views/default_{resolution}_anim_f",
            Self::EMOTICON_API_BASE_URL,
        );

        let image_bytes = self.client.get(&url).send().await?.bytes().await?;
        let image = image::load_from_memory(&image_bytes)?;

        Ok(image)
    }
}
