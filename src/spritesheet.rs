use std::io::Write;

use image::{
    codecs::{
        gif::{GifEncoder, Repeat},
        png::PngEncoder,
    },
    Delay, DynamicImage, ExtendedColorType, Frame, ImageEncoder, ImageResult,
};

#[derive(Debug)]
pub struct Spritesheet {
    pub name: String,

    image: DynamicImage,

    frame_count: u32,
    first_frame: u32,
    delay: Delay,
}

impl Spritesheet {
    pub fn new(
        name: &str,
        image: DynamicImage,
        frame_count: u32,
        first_frame: u32,
        fps: u32,
    ) -> Self {
        // TODO: should we validate the image dimensions?

        let delay = Delay::from_numer_denom_ms(1000, fps);

        Self {
            name: name.to_string(),
            image,
            frame_count,
            first_frame,
            delay,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    #[inline]
    pub fn is_single_frame(&self) -> bool {
        self.frame_count == 1
    }

    fn get_frame(&self, index: u32) -> Frame {
        let width = self.width();

        let image_frame = self.image.crop_imm(0, index * width, width, width);

        Frame::from_parts(image_frame.into(), 0, 0, self.delay)
    }

    pub fn as_frames(&self) -> Vec<Frame> {
        let mut frames = Vec::new();

        for i in self.first_frame..self.frame_count {
            frames.push(self.get_frame(i));
        }

        for i in 0..self.first_frame {
            frames.push(self.get_frame(i));
        }

        assert!(frames.len() == self.frame_count as usize);

        frames
    }

    pub fn write_to_gif<W: Write>(&self, writer: W) -> ImageResult<()> {
        let frames = self.as_frames();

        let mut gif_encoder = GifEncoder::new(writer);
        gif_encoder.set_repeat(Repeat::Infinite)?;
        gif_encoder.encode_frames(frames)?;

        Ok(())
    }

    pub fn write_to_png<W: Write>(&self, writer: W) -> ImageResult<()> {
        assert!(self.frame_count == 1);
        assert!(self.first_frame == 1);

        let width = self.width();
        let buffer = self.image.as_rgba8().unwrap();

        let png_encoder = PngEncoder::new(writer);
        png_encoder.write_image(buffer, width, width, ExtendedColorType::Rgba8)?;

        Ok(())
    }
}
