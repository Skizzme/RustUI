use std::hash::{Hash, Hasher};

use num_traits::NumCast;

use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::FormatItem::{Offset, Size};
use crate::components::spatial::vec2::Vec2;

#[macro_export]
macro_rules! text {
    [$( $expr:expr ),* $(,)?] => {
        {
            let mut texts: Vec<Text> = Vec::new();
            $(
                texts.push($expr.into());
            )*
            let text: Text = texts.into();
            text
        }
    };
}

pub trait Formatter {
    fn parse(&mut self) -> bool;
    fn parsed(&self) -> &Text;
    fn parse_all(&mut self);
    fn set_text(&mut self, to_parse: String);
}

/// The simplest form of render-able text. Uses [`FormatItem`] to
/// split each item, including color, formatting, and actual text into
/// their own parts.
///
/// This is done as a "universal" form of rendering, allowing for implementations
/// of the [`Formatter`] to create this from any text format that exists.
#[derive(Debug, Clone, Hash)]
pub struct Text {
    items: Vec<FormatItem>,
    visible_length: usize,
    color_count: usize,
}

impl Text {
    pub fn new() -> Text {
        Text {
            items: vec![],
            visible_length: 0,
            color_count: 0,
        }
    }

    pub fn push(&mut self, item: FormatItem) {
        match &item {
            FormatItem::String(t) => self.visible_length += t.len(),
            FormatItem::Color(_) => self.color_count += 1,
            _ => {}
        }
        self.items.push(item);
    }

    pub fn append(&mut self, all: &Text) {
        for item in all.items() {
            self.push(item.clone())
        }
    }

    pub fn with_formatter<F: Formatter>(&self, formatter: &mut F) -> Text {
        let mut new_text = Text::new();
        for item in self.items.clone() {
            match item {
                FormatItem::String(text) => {
                    formatter.set_text(text);
                    formatter.parse_all();
                    new_text.append(formatter.parsed());
                }
                _ => new_text.push(item)
            }
        }

        new_text
    }

    pub fn visible_length(&self) -> usize {
        self.visible_length
    }

    pub fn color_count(&self) -> usize {
        self.color_count
    }

    pub fn items(&self) -> &Vec<FormatItem> {
        &self.items
    }
}

impl<S: NumCast, T: ToString, C: ToColor> Into<Text> for (S, T, C) {
    fn into(self) -> Text {
        let (size, text, color) = self;
        let color = color.to_color();
        let mut ft = Text::new();
        ft.push(Size(size.to_f32().unwrap()));
        ft.push(FormatItem::Color(color));

        ft.push(FormatItem::String(text.to_string()));
        ft
    }
}

impl Into<Text> for FormatItem {
    fn into(self) -> Text {
        let mut f = Text::new();
        f.push(self);
        f
    }
}

impl Into<Text> for &str {
    fn into(self) -> Text {
        let mut f = Text::new();
        f.push(FormatItem::String(self.to_string()));
        f
    }
}
impl Into<Text> for String {
    fn into(self) -> Text {
        let mut f = Text::new();
        f.push(FormatItem::String(self));
        f
    }
}

impl Into<Text> for Vec<FormatItem> {
    fn into(self) -> Text {
        let mut fm = Text::new();
        for item in self {
            fm.push(item.clone());
        }
        fm
    }
}

impl Into<Text> for Vec<Text> {
    fn into(self) -> Text {
        let mut fm = Text::new();
        for item in self {
            fm.append(&item);
        }
        fm
    }
}

#[derive(Debug, Clone, Default)]
pub enum Alignment {
    #[default]
    /// Value of 0.5
    Center,
    /// Value of 0
    Left,
    /// Value of 1.0
    Right,
    /// Value of 0
    Top,
    /// Value of 1.0
    Bottom,
    Custom(f32)
}

impl Alignment {
    pub fn get_value(&self) -> f32 {
        match self {
            Alignment::Top => 0.,
            Alignment::Bottom => 1.,
            Alignment::Left => 0.,
            Alignment::Center => 0.5,
            Alignment::Right => 1.,
            Alignment::Custom(v) => *v,
        }
    }
}

impl Hash for Alignment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Alignment::Left => state.write_u8(0),
            Alignment::Center => state.write_u8(1),
            Alignment::Right => state.write_u8(2),
            Alignment::Top => state.write_u8(3),
            Alignment::Bottom => state.write_u8(4),
            Alignment::Custom(v) => state.write(&v.to_be_bytes()),
        }
    }
}

