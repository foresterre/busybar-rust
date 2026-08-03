use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use busybar_render::{ImageFormat, PixelLayout, Raster, RawImage, RenderError};
use busylib::proto::bsb_frame::{Encoding, Frame as StreamedFrame, PixelFormat, Screen};

use crate::types::screen_arg::ScreenArg;

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("the {screen} screen frame is not valid base64")]
    Base64 {
        screen: ScreenArg,
        #[source]
        source: base64::DecodeError,
    },

    #[error("the device streamed a {encoding} frame, which is not supported yet")]
    UnsupportedEncoding { encoding: &'static str },

    #[error("the device streamed a {encoding} frame in {format}, which is not supported yet")]
    UnsupportedCombination {
        encoding: &'static str,
        format: &'static str,
    },

    #[error("the run-length data ends mid-run, {offset} bytes in")]
    TruncatedRunLength { offset: usize },

    #[error(transparent)]
    Render(#[from] RenderError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame(RawImage);

impl Frame {
    pub fn decode(screen: ScreenArg, body: &[u8]) -> Result<Self, FrameError> {
        let pixels = STANDARD
            .decode(body.trim_ascii())
            .map_err(|source| FrameError::Base64 { screen, source })?;

        Self::from_pixels(screen, &pixels)
    }

    pub fn from_pixels(screen: ScreenArg, pixels: &[u8]) -> Result<Self, FrameError> {
        let (width, height, layout) = match screen {
            ScreenArg::Front => (72, 16, PixelLayout::Bgr888),
            ScreenArg::Back => (80, 80, PixelLayout::Gray8),
        };

        Ok(Self(RawImage::new(width, height, layout, pixels)?))
    }

    pub fn from_streamed(frame: &StreamedFrame) -> Result<Self, FrameError> {
        let pixels = self::decode_pixels(frame)?;

        let layout = match frame.pixel_format() {
            PixelFormat::Rgb888 => PixelLayout::Bgr888,
            PixelFormat::L8 | PixelFormat::L4 => PixelLayout::Gray8,
        };

        Ok(Self(RawImage::new(
            frame.width,
            frame.height,
            layout,
            &pixels,
        )?))
    }

    pub fn image(&self) -> &RawImage {
        &self.0
    }

    pub fn with_raster(&self, raster: Raster) -> Result<Self, FrameError> {
        Ok(Self(self.0.with_raster(raster)?))
    }

    pub fn encode(&self, format: ImageFormat) -> Result<Vec<u8>, FrameError> {
        Ok(self.0.encode(format)?)
    }

    pub fn encode_base64(&self, format: ImageFormat) -> Result<String, FrameError> {
        Ok(self.0.encode_base64(format)?)
    }
}

fn decode_pixels(frame: &StreamedFrame) -> Result<Vec<u8>, FrameError> {
    let bytes_per_pixel = match frame.pixel_format() {
        PixelFormat::Rgb888 => 3,
        PixelFormat::L8 => 1,
        PixelFormat::L4 => 0,
    };

    match (frame.encoding(), bytes_per_pixel) {
        (Encoding::Plain, 0) => Ok(self::expand_nibbles(&frame.data)),
        (Encoding::Plain, _) => Ok(frame.data.clone()),
        (Encoding::RunLength, 0) => Err(FrameError::UnsupportedCombination {
            encoding: self::encoding_name(Encoding::RunLength),
            format: self::format_name(PixelFormat::L4),
        }),
        (Encoding::RunLength, size) => self::run_length(&frame.data, size),
        (encoding @ (Encoding::Deflate | Encoding::DeflateRunLength), _) => {
            Err(FrameError::UnsupportedEncoding {
                encoding: self::encoding_name(encoding),
            })
        }
    }
}

fn run_length(data: &[u8], bytes_per_pixel: usize) -> Result<Vec<u8>, FrameError> {
    let mut pixels = Vec::with_capacity(data.len());
    let mut offset = 0;

    while offset < data.len() {
        let control = data[offset];
        offset += 1;

        let count = usize::from(control & 0x7F);
        let span = if control & 0x80 == 0 {
            bytes_per_pixel
        } else {
            count * bytes_per_pixel
        };

        let Some(chunk) = data.get(offset..offset + span) else {
            return Err(FrameError::TruncatedRunLength { offset });
        };

        if control & 0x80 == 0 {
            for _ in 0..count {
                pixels.extend_from_slice(chunk);
            }
        } else {
            pixels.extend_from_slice(chunk);
        }

        offset += span;
    }

    Ok(pixels)
}

fn expand_nibbles(data: &[u8]) -> Vec<u8> {
    let mut pixels = Vec::with_capacity(data.len() * 2);

    for byte in data {
        pixels.push((byte >> 4) * 17);
        pixels.push((byte & 0x0F) * 17);
    }

    pixels
}

pub fn encoding_name(encoding: Encoding) -> &'static str {
    match encoding {
        Encoding::Plain => "plain",
        Encoding::RunLength => "run-length",
        Encoding::Deflate => "deflate",
        Encoding::DeflateRunLength => "deflate+run-length",
    }
}

pub fn format_name(format: PixelFormat) -> &'static str {
    match format {
        PixelFormat::Rgb888 => "rgb888",
        PixelFormat::L8 => "l8",
        PixelFormat::L4 => "l4",
    }
}

