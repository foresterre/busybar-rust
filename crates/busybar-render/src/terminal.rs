use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{QueueableCommand as _, queue};
use image::GenericImageView as _;

use crate::raw::{Raster, RawImage};

pub const MIRROR_RASTER: Raster = match Raster::new(1, 1) {
    Some(raster) => raster,
    None => panic!("a pixel of one is not zero"),
};

const UPPER_HALF_BLOCK: char = '▀';
const UNLIT: Color = Color::Rgb { r: 0, g: 0, b: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cells {
    pub columns: u32,
    pub rows: u32,
}

impl Cells {
    pub fn of(image: &RawImage) -> Self {
        Self {
            columns: image.width(),
            rows: image.height().div_ceil(2),
        }
    }
}

#[derive(Debug)]
pub struct Mirror<W: Write> {
    out: W,
    active: bool,
}

impl<W: Write> Mirror<W> {
    pub fn new(out: W) -> Self {
        Self { out, active: false }
    }

    pub fn enter(&mut self) -> io::Result<()> {
        if self.active {
            return Ok(());
        }

        self.active = true;

        queue!(self.out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
        self.out.flush()
    }

    pub fn leave(&mut self) -> io::Result<()> {
        if !self.active {
            return Ok(());
        }

        self.active = false;

        queue!(self.out, ResetColor, Show, LeaveAlternateScreen)?;
        self.out.flush()
    }

    pub fn draw(&mut self, image: &RawImage) -> io::Result<()> {
        let cells = Cells::of(image);
        let (columns, rows) = viewport();

        if cells.columns > columns || cells.rows > rows {
            return self.notice(&format!(
                "the terminal is {columns}x{rows} cells, but this frame needs {}x{}",
                cells.columns, cells.rows
            ));
        }

        paint(&mut self.out, image, cells)?;
        self.out.flush()
    }

    pub fn notice(&mut self, text: &str) -> io::Result<()> {
        queue!(
            self.out,
            ResetColor,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(text)
        )?;
        self.out.flush()
    }
}

impl<W: Write> Drop for Mirror<W> {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}

fn viewport() -> (u32, u32) {
    match crossterm::terminal::size() {
        Ok((columns, rows)) => (u32::from(columns), u32::from(rows)),
        Err(_) => (u32::MAX, u32::MAX),
    }
}

fn paint<W: Write>(out: &mut W, image: &RawImage, cells: Cells) -> io::Result<()> {
    for row in 0..cells.rows {
        let mut foreground = None;
        let mut background = None;

        out.queue(MoveTo(0, coordinate(row)))?;

        for column in 0..cells.columns {
            let top = sample(image, column, row * 2);
            let bottom = sample(image, column, row * 2 + 1);

            if foreground != Some(top) {
                out.queue(SetForegroundColor(top))?;
                foreground = Some(top);
            }

            if background != Some(bottom) {
                out.queue(SetBackgroundColor(bottom))?;
                background = Some(bottom);
            }

            out.queue(Print(UPPER_HALF_BLOCK))?;
        }

        queue!(out, ResetColor, Clear(ClearType::UntilNewLine))?;
    }

    queue!(
        out,
        MoveTo(0, coordinate(cells.rows)),
        Clear(ClearType::FromCursorDown)
    )?;

    Ok(())
}

fn sample(image: &RawImage, x: u32, y: u32) -> Color {
    if y >= image.height() {
        return UNLIT;
    }

    let [r, g, b, _] = image.buffer().get_pixel(x, y).0;

    Color::Rgb { r, g, b }
}

fn coordinate(value: u32) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::PixelLayout;

    fn render(image: &RawImage) -> String {
        let mut out = Vec::new();
        paint(&mut out, image, Cells::of(image)).unwrap();

        String::from_utf8(out).unwrap()
    }

    #[test]
    fn a_cell_stacks_two_pixel_rows() {
        let image = RawImage::new(3, 4, PixelLayout::Gray8, &[0; 12]).unwrap();

        assert_eq!(
            Cells::of(&image),
            Cells {
                columns: 3,
                rows: 2
            }
        );
    }

    #[test]
    fn an_odd_pixel_row_still_gets_a_whole_cell() {
        let image = RawImage::new(72, 31, PixelLayout::Gray8, &[0; 72 * 31]).unwrap();

        assert_eq!(
            Cells::of(&image),
            Cells {
                columns: 72,
                rows: 16
            }
        );
    }

    #[test]
    fn the_mirror_raster_spaces_a_front_frame_out_over_the_terminal_grid() {
        let image = RawImage::new(72, 16, PixelLayout::Rgb888, &[0; 72 * 16 * 3]).unwrap();
        let rastered = image.with_raster(MIRROR_RASTER).unwrap();

        assert_eq!((rastered.width(), rastered.height()), (143, 31));
        assert_eq!(
            Cells::of(&rastered),
            Cells {
                columns: 143,
                rows: 16
            }
        );
    }

    #[test]
    fn a_column_carries_its_top_pixel_as_the_foreground_and_its_bottom_as_the_background() {
        let image = RawImage::new(
            1,
            2,
            PixelLayout::Rgb888,
            &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        )
        .unwrap();

        let rendered = render(&image);

        assert!(rendered.contains("\x1b[38;2;17;34;51m"));
        assert!(rendered.contains("\x1b[48;2;68;85;102m"));
        assert!(rendered.contains(UPPER_HALF_BLOCK));
    }

    #[test]
    fn a_missing_bottom_pixel_reads_as_an_unlit_one() {
        let image = RawImage::new(1, 1, PixelLayout::Rgb888, &[0xff, 0xff, 0xff]).unwrap();

        let rendered = render(&image);

        assert!(rendered.contains("\x1b[38;2;255;255;255m"));
        assert!(rendered.contains("\x1b[48;2;0;0;0m"));
    }

    #[test]
    fn a_grayscale_pixel_repeats_over_all_three_channels() {
        let image = RawImage::new(1, 2, PixelLayout::Gray8, &[0x40, 0x80]).unwrap();

        let rendered = render(&image);

        assert!(rendered.contains("\x1b[38;2;64;64;64m"));
        assert!(rendered.contains("\x1b[48;2;128;128;128m"));
    }

    #[test]
    fn a_run_of_equal_pixels_only_sets_its_colours_once() {
        let image = RawImage::new(4, 2, PixelLayout::Gray8, &[0x10; 8]).unwrap();

        let rendered = render(&image);

        assert_eq!(rendered.matches("\x1b[38;2;16;16;16m").count(), 1);
        assert_eq!(rendered.matches("\x1b[48;2;16;16;16m").count(), 1);
        assert_eq!(rendered.matches(UPPER_HALF_BLOCK).count(), 4);
    }

    #[test]
    fn every_row_is_placed_at_a_fixed_position_so_the_frame_stays_put() {
        let image = RawImage::new(1, 6, PixelLayout::Gray8, &[0; 6]).unwrap();

        let rendered = render(&image);

        assert!(rendered.starts_with("\x1b[1;1H"));
        assert!(rendered.contains("\x1b[2;1H"));
        assert!(rendered.contains("\x1b[3;1H"));
        assert!(rendered.ends_with("\x1b[4;1H\x1b[J"));
    }

    #[test]
    fn a_frame_which_fits_the_viewport_is_painted() {
        let image = RawImage::new(4, 4, PixelLayout::Gray8, &[0x10; 16]).unwrap();

        let mut mirror = Mirror::new(Vec::new());
        mirror.draw(&image).unwrap();

        let rendered = String::from_utf8(std::mem::take(&mut mirror.out)).unwrap();

        assert_eq!(rendered.matches(UPPER_HALF_BLOCK).count(), 8);
    }

    #[test]
    fn a_frame_which_outgrows_the_viewport_says_how_much_room_it_needs() {
        let image = RawImage::new(4096, 2, PixelLayout::Gray8, &[0; 8192]).unwrap();

        let mut mirror = Mirror::new(Vec::new());
        mirror.draw(&image).unwrap();

        let rendered = String::from_utf8(std::mem::take(&mut mirror.out)).unwrap();

        assert!(!rendered.contains(UPPER_HALF_BLOCK));
        assert!(rendered.ends_with("but this frame needs 4096x1"));
    }

    #[test]
    fn entering_and_leaving_restores_the_terminal_once() {
        let mut mirror = Mirror::new(Vec::new());

        mirror.enter().unwrap();
        mirror.enter().unwrap();
        mirror.leave().unwrap();
        mirror.leave().unwrap();

        let rendered = String::from_utf8(std::mem::take(&mut mirror.out)).unwrap();

        assert_eq!(rendered.matches("\x1b[?1049h").count(), 1);
        assert_eq!(rendered.matches("\x1b[?1049l").count(), 1);
        assert_eq!(rendered.matches("\x1b[?25l").count(), 1);
        assert_eq!(rendered.matches("\x1b[?25h").count(), 1);
    }

    #[test]
    fn a_notice_replaces_whatever_was_on_screen() {
        let mut mirror = Mirror::new(Vec::new());
        mirror
            .notice("the device streamed a deflate frame")
            .unwrap();

        let rendered = String::from_utf8(std::mem::take(&mut mirror.out)).unwrap();

        assert!(rendered.contains("\x1b[2J"));
        assert!(rendered.ends_with("the device streamed a deflate frame"));
    }
}