/// Wrapping to be used for rendering
///
/// When not [Wrapping::None], the enum should contain the maximum line length (in pixels)
pub enum Wrapping {
    /// No wrapping
    None,
    /// Will wrap at any character, and could split words up
    Hard(f32),
    /// Will wrap only at spaces. Will not break up words
    Soft(f32),
    /// Will try to wrap only at spaces, but if one word is longer than the maximum line length, it would resort to hard wrapping
    SoftHard(f32),
}

#[derive(Debug, Clone)]
pub enum FormatItem {
    Color(Color),
    Size(f32),
    String(String),
    Offset(Vec2),
    AlignH(Alignment),
    AlignV(Alignment),
    TabLength(u32),
    LineSpacing(f32),
    None,
}

impl Into<FormatItem> for &str {
    fn into(self) -> FormatItem {
        FormatItem::String(self.to_string())
    }
}

impl Into<FormatItem> for String {
    fn into(self) -> FormatItem {
        FormatItem::String(self)
    }
}

impl Into<FormatItem> for Color {
    fn into(self) -> FormatItem {
        FormatItem::Color(self)
    }
}

impl Into<FormatItem> for f32 {
    fn into(self) -> FormatItem {
        Size(self)
    }
}

impl Into<FormatItem> for Vec2 {
    fn into(self) -> FormatItem {
        Offset(self)
    }
}

impl Hash for FormatItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FormatItem::Color(v) => v.hash(state),
            FormatItem::String(v) => v.hash(state),
            Offset(v) => v.hash(state),
            Size(v) => state.write(&v.to_be_bytes()),
            FormatItem::None => state.write(&[0u8]),
            FormatItem::AlignH(v) => v.hash(state),
            FormatItem::AlignV(v) => v.hash(state),
            FormatItem::TabLength(v) => v.hash(state),
            FormatItem::LineSpacing(v) => state.write(&v.to_be_bytes()),
        }
    }
}

pub struct DefaultFormatter {
    index: usize,
    char: char,
    current: FormatItem,
    raw: String,
    parsed: Text,
}

impl DefaultFormatter {
    pub fn new() -> DefaultFormatter {
        DefaultFormatter {
            index: 0,
            char: 'a',
            current: FormatItem::None,
            raw: String::new(),
            parsed: Text::new(),
        }
    }

    fn next(&mut self) -> char {
        if self.index + 1 >= self.raw.as_bytes().len() {
            return 0u8 as char;
        }
        self.index += 1;
        self.char = self.raw.as_bytes()[self.index] as char;
        self.char
    }

    fn finish(&mut self) -> bool {
        match self.current {
            FormatItem::None => {
                return false;
            }
            _ => {}
        }

        let mut current = FormatItem::None;
        std::mem::swap(&mut self.current, &mut current);
        self.parsed.push(current);

        true
    }
}

impl Formatter for DefaultFormatter {
    fn parse(&mut self) -> bool { // should only parse 1 token at a time
        if self.raw.len() == 0 {
            return true;
        }
        loop {
            if self.char == '&' {
                self.finish(); // finish the possible previous token
                self.next();
                let mut color = String::new();
                for _ in 0..8 {
                    color.push(self.char);
                    self.next();
                }

                self.current = FormatItem::Color(Color::from_u32(u32::from_str_radix(&color, 16).unwrap_or(0xff20ff20)));
                self.finish();

                break;
            } else {
                // TODO figure out the damn macros
                match &mut self.current {
                    FormatItem::String(ref mut text) => {
                        text.push(self.char)
                    }
                    FormatItem::None => {
                        self.current = FormatItem::String(self.char.to_string())
                    }
                    _ => {}
                }
            }
            if self.index >= self.raw.len()-1 {
                self.finish();
                break;
            }
            self.next();
        }

        self.index >= self.raw.len()-1
    }

    fn parse_all(&mut self) {
        while !self.parse() {}
    }

    fn parsed(&self) -> &Text {
        &self.parsed
    }

    fn set_text(&mut self, to_parse: String) {
        let char = match to_parse.len() {
            0 => 0x00 as char,
            _ => to_parse.as_bytes()[0] as char,
        };
        self.index = 0;
        self.char = char;
        self.current = FormatItem::None;
        self.raw = to_parse;
        self.parsed = Text::new();
    }
}

#[test]
pub fn format() {
    let mut formatter = DefaultFormatter::new("thius is a test &ff9020ff sting".to_string());
    formatter.parse_all();

    println!("{:?}", formatter.parsed);
}
