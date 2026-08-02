use serde::{Deserialize, Serialize};

use crate::types::{
    AppName, AssetPath, Color, ElementId, InvalidValue, Opacity, Priority, StockPath, Text,
    TryIntoValue,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayElements {
    /// Application ID for organizing assets
    pub application_name: AppName,
    /// Draw priority in the range [1, 100] inclusive. A draw request is accepted when its
    /// priority is greater than or equal to (>=) the priority of the currently running system
    /// app. Equal-priority requests from a different application_name override whatever is on
    /// screen. System app priority levels: stub/poweroff apps = 0 (always preemptable), any
    /// standard built-in app = 10, active BUSY/CUSTOM work session = 90. The draw API only
    /// accepts values 1–100; 0 is reserved for internal use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Color to blink the status LED, in #RRGGBBAA format.  If not specified, the LED will not
    /// blink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_notification_color: Option<Color>,
    /// Array of elements to display
    pub elements: Vec<DisplayElement>,
}

impl DisplayElements {
    pub fn new(application_name: impl TryIntoValue<AppName>) -> Result<Self, InvalidValue> {
        Ok(Self {
            application_name: application_name.try_into_value()?,
            priority: None,
            led_notification_color: None,
            elements: Vec::new(),
        })
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn led_notification_color(mut self, color: Color) -> Self {
        self.led_notification_color = Some(color);
        self
    }

    pub fn element(mut self, element: DisplayElement) -> Self {
        self.elements.push(element);
        self
    }

    pub fn elements(mut self, elements: impl IntoIterator<Item = DisplayElement>) -> Self {
        self.elements.extend(elements);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayElement {
    /// Unique identifier for the element
    pub id: ElementId,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<Lifetime>,
    /// X coordinate of selected anchor point relative to top-left of display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i16>,
    /// Y coordinate of selected anchor point relative to top-left of display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i16>,
    /// Which display to show the element on (for dual-display devices)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Screen>,
    /// Anchor point of element. Also use `x` and `y` to position element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    #[serde(flatten)]
    pub kind: ElementKind,
}

impl DisplayElement {
    pub fn builder(
        id: impl TryIntoValue<ElementId>,
    ) -> Result<DisplayElementBuilder, InvalidValue> {
        Ok(DisplayElementBuilder {
            id: id.try_into_value()?,
            lifetime: None,
            x: None,
            y: None,
            display: None,
            align: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DisplayElementBuilder {
    id: ElementId,
    lifetime: Option<Lifetime>,
    x: Option<i16>,
    y: Option<i16>,
    display: Option<Screen>,
    align: Option<Align>,
}

impl DisplayElementBuilder {
    pub fn at(mut self, x: i16, y: i16) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn screen(mut self, screen: Screen) -> Self {
        self.display = Some(screen);
        self
    }

    pub fn timeout_secs(mut self, seconds: u32) -> Self {
        self.lifetime = Some(Lifetime::timeout_secs(seconds));
        self
    }

    pub fn display_until(mut self, unix_seconds: u64) -> Self {
        self.lifetime = Some(Lifetime::display_until(unix_seconds));
        self
    }

    pub fn text(self, text: TextElement) -> DisplayElement {
        self.finish(ElementKind::Text(text))
    }

    pub fn image(self, image: ImageElement) -> DisplayElement {
        self.finish(ElementKind::Image(image))
    }

    pub fn animation(self, animation: AnimationElement) -> DisplayElement {
        self.finish(ElementKind::Animation(animation))
    }

    pub fn countdown(self, countdown: CountdownElement) -> DisplayElement {
        self.finish(ElementKind::Countdown(countdown))
    }

    pub fn rectangle(self, rectangle: RectangleElement) -> DisplayElement {
        self.finish(ElementKind::Rectangle(rectangle))
    }

    fn finish(self, kind: ElementKind) -> DisplayElement {
        DisplayElement {
            id: self.id,
            lifetime: self.lifetime,
            x: self.x,
            y: self.y,
            display: self.display,
            align: self.align,
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Lifetime {
    Timeout {
        /// Time in seconds the element should be displayed (0 for no timeout). Mutually
        /// exclusive with display_until.
        timeout: u32,
    },
    DisplayUntil {
        /// The element will be hidden when system time reaches the specified Unix timestamp
        /// (in seconds). Mutually exclusive with timeout.
        #[serde(with = "crate::serde_util::string_u64")]
        display_until: u64,
    },
}

impl Lifetime {
    pub fn timeout_secs(seconds: u32) -> Self {
        Self::Timeout { timeout: seconds }
    }

    pub fn display_until(unix_seconds: u64) -> Self {
        Self::DisplayUntil {
            display_until: unix_seconds,
        }
    }
}

/// Type of display element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ElementKind {
    Text(TextElement),
    Image(ImageElement),
    Animation(AnimationElement),
    Countdown(CountdownElement),
    Rectangle(RectangleElement),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextElement {
    /// Text content to display (printable ASCII only; fonts are bitmap ASCII)
    pub text: Text,
    /// One of the available fonts to display the text in
    pub font: Font,
    /// Color to display the text in, in #RRGGBBAA format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Width of the label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    /// Scroll rate in pixels per minute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_rate: Option<u32>,
    /// Delay in milliseconds before the scroll animation begins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_start_delay: Option<u32>,
    /// Pause duration in milliseconds between successive scroll cycles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_repeat_delay: Option<u32>,
}

impl TextElement {
    pub fn new(text: impl TryIntoValue<Text>, font: Font) -> Result<Self, InvalidValue> {
        Ok(Self {
            text: text.try_into_value()?,
            font,
            color: None,
            width: None,
            scroll_rate: None,
            scroll_start_delay: None,
            scroll_repeat_delay: None,
        })
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn width(mut self, pixels: u16) -> Self {
        self.width = Some(pixels);
        self
    }

    pub fn scroll_rate(mut self, pixels_per_minute: u32) -> Self {
        self.scroll_rate = Some(pixels_per_minute);
        self
    }

    pub fn scroll_start_delay_ms(mut self, milliseconds: u32) -> Self {
        self.scroll_start_delay = Some(milliseconds);
        self
    }

    pub fn scroll_repeat_delay_ms(mut self, milliseconds: u32) -> Self {
        self.scroll_repeat_delay = Some(milliseconds);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageSource {
    Asset { path: AssetPath },
    Stock { stock_path: StockPath },
}

impl ImageSource {
    pub fn asset(path: impl TryIntoValue<AssetPath>) -> Result<Self, InvalidValue> {
        Ok(Self::Asset {
            path: path.try_into_value()?,
        })
    }

    pub fn stock(stock_path: impl TryIntoValue<StockPath>) -> Result<Self, InvalidValue> {
        Ok(Self::Stock {
            stock_path: stock_path.try_into_value()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageElement {
    #[serde(flatten)]
    pub source: ImageSource,
    /// Opacity of the image in percentage (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<Opacity>,
}

impl ImageElement {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            opacity: None,
        }
    }

    pub fn asset(path: impl TryIntoValue<AssetPath>) -> Result<Self, InvalidValue> {
        Ok(Self::new(ImageSource::asset(path)?))
    }

    pub fn stock(stock_path: impl TryIntoValue<StockPath>) -> Result<Self, InvalidValue> {
        Ok(Self::new(ImageSource::stock(stock_path)?))
    }

    pub fn opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationElement {
    #[serde(flatten)]
    pub source: ImageSource,
    /// Whether to loop the requested part of the animation
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub repeat: Option<bool>,
    /// If the element has been created before and this flag is true, the previous range will
    /// finish before the requested one starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_previous_end: Option<bool>,
    /// Name of the section to play back. Specifying "default" selects the entire animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Opacity of the animated image in percentage (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<Opacity>,
}

impl AnimationElement {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            repeat: None,
            await_previous_end: None,
            section: None,
            opacity: None,
        }
    }

    pub fn asset(path: impl TryIntoValue<AssetPath>) -> Result<Self, InvalidValue> {
        Ok(Self::new(ImageSource::asset(path)?))
    }

    pub fn stock(stock_path: impl TryIntoValue<StockPath>) -> Result<Self, InvalidValue> {
        Ok(Self::new(ImageSource::stock(stock_path)?))
    }

    pub fn repeat(mut self, repeat: bool) -> Self {
        self.repeat = Some(repeat);
        self
    }

    pub fn await_previous_end(mut self, await_previous_end: bool) -> Self {
        self.await_previous_end = Some(await_previous_end);
        self
    }

    pub fn section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    pub fn opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountdownElement {
    /// Seconds-based Unix UTC timestamp to count down or up to. Note: it's a number in a
    /// string.
    #[serde(with = "crate::serde_util::string_u64")]
    pub timestamp: u64,
    /// Color to display the text in, in #RRGGBBAA format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Whether to count up or down
    pub direction: CountdownDirection,
    /// When to show the hours position
    pub show_hours: ShowHours,
}

impl CountdownElement {
    pub fn new(unix_seconds: u64, direction: CountdownDirection, show_hours: ShowHours) -> Self {
        Self {
            timestamp: unix_seconds,
            color: None,
            direction,
            show_hours,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RectangleElement {
    /// Width of the rectangle in pixels
    pub width: u16,
    /// Height of the rectangle in pixels
    pub height: u16,
    /// Corner radius of the rectangle in pixels (0 for sharp corners)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u16>,
    /// Fill style of the rectangle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fill>,
    /// Colors used for filling the rectangle. For solid fill, provide one color. For gradient
    /// fill, provide two colors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_colors: Option<Vec<Color>>,
    /// Width of the rectangle border in pixels (0 for no border)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<u16>,
    /// Color of the rectangle border in #RRGGBBAA format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<Color>,
}

impl RectangleElement {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            radius: None,
            fill: None,
            fill_colors: None,
            border_width: None,
            border_color: None,
        }
    }

    pub fn radius(mut self, pixels: u16) -> Self {
        self.radius = Some(pixels);
        self
    }

    pub fn solid(mut self, color: Color) -> Self {
        self.fill = Some(Fill::Solid);
        self.fill_colors = Some(vec![color]);
        self
    }

    pub fn horizontal_gradient(mut self, from: Color, to: Color) -> Self {
        self.fill = Some(Fill::GradientH);
        self.fill_colors = Some(vec![from, to]);
        self
    }

    pub fn vertical_gradient(mut self, from: Color, to: Color) -> Self {
        self.fill = Some(Fill::GradientV);
        self.fill_colors = Some(vec![from, to]);
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill = Some(Fill::None);
        self.fill_colors = None;
        self
    }

    pub fn border(mut self, width: u16, color: Color) -> Self {
        self.border_width = Some(width);
        self.border_color = Some(color);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Font {
    Tiny,
    Small,
    Normal,
    Condensed,
    Bold,
    Large,
    ExtraLarge,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    Center,
    MidRight,
    BottomLeft,
    BottomMid,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Screen {
    Front,
    Back,
}

impl Screen {
    pub fn index(self) -> u8 {
        match self {
            Screen::Front => 0,
            Screen::Back => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fill {
    None,
    Solid,
    GradientH,
    GradientV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountdownDirection {
    TimeLeft,
    TimeSince,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShowHours {
    WhenNonZero,
    Always,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct DisplayBrightnessInfo {
    /// Displays brightness (0-100/auto)
    pub value: crate::types::Brightness,
}
