use std::fmt;
use std::io::Cursor;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use image::{DynamicImage, GrayImage, RgbImage};

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("{width}x{height} in {layout} needs {expected} bytes, but {actual} were given")]
    UnexpectedSize {
        width: u32,
        height: u32,
        layout: PixelLayout,
        expected: usize,
        actual: usize,
    },

    #[error("a {width}x{height} image does not fit a raster of {pixel} with a {gap} gap")]
    RasterTooLarge {
        width: u32,
        height: u32,
        pixel: u32,
        gap: u32,
    },

    #[error("could not encode a {width}x{height} image as {format}")]
    Encode {
        width: u32,
        height: u32,
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
pub struct Raster {
    pixel: u32,
    gap: u32,
}

impl Raster {
    pub fn new(pixel: u32, gap: u32) -> Option<Self> {
        (pixel > 0).then_some(Self { pixel, gap })
    }

    pub fn pixel(self) -> u32 {
        self.pixel
    }

    pub fn gap(self) -> u32 {
        self.gap
    }

    fn extent(self, count: u32) -> Option<u32> {
        let cells = count.checked_mul(self.pixel)?;
        let gaps = count.saturating_sub(1).checked_mul(self.gap)?;

        cells.checked_add(gaps)
    }
}

impl Default for Raster {
    fn default() -> Self {
        Self { pixel: 3, gap: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelLayout {
    Rgb888,
    Bgr888,
    Gray8,
    Gray4,
}

impl PixelLayout {
    fn byte_len(self, width: u32, height: u32) -> usize {
        let pixels = (width as usize) * (height as usize);

        match self {
            Self::Rgb888 | Self::Bgr888 => pixels * 3,
            Self::Gray8 => pixels,
            Self::Gray4 => pixels.div_ceil(2),
        }
    }
}

impl fmt::Display for PixelLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb888 => f.write_str("rgb888"),
            Self::Bgr888 => f.write_str("bgr888"),
            Self::Gray8 => f.write_str("l8"),
            Self::Gray4 => f.write_str("l4"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawImage {
    width: u32,
    height: u32,
    image: DynamicImage,
}

impl RawImage {
    pub fn new(
        width: u32,
        height: u32,
        layout: PixelLayout,
        pixels: &[u8],
    ) -> Result<Self, RenderError> {
        let expected = layout.byte_len(width, height);

        if pixels.len() != expected {
            return Err(RenderError::UnexpectedSize {
                width,
                height,
                layout,
                expected,
                actual: pixels.len(),
            });
        }

        let image = match layout {
            PixelLayout::Rgb888 => DynamicImage::ImageRgb8(
                RgbImage::from_raw(width, height, pixels.to_vec()).expect("checked above"),
            ),
            PixelLayout::Bgr888 => {
                let mut pixels = pixels.to_vec();

                // The device says it outputs RGB but it really doesn't, so we swap
                for pixel in pixels.chunks_exact_mut(3) {
                    pixel.swap(0, 2);
                }

                DynamicImage::ImageRgb8(
                    RgbImage::from_raw(width, height, pixels).expect("checked above"),
                )
            }
            PixelLayout::Gray8 => DynamicImage::ImageLuma8(
                GrayImage::from_raw(width, height, pixels.to_vec()).expect("checked above"),
            ),
            PixelLayout::Gray4 => {
                let mut expanded = Vec::with_capacity(pixels.len() * 2);

                for byte in pixels {
                    expanded.push((byte >> 4) * 17);
                    expanded.push((byte & 0x0F) * 17);
                }

                expanded.truncate((width as usize) * (height as usize));

                DynamicImage::ImageLuma8(
                    GrayImage::from_raw(width, height, expanded).expect("checked above"),
                )
            }
        };

        Ok(Self {
            width,
            height,
            image,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn with_raster(&self, raster: Raster) -> Result<Self, RenderError> {
        let too_large = || RenderError::RasterTooLarge {
            width: self.width,
            height: self.height,
            pixel: raster.pixel,
            gap: raster.gap,
        };

        let width = raster.extent(self.width).ok_or_else(too_large)?;
        let height = raster.extent(self.height).ok_or_else(too_large)?;

        let step = raster.pixel + raster.gap;

        let image = match &self.image {
            DynamicImage::ImageLuma8(source) => {
                let mut target = GrayImage::new(width, height);
                self::paint(source, &mut target, step, raster.pixel);
                DynamicImage::ImageLuma8(target)
            }
            source => {
                let source = source.to_rgb8();
                let mut target = RgbImage::new(width, height);
                self::paint(&source, &mut target, step, raster.pixel);
                DynamicImage::ImageRgb8(target)
            }
        };

        Ok(Self {
            width,
            height,
            image,
        })
    }

    pub fn encode(&self, format: ImageFormat) -> Result<Vec<u8>, RenderError> {
        let mut buffer = Cursor::new(Vec::new());

        self.image
            .write_to(&mut buffer, format.encoding())
            .map_err(|source| RenderError::Encode {
                width: self.width,
                height: self.height,
                format,
                source,
            })?;

        Ok(buffer.into_inner())
    }

    pub fn encode_base64(&self, format: ImageFormat) -> Result<String, RenderError> {
        self.encode(format).map(|bytes| STANDARD.encode(bytes))
    }
}

fn paint<P: image::Pixel<Subpixel = u8>>(
    source: &image::ImageBuffer<P, Vec<u8>>,
    target: &mut image::ImageBuffer<P, Vec<u8>>,
    step: u32,
    size: u32,
) {
    for (x, y, pixel) in source.enumerate_pixels() {
        for offset_y in 0..size {
            for offset_x in 0..size {
                target.put_pixel(x * step + offset_x, y * step + offset_y, *pixel);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn reads_rgb_in_the_order_it_is_given() {
        let image = RawImage::new(1, 1, PixelLayout::Rgb888, &[0x11, 0x22, 0x33]).unwrap();

        assert_eq!(
            image.image.as_rgb8().unwrap().get_pixel(0, 0).0,
            [0x11, 0x22, 0x33]
        );
    }

    #[test]
    fn swaps_the_channels_of_a_bgr_buffer() {
        let image = RawImage::new(1, 1, PixelLayout::Bgr888, &[0x11, 0x22, 0x33]).unwrap();

        assert_eq!(
            image.image.as_rgb8().unwrap().get_pixel(0, 0).0,
            [0x33, 0x22, 0x11]
        );
    }

    #[test]
    fn expands_four_bit_grayscale_to_eight() {
        let image = RawImage::new(2, 1, PixelLayout::Gray4, &[0xf0]).unwrap();
        let luma = image.image.as_luma8().unwrap();

        assert_eq!(luma.get_pixel(0, 0).0, [0xff]);
        assert_eq!(luma.get_pixel(1, 0).0, [0x00]);
    }

    #[test]
    fn rejects_a_buffer_which_does_not_match_the_geometry() {
        let error = RawImage::new(72, 16, PixelLayout::Rgb888, &[0; 16]).unwrap_err();

        assert_eq!(
            error.to_string(),
            "72x16 in rgb888 needs 3456 bytes, but 16 were given"
        );
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
    fn a_raster_spaces_the_pixels_out_and_leaves_black_between_them() {
        let image = RawImage::new(
            2,
            2,
            PixelLayout::Rgb888,
            &[
                0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
            ],
        )
        .unwrap();

        let raster = Raster::new(3, 1).unwrap();
        let rastered = image.with_raster(raster).unwrap();

        assert_eq!((rastered.width(), rastered.height()), (7, 7));

        let pixels = rastered.image.to_rgb8();

        assert_eq!(pixels.get_pixel(0, 0).0, [0xff, 0x00, 0x00]);
        assert_eq!(pixels.get_pixel(2, 2).0, [0xff, 0x00, 0x00]);
        assert_eq!(pixels.get_pixel(3, 0).0, [0x00, 0x00, 0x00]);
        assert_eq!(pixels.get_pixel(0, 3).0, [0x00, 0x00, 0x00]);
        assert_eq!(pixels.get_pixel(4, 0).0, [0x00, 0xff, 0x00]);
        assert_eq!(pixels.get_pixel(4, 4).0, [0xff, 0xff, 0xff]);
    }

    #[test]
    fn a_raster_keeps_a_grayscale_image_grayscale() {
        let image = RawImage::new(2, 1, PixelLayout::Gray8, &[0xff, 0x40]).unwrap();
        let rastered = image.with_raster(Raster::default()).unwrap();

        assert_eq!((rastered.width(), rastered.height()), (7, 3));

        let pixels = rastered.image.as_luma8().expect("stays grayscale");

        assert_eq!(pixels.get_pixel(0, 0).0, [0xff]);
        assert_eq!(pixels.get_pixel(3, 0).0, [0x00]);
        assert_eq!(pixels.get_pixel(4, 0).0, [0x40]);
    }

    #[test]
    fn a_raster_needs_a_pixel_size() {
        assert!(Raster::new(0, 1).is_none());
        assert!(Raster::new(1, 0).is_some());
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
