use std::fmt;

use image::{DynamicImage, GrayImage, RgbImage};

use crate::error::RenderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raster {
    pixel: u32,
    gap: u32,
}

impl Raster {
    pub const fn new(pixel: u32, gap: u32) -> Option<Self> {
        if pixel == 0 {
            return None;
        }

        Some(Self { pixel, gap })
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

    pub(crate) fn buffer(&self) -> &DynamicImage {
        &self.image
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
                paint(source, &mut target, step, raster.pixel);
                DynamicImage::ImageLuma8(target)
            }
            source => {
                let source = source.to_rgb8();
                let mut target = RgbImage::new(width, height);
                paint(&source, &mut target, step, raster.pixel);
                DynamicImage::ImageRgb8(target)
            }
        };

        Ok(Self {
            width,
            height,
            image,
        })
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
    fn reads_rgb_in_the_order_it_is_given() {
        let image = RawImage::new(1, 1, PixelLayout::Rgb888, &[0x11, 0x22, 0x33]).unwrap();

        assert_eq!(
            image.buffer().as_rgb8().unwrap().get_pixel(0, 0).0,
            [0x11, 0x22, 0x33]
        );
    }

    #[test]
    fn swaps_the_channels_of_a_bgr_buffer() {
        let image = RawImage::new(1, 1, PixelLayout::Bgr888, &[0x11, 0x22, 0x33]).unwrap();

        assert_eq!(
            image.buffer().as_rgb8().unwrap().get_pixel(0, 0).0,
            [0x33, 0x22, 0x11]
        );
    }

    #[test]
    fn expands_four_bit_grayscale_to_eight() {
        let image = RawImage::new(2, 1, PixelLayout::Gray4, &[0xf0]).unwrap();
        let luma = image.buffer().as_luma8().unwrap();

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

        let pixels = rastered.buffer().to_rgb8();

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

        let pixels = rastered.buffer().as_luma8().expect("stays grayscale");

        assert_eq!(pixels.get_pixel(0, 0).0, [0xff]);
        assert_eq!(pixels.get_pixel(3, 0).0, [0x00]);
        assert_eq!(pixels.get_pixel(4, 0).0, [0x40]);
    }

    #[test]
    fn a_raster_needs_a_pixel_size() {
        assert!(Raster::new(0, 1).is_none());
        assert!(Raster::new(1, 0).is_some());
    }
}
