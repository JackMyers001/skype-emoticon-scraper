use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long = "res", value_delimiter = ' ', num_args = 1.., default_value = "240", value_parser = validate_resolutions, help = "Resolution(s) to download emoticons in")]
    pub resolutions: Vec<u32>,

    #[arg(
        short,
        long,
        default_value_t = 128,
        help = "Number of images to download in parallel. Higher values will run faster but increase memory usage"
    )]
    pub batch_size: usize,

    #[arg(
        short,
        long,
        default_value = "output",
        help = "Directory to save downloaded emoticons to"
    )]
    pub output_dir: PathBuf,

    #[arg(
        long = "lang",
        default_value = "en-US",
        help = "[UNUSED] Language code to use for downloading emoticons. Emoticons names are localized, so this will affect the names of the downloaded emoticons"
    )]
    pub language: String,

    #[arg(
        long,
        default_value = skype::SkypeClient::SKYPE_CLIENT_VERSION,
        help = "Skype client version to use for downloading emoticons. Different versions may have different emoticons",
    )]
    pub skype_version: String,
}

fn validate_resolutions(s: &str) -> Result<u32, String> {
    // Valid resolutions for Skype emoticons (i.e. what's stored on Skype's servers)
    const VALID_RESOLUTIONS: [u32; 12] = [20, 25, 30, 40, 50, 60, 80, 100, 120, 160, 180, 240];

    let resolution = s
        .parse::<u32>()
        .map_err(|_| "Resolution must be a non-negative whole number".to_string())?;

    if !VALID_RESOLUTIONS.contains(&resolution) {
        return Err(format!(
            "Resolution must be one of: [{}]",
            VALID_RESOLUTIONS
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    Ok(resolution)
}
