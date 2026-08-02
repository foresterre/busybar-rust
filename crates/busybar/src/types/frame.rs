use std::fmt;
use std::io::Cursor;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use image::{DynamicImage, GrayImage, RgbImage};

use crate::values::ScreenArg;

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("the {screen} screen frame is not valid base64")]
    Base64 {
        screen: ScreenArg,
        #[source]
        source: base64::DecodeError,
    },

    #[error(
        "expected {expected} bytes of pixel data for the {screen} screen, but the device sent {actual}"
    )]
    UnexpectedSize {
        screen: ScreenArg,
        expected: usize,
        actual: usize,
    },

    #[error("could not encode the {screen} screen frame as {format}")]
    Encode {
        screen: ScreenArg,
        format: ImageFormat,
        #[source]
        source: image::ImageError,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Bmp,
    Jpeg,
    Png,
}

impl ImageFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_ascii_lowercase();

        match extension.as_str() {
            "bmp" => Some(Self::Bmp),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            _ => None,
        }
    }

    fn encoding(self) -> image::ImageFormat {
        match self {
            Self::Bmp => image::ImageFormat::Bmp,
            Self::Jpeg => image::ImageFormat::Jpeg,
            Self::Png => image::ImageFormat::Png,
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bmp => f.write_str("bmp"),
            Self::Jpeg => f.write_str("jpeg"),
            Self::Png => f.write_str("png"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PixelLayout {
    Bgr888 { width: u32, height: u32 },
    Gray8 { width: u32, height: u32 },
}

impl PixelLayout {
    const fn of(screen: ScreenArg) -> Self {
        match screen {
            ScreenArg::Front => Self::Bgr888 {
                width: 72,
                height: 16,
            },
            ScreenArg::Back => Self::Gray8 {
                width: 80,
                height: 80,
            },
        }
    }

    const fn byte_len(self) -> usize {
        match self {
            Self::Bgr888 { width, height } => (width as usize) * (height as usize) * 3,
            Self::Gray8 { width, height } => (width as usize) * (height as usize),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    screen: ScreenArg,
    image: DynamicImage,
}

impl Frame {
    pub fn decode(screen: ScreenArg, body: &[u8]) -> Result<Self, FrameError> {
        let pixels = STANDARD
            .decode(body.trim_ascii())
            .map_err(|source| FrameError::Base64 { screen, source })?;

        Self::from_pixels(screen, &pixels)
    }

    pub fn from_pixels(screen: ScreenArg, pixels: &[u8]) -> Result<Self, FrameError> {
        let layout = PixelLayout::of(screen);
        let expected = layout.byte_len();

        if pixels.len() != expected {
            return Err(FrameError::UnexpectedSize {
                screen,
                expected,
                actual: pixels.len(),
            });
        }

        let image = match layout {
            PixelLayout::Bgr888 { width, height } => {
                let mut buffer = Vec::with_capacity(pixels.len());

                for pixel in pixels.chunks_exact(3) {
                    buffer.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
                }

                let buffer = RgbImage::from_raw(width, height, buffer)
                    .expect("buffer holds width * height * 3 bytes");

                DynamicImage::ImageRgb8(buffer)
            }
            PixelLayout::Gray8 { width, height } => {
                let buffer = GrayImage::from_raw(width, height, pixels.to_vec())
                    .expect("buffer holds width * height bytes");

                DynamicImage::ImageLuma8(buffer)
            }
        };

        Ok(Self { screen, image })
    }

    pub fn encode(&self, format: ImageFormat) -> Result<Vec<u8>, FrameError> {
        let mut buffer = Cursor::new(Vec::new());

        self.image
            .write_to(&mut buffer, format.encoding())
            .map_err(|source| FrameError::Encode {
                screen: self.screen,
                format,
                source,
            })?;

        Ok(buffer.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn front_pixels() -> Vec<u8> {
        let mut pixels = vec![0u8; 72 * 16 * 3];
        pixels[0..3].copy_from_slice(&[0x11, 0x22, 0x33]);
        pixels
    }

    fn back_pixels() -> Vec<u8> {
        let mut pixels = vec![0u8; 80 * 80];
        pixels[0] = 0x7f;
        pixels
    }

    #[test]
    fn picks_a_format_from_the_file_extension() {
        assert_eq!(
            ImageFormat::from_path(Path::new("frame.bmp")),
            Some(ImageFormat::Bmp)
        );
        assert_eq!(
            ImageFormat::from_path(Path::new("frame.jpg")),
            Some(ImageFormat::Jpeg)
        );
        assert_eq!(
            ImageFormat::from_path(Path::new("frame.JPEG")),
            Some(ImageFormat::Jpeg)
        );
        assert_eq!(
            ImageFormat::from_path(Path::new("./out/frame.png")),
            Some(ImageFormat::Png)
        );
        assert_eq!(ImageFormat::from_path(Path::new("frame.raw")), None);
        assert_eq!(ImageFormat::from_path(Path::new("frame")), None);
    }

    #[test]
    fn decodes_a_front_frame_as_rgb() {
        let body = STANDARD.encode(front_pixels());
        let frame = Frame::decode(ScreenArg::Front, body.as_bytes()).unwrap();

        let image = frame.image.as_rgb8().unwrap();

        assert_eq!(image.dimensions(), (72, 16));
        assert_eq!(image.get_pixel(0, 0).0, [0x33, 0x22, 0x11]);
    }

    #[test]
    fn decodes_a_back_frame_as_grayscale() {
        let body = STANDARD.encode(back_pixels());
        let frame = Frame::decode(ScreenArg::Back, body.as_bytes()).unwrap();

        let image = frame.image.as_luma8().unwrap();

        assert_eq!(image.dimensions(), (80, 80));
        assert_eq!(image.get_pixel(0, 0).0, [0x7f]);
    }

    #[test]
    fn accepts_a_body_with_trailing_whitespace() {
        let body = format!("{}\n", STANDARD.encode(front_pixels()));

        assert!(Frame::decode(ScreenArg::Front, body.as_bytes()).is_ok());
    }

    #[test]
    fn rejects_a_body_which_is_not_base64() {
        let error = Frame::decode(ScreenArg::Front, b"not base64!").unwrap_err();

        assert!(matches!(error, FrameError::Base64 { .. }));
        assert_eq!(
            error.to_string(),
            "the front screen frame is not valid base64"
        );
    }

    #[test]
    fn rejects_pixel_data_of_the_wrong_length() {
        let error = Frame::from_pixels(ScreenArg::Front, &[0u8; 16]).unwrap_err();

        assert_eq!(
            error.to_string(),
            "expected 3456 bytes of pixel data for the front screen, but the device sent 16"
        );
    }

    #[test]
    fn encodes_a_front_frame_in_every_format() {
        let frame = Frame::from_pixels(ScreenArg::Front, &front_pixels()).unwrap();

        assert!(frame.encode(ImageFormat::Bmp).unwrap().starts_with(b"BM"));
        assert!(
            frame
                .encode(ImageFormat::Jpeg)
                .unwrap()
                .starts_with(&[0xff, 0xd8])
        );
        assert!(
            frame
                .encode(ImageFormat::Png)
                .unwrap()
                .starts_with(b"\x89PNG")
        );
    }

    #[test]
    fn encodes_a_back_frame_in_every_format() {
        let frame = Frame::from_pixels(ScreenArg::Back, &back_pixels()).unwrap();

        assert!(frame.encode(ImageFormat::Bmp).unwrap().starts_with(b"BM"));
        assert!(
            frame
                .encode(ImageFormat::Jpeg)
                .unwrap()
                .starts_with(&[0xff, 0xd8])
        );
        assert!(
            frame
                .encode(ImageFormat::Png)
                .unwrap()
                .starts_with(b"\x89PNG")
        );
    }

    #[test]
    fn round_trips_a_png_back_to_the_original_pixels() {
        let frame = Frame::from_pixels(ScreenArg::Front, &front_pixels()).unwrap();
        let png = frame.encode(ImageFormat::Png).unwrap();

        let decoded = image::load_from_memory_with_format(&png, image::ImageFormat::Png).unwrap();

        assert_eq!(decoded, frame.image);
    }
}
