use std::fmt;
use std::io::Cursor;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

use crate::error::RenderError;
use crate::raw::RawImage;

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

impl RawImage {
    pub fn encode(&self, format: ImageFormat) -> Result<Vec<u8>, RenderError> {
        let mut buffer = Cursor::new(Vec::new());

        self.buffer()
            .write_to(&mut buffer, format.encoding())
            .map_err(|source| RenderError::Encode {
                width: self.width(),
                height: self.height(),
                format,
                source,
            })?;

        Ok(buffer.into_inner())
    }

    pub fn encode_base64(&self, format: ImageFormat) -> Result<String, RenderError> {
        self.encode(format).map(|bytes| STANDARD.encode(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::PixelLayout;

    #[test]
    fn picks_a_format_from_the_file_extension() {
        assert_eq!(
            ImageFormat::from_path(Path::new("frame.bmp")),
            Some(ImageFormat::Bmp)
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
    fn encodes_every_format_it_supports() {
        let image = RawImage::new(2, 2, PixelLayout::Gray8, &[0, 64, 128, 255]).unwrap();

        assert!(image.encode(ImageFormat::Bmp).unwrap().starts_with(b"BM"));
        assert!(
            image
                .encode(ImageFormat::Jpeg)
                .unwrap()
                .starts_with(&[0xff, 0xd8])
        );
        assert!(
            image
                .encode(ImageFormat::Png)
                .unwrap()
                .starts_with(b"\x89PNG")
        );
    }

    #[test]
    fn encodes_to_base64() {
        let image = RawImage::new(1, 1, PixelLayout::Gray8, &[0x7f]).unwrap();
        let encoded = image.encode_base64(ImageFormat::Png).unwrap();

        assert_eq!(
            STANDARD.decode(&encoded).unwrap(),
            image.encode(ImageFormat::Png).unwrap()
        );
    }
}
