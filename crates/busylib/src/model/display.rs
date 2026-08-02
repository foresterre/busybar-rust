use serde::{Deserialize, Serialize};

use crate::types::{
    AppName, AssetPath, Color, ElementId, InvalidValue, Opacity, Priority, StockPath, Text,
    TryIntoValue,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayElements {
    pub application_name: AppName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_notification_color: Option<Color>,
    pub elements: Vec<Element>,
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

    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }

    pub fn elements(mut self, elements: impl IntoIterator<Item = Element>) -> Self {
        self.elements.extend(elements);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: ElementId,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<Lifetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Screen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    #[serde(flatten)]
    pub kind: ElementKind,
}

impl Element {
    pub fn builder(id: impl TryIntoValue<ElementId>) -> Result<ElementBuilder, InvalidValue> {
        Ok(ElementBuilder {
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
pub struct ElementBuilder {
    id: ElementId,
    lifetime: Option<Lifetime>,
    x: Option<i16>,
    y: Option<i16>,
    display: Option<Screen>,
    align: Option<Align>,
}

impl ElementBuilder {
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

    pub fn text(self, text: TextElement) -> Element {
        self.finish(ElementKind::Text(text))
    }

    pub fn image(self, image: ImageElement) -> Element {
        self.finish(ElementKind::Image(image))
    }

    pub fn animation(self, animation: AnimationElement) -> Element {
        self.finish(ElementKind::Animation(animation))
    }

    pub fn countdown(self, countdown: CountdownElement) -> Element {
        self.finish(ElementKind::Countdown(countdown))
    }

    pub fn rectangle(self, rectangle: RectangleElement) -> Element {
        self.finish(ElementKind::Rectangle(rectangle))
    }

    fn finish(self, kind: ElementKind) -> Element {
        Element {
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
        timeout: u32,
    },
    DisplayUntil {
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
    pub text: Text,
    pub font: Font,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_start_delay: Option<u32>,
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
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub repeat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_previous_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
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
    #[serde(with = "crate::serde_util::string_u64")]
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub direction: CountdownDirection,
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
    pub width: u16,
    pub height: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_colors: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<u16>,
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
pub(crate) struct DisplayBrightnessResponse {
    pub value: crate::types::Brightness,
}
