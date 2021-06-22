use gif::DisposalMethod;
use iced::{image::Handle, Image};
use image::{buffer::ConvertBuffer, GenericImage, ImageBuffer};
use std::{
    fs::File,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum GifError {
    IoError(std::io::Error),
    Decode(gif::DecodingError),
    Other(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for GifError {
    fn from(e: std::io::Error) -> Self {
        GifError::IoError(e)
    }
}

impl From<gif::DecodingError> for GifError {
    fn from(e: gif::DecodingError) -> Self {
        GifError::Decode(e)
    }
}

impl From<Box<dyn std::error::Error>> for GifError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        GifError::Other(e)
    }
}

pub struct AnimatedImage {
    frames: Vec<ImageFrame>,
    index: usize,
    time: Option<Instant>,
}

struct ImageFrame {
    handle: Handle,
    delay: u16,
}

impl AnimatedImage {
    pub fn from_gif<P: AsRef<Path>>(path: P) -> std::result::Result<Self, GifError> {
        type BgraImage = ImageBuffer<image::Bgra<u8>, Vec<u8>>;
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let file = File::open(path)?;
        let mut decoder = decoder.read_info(file)?;
        let width = decoder.width();
        let height = decoder.height();

        let mut frames = Vec::new();
        let mut base_frame: Option<BgraImage> = None;
        while let Some(frame) = decoder.read_next_frame()? {
            let img: ImageBuffer<image::Rgba<u8>, _> = image::ImageBuffer::from_raw(
                frame.width as u32,
                frame.height as u32,
                &frame.buffer[..],
            )
            .unwrap();
            let img: BgraImage = img.convert();
            let mut canvas: BgraImage = if let Some(ref base) = base_frame {
                base.clone()
            } else {
                ImageBuffer::new(width as u32, height as u32)
            };
            canvas
                .copy_from(&img, frame.left as u32, frame.top as u32)
                .unwrap();

            dbg!(frame.dispose);
            match frame.dispose {
                DisposalMethod::Any => {}
                DisposalMethod::Keep => {
                    base_frame = Some(canvas.clone());
                }
                DisposalMethod::Background => {
                    //if full screen
                    if frame.top == 0
                        && frame.left == 0
                        && frame.width == width
                        && frame.height == height
                    {
                        base_frame = None;
                    } else {
                        //clear rect
                        let fill: BgraImage =
                            ImageBuffer::new(frame.width as u32, frame.height as u32);
                        let mut img = canvas.clone();
                        img.copy_from(&fill, frame.left as u32, frame.top as u32)
                            .unwrap();
                        base_frame = Some(img);
                    }
                }
                DisposalMethod::Previous => {}
            }
            let frame = ImageFrame {
                handle: Handle::from_pixels(width as u32, height as u32, canvas.to_vec()),
                delay: frame.delay,
            };
            frames.push(frame);
        }
        Ok(Self {
            frames,
            index: 0,
            time: None,
        })
    }

    pub fn view(&mut self) -> Image {
        use image::{Bgra, GenericImage, Rgba};
        //find frame
        let mut frame = &self.frames[self.index];
        let time = self.time.get_or_insert_with(|| Instant::now());
        if time.elapsed() >= Duration::from_millis(frame.delay as u64 * 10) {
            if self.index + 1 == self.frames.len() {
                self.index = 0;
            } else {
                self.index += 1;
            }
            self.time = Some(Instant::now());
            dbg!(self.index);
            frame = &self.frames[self.index];
        }
        //display
        Image::new(frame.handle.clone())
    }
}
