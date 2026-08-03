use crate::encoding::ImageFormat;
use crate::raw::PixelLayout;

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
