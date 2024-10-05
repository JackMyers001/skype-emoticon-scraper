use std::{collections::HashMap, fs::File, io::BufWriter, path::PathBuf, sync::Arc};

use clap::Parser;
use color_eyre::eyre::Result;
use emoticon::Emoticon;
use futures::StreamExt;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use skype::{emoticon_data::ItemType, DownloadEmoticonImageError, SkypeClient};
use spritesheet::Spritesheet;
use tracing::{error, info, instrument, level_filters::LevelFilter, trace, warn, Instrument};
use tracing_subscriber::EnvFilter;

mod cli;
mod emoticon;
mod spritesheet;

#[instrument(skip(spritesheet))]
fn emoticon_to_file(spritesheet: &Spritesheet, output_directory: &PathBuf) -> Result<()> {
    let extension = if spritesheet.is_single_frame() {
        "png"
    } else {
        "gif"
    };

    let path = output_directory.join(format!(
        "{}/{}.{}",
        spritesheet.width(),
        spritesheet.name,
        extension
    ));

    // Create required directories, if they don't exist
    std::fs::create_dir_all(path.parent().unwrap())?;

    if path.exists() {
        warn!("File already exists: {:?}. Overwriting!", path);
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    if spritesheet.is_single_frame() {
        spritesheet.write_to_png(writer)?;
    } else {
        spritesheet.write_to_gif(writer)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    let args = cli::Args::parse();

    let skype_client = SkypeClient::with_version(&args.skype_version);
    let skype_config = skype_client.get_config().await?;
    let emoticon_config = skype_client.get_emoticon_config(&skype_config).await?;

    let emoticons: HashMap<_, _> = emoticon_config
        .items
        .into_iter()
        .filter(|item| item.type_field == ItemType::Emoticon)
        .map(Emoticon::from)
        .map(|emoticon| (emoticon.id.clone(), emoticon))
        .collect();

    let emoticons_count = emoticons.len();

    trace!("Emoticon count: {}", emoticons_count);

    let skype_client = Arc::new(skype_client);

    for resolution in args.resolutions {
        for emoticon_chunk in &emoticons.values().chunks(args.batch_size) {
            let futures = emoticon_chunk.enumerate().map(|(idx, emoticon)| {
                let skype_client = skype_client.clone();

                async move {
                    trace!("Downloading emoticon {} of {}", idx + 1, emoticons_count);

                    let image = skype_client
                        .download_emoticon_image(&emoticon.id, resolution)
                        .await?;

                    assert!(image.width() == resolution);
                    assert!(image.height() == emoticon.media.frame_count * resolution);

                    Ok::<_, DownloadEmoticonImageError>(Spritesheet::new(
                        &emoticon.id,
                        image,
                        emoticon.media.frame_count,
                        emoticon.media.first_frame,
                        emoticon.media.fps,
                    ))
                }
            });

            let spritesheets = futures::stream::iter(futures)
                .buffer_unordered(args.batch_size / 2)
                .filter_map(|r| async { r.ok() })
                .collect::<Vec<_>>()
                .instrument(tracing::info_span!("Downloading all emoticons"))
                .await;

            info!(
                "Downloaded {} emoticons of {} for resolution {}. Writing to file...",
                spritesheets.len(),
                emoticons_count,
                resolution
            );

            spritesheets.par_iter().for_each(|spritesheet| {
                if emoticon_to_file(spritesheet, &args.output_dir).is_err() {
                    error!("Failed to convert emoticon to GIF: {:?}", spritesheet);
                }
            });
        }

        info!(
            "Finished downloading all emoticons for resolution '{}'",
            resolution
        );
    }

    info!("Finished downloading all emoticons for all resolutions!");

    Ok(())
}