pub fn screen_name(screen: Screen) -> &'static str {
    match screen {
        Screen::Front => "front",
        Screen::Back => "back",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn streamed(
        encoding: Encoding,
        format: PixelFormat,
        width: u32,
        height: u32,
        data: Vec<u8>,
    ) -> StreamedFrame {
        StreamedFrame {
            screen: Screen::Front as i32,
            width,
            height,
            encoding: encoding as i32,
            pixel_format: format as i32,
            data,
        }
    }

    fn front_pixels() -> Vec<u8> {
        let mut pixels = vec![0u8; 72 * 16 * 3];
        pixels[0..3].copy_from_slice(&[0x11, 0x22, 0x33]);
        pixels
    }

    #[test]
    fn decodes_a_front_frame() {
        let body = STANDARD.encode(front_pixels());
        let frame = Frame::decode(ScreenArg::Front, body.as_bytes()).unwrap();

        assert!(
            frame
                .encode(ImageFormat::Png)
                .unwrap()
                .starts_with(b"\x89PNG")
        );
    }

    #[test]
    fn accepts_a_body_with_trailing_whitespace() {
        let body = format!("{}\n", STANDARD.encode(front_pixels()));

        assert!(Frame::decode(ScreenArg::Front, body.as_bytes()).is_ok());
    }

    #[test]
    fn rejects_a_body_which_is_not_base64() {
        let error = Frame::decode(ScreenArg::Front, b"not base64!").unwrap_err();

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
            "72x16 in bgr888 needs 3456 bytes, but 16 were given"
        );
    }

    #[test]
    fn a_run_length_frame_decodes_to_the_same_pixels_as_a_plain_one() {
        let mut runs = Vec::new();
        for _ in 0..9 {
            runs.extend_from_slice(&[0x7f, 0x11, 0x22, 0x33]);
        }
        runs.extend_from_slice(&[0x09, 0x44, 0x55, 0x66]);

        let mut plain = Vec::new();
        for _ in 0..1143 {
            plain.extend_from_slice(&[0x11, 0x22, 0x33]);
        }
        for _ in 0..9 {
            plain.extend_from_slice(&[0x44, 0x55, 0x66]);
        }

        let encoded = Frame::from_streamed(&self::streamed(
            Encoding::RunLength,
            PixelFormat::Rgb888,
            72,
            16,
            runs,
        ))
        .unwrap()
        .encode(ImageFormat::Png)
        .unwrap();

        let expected = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::Rgb888,
            72,
            16,
            plain,
        ))
        .unwrap()
        .encode(ImageFormat::Png)
        .unwrap();

        assert_eq!(encoded, expected);
    }

    #[test]
    fn a_literal_span_decodes_to_the_pixels_it_carries() {
        let mut runs = vec![0x82, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        runs.extend_from_slice(&[0x7e, 0, 0, 0]);
        runs.extend_from_slice(&[0x7e, 0, 0, 0]);
        runs.extend_from_slice(&[0x06, 0, 0, 0]);

        let mut plain = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        plain.extend(std::iter::repeat_n(0, 258 * 3));

        let encoded = Frame::from_streamed(&self::streamed(
            Encoding::RunLength,
            PixelFormat::Rgb888,
            26,
            10,
            runs,
        ))
        .unwrap()
        .encode(ImageFormat::Png)
        .unwrap();

        let expected = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::Rgb888,
            26,
            10,
            plain,
        ))
        .unwrap()
        .encode(ImageFormat::Png)
        .unwrap();

        assert_eq!(encoded, expected);
    }

    #[test]
    fn decodes_a_plain_grayscale_frame() {
        let frame = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::L8,
            80,
            80,
            vec![0x7f; 6400],
        ))
        .unwrap();

        assert!(frame.encode(ImageFormat::Bmp).unwrap().starts_with(b"BM"));
    }

    #[test]
    fn expands_four_bit_grayscale_before_rendering() {
        let frame = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::L4,
            2,
            1,
            vec![0xf0],
        ))
        .unwrap();

        let expected = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::L8,
            2,
            1,
            vec![0xff, 0x00],
        ))
        .unwrap();

        assert_eq!(
            frame.encode(ImageFormat::Png).unwrap(),
            expected.encode(ImageFormat::Png).unwrap()
        );
    }

    #[test]
    fn encodes_to_base64() {
        let frame = Frame::from_streamed(&self::streamed(
            Encoding::Plain,
            PixelFormat::L8,
            2,
            1,
            vec![0x00, 0xff],
        ))
        .unwrap();

        assert_eq!(
            STANDARD
                .decode(frame.encode_base64(ImageFormat::Png).unwrap())
                .unwrap(),
            frame.encode(ImageFormat::Png).unwrap()
        );
    }

    #[test]
    fn rejects_an_encoding_it_cannot_decode() {
        let error = Frame::from_streamed(&self::streamed(
            Encoding::Deflate,
            PixelFormat::Rgb888,
            72,
            16,
            vec![0],
        ))
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "the device streamed a deflate frame, which is not supported yet"
        );
    }

    #[test]
    fn rejects_run_length_data_which_ends_mid_run() {
        let error = Frame::from_streamed(&self::streamed(
            Encoding::RunLength,
            PixelFormat::Rgb888,
            72,
            16,
            vec![0x7f, 0x11],
        ))
        .unwrap_err();

        assert!(matches!(error, FrameError::TruncatedRunLength { .. }));
    }

    #[test]
    fn rejects_a_frame_which_does_not_fill_its_geometry() {
        let error = Frame::from_streamed(&self::streamed(
            Encoding::RunLength,
            PixelFormat::Rgb888,
            72,
            16,
            vec![0x01, 0x11, 0x22, 0x33],
        ))
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "72x16 in bgr888 needs 3456 bytes, but 3 were given"
        );
    }
}
