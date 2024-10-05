# Skype Emoticon Scraper

A simple CLI utility to download emoticons (emoji) from Skype as `.png` and `.gif`. Emoticons are saved with full quality, transparency, and correct animation speed.

If you just want all emoticons at full resolution (240x240), I have pre-packaged the current emoticon set (as of 2024-10-05) and added it to [the Releases section](https://github.com/JackMyers001/skype-emoticon-scraper/releases).

## Features

- Fast
- Emoticons are downloaded directly from Skype
- Supports all available emoticon resolutions
- Animated emoticons start and run at the correct frame and speed

## Roadmap

- [x] Download emoticons and convert to images
- [ ] Non-flat (detailed?) emoticon variants
- [ ] Diverse emoticon variants

## Installation

### Requirements

- Rust (latest stable version - project was developed with `1.81.0`)

### Building from Source

1. Clone the repository

```bash
git clone https://github.com/JackMyers001/skype-emoticon-scraper.git
cd skype-emoticon-scraper
```

2. Build the project:

```bash
cargo build --release
```

3. The executable will be available at `target/release/skype-emoticon-scraper`

## Usage

For full CLI args, run:

```bash
skype-emoticon-scraper --help
```

### Examples

Download emoticons at maximum resolution:

```bash
skype-emoticon-scraper
```

Download emoticons at multiple resolutions:

```bash
skype-emoticon-scraper --res 20 120 240
```

Specify a custom output directory:

```bash
skype-emoticon-scraper -o path_to_emoticons
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is not associated with, endorsed by, or affiliated with Skype or Microsoft. It is an independent project created for educational and personal use. Users are responsible for ensuring their use of this tool complies with Skype's terms of service and applicable laws.
