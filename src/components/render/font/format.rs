use std::hash::{Hash, Hasher};

use num_traits::NumCast;

use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::FormatItem::{Offset, Size, Text};
use crate::components::spatial::vec2::Vec2;

#[macro_export]
macro_rules! text_vec {
    [$( $expr:expr ),*] => {
        {
            let mut formatted: Vec<FormattedText> = Vec::new();
            $(
                formatted.push($expr.into());
            )*
            formatted.into()
        }
    };
}

pub trait Formatter {
    fn parse(&mut self) -> bool;
    fn parsed(&self) -> &FormattedText;
    fn parse_all(&mut self);
}

/// The simplest form of render-able text. Uses [`FormatItem`] to
/// split each item, including color, formatting, and actual text into
/// their own parts.
///
/// This is done as a "universal" form of rendering, allowing for implementations
/// of the [`Formatter`] to create this from any text format that exists.
#[derive(Debug, Clone, Hash)]
pub struct FormattedText {
    items: Vec<FormatItem>,
    visible_length: usize,
    color_count: usize,
}

impl FormattedText {
    pub fn new() -> FormattedText {
        FormattedText {
            items: vec![],
            visible_length: 0,
            color_count: 0,
        }
    }

    pub fn push(&mut self, item: FormatItem) {
        match &item {
            Text(t) => self.visible_length += t.len(),
            FormatItem::Color(_) => self.color_count += 1,
            _ => {}
        }
        self.items.push(item);
    }

    pub fn append(&mut self, all: &FormattedText) {
        for item in all.items() {
            self.push(item.clone())
        }
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

impl<S: NumCast, T: ToString, C: ToColor> Into<FormattedText> for (S, T, C) {
    fn into(self) -> FormattedText {
        let (size, text, color) = self;
        let color = color.to_color();
        let mut ft = FormattedText::new();
        ft.push(Size(size.to_f32().unwrap()));
        ft.push(FormatItem::Color(color));

        let mut fm = DefaultFormatter::new(text.to_string());
        fm.parse_all();
        ft.append(fm.parsed());
        ft
    }
}

impl Into<FormattedText> for Vec<FormatItem> {
    fn into(self) -> FormattedText {
        let mut fm = FormattedText::new();
        for item in self {
            fm.push(item.clone());
        }
        fm
    }
}

impl Into<FormattedText> for Vec<FormattedText> {
    fn into(self) -> FormattedText {
        let mut fm = FormattedText::new();
        for item in self {
            fm.append(&item);
        }
        fm
    }
}

#[derive(Debug, Clone, Default)]
pub enum AlignH {
    #[default]
    Left,
    Middle,
    Right,
    Custom(f32)
}

impl AlignH {
    pub fn get_value(&self) -> f32 {
        match self {
            AlignH::Left => 0.,
            AlignH::Middle => 0.5,
            AlignH::Right => 1.,
            AlignH::Custom(v) => *v,
        }
    }
}

impl Hash for AlignH {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            AlignH::Left => state.write_u8(0),
            AlignH::Middle => state.write_u8(1),
            AlignH::Right => state.write_u8(2),
            AlignH::Custom(v) => state.write(&v.to_be_bytes()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum AlignV {
    Top,
    #[default]
    Middle,
    Bottom,
    Custom(f32),
}

impl AlignV {
    pub fn get_value(&self) -> f32 {
        match self {
            AlignV::Top => 0.,
            AlignV::Middle => 0.5,
            AlignV::Bottom => 1.,
            AlignV::Custom(v) => *v,
        }
    }
}

impl Hash for AlignV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            AlignV::Top => state.write_u8(0),
            AlignV::Middle => state.write_u8(1),
            AlignV::Bottom => state.write_u8(2),
            AlignV::Custom(v) => state.write(&v.to_be_bytes()),
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
    Text(String),
    Offset(Vec2),
    AlignH(AlignH),
    AlignV(AlignV),
    TabLength(u32),
    LineSpacing(f32),
    None,
}

impl Into<FormatItem> for String {
    fn into(self) -> FormatItem {
        Text(self)
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
            Text(v) => v.hash(state),
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
    parsed: FormattedText,
}

impl DefaultFormatter {
    fn new(text: String) -> DefaultFormatter {
        let char = match text.len() {
            0 => 0x00 as char,
            _ => text.as_bytes()[0] as char,
        };
        DefaultFormatter {
            index: 0,
            char: char,
            current: FormatItem::None,
            raw: text,
            parsed: FormattedText::new(),
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
                    Text(ref mut text) => {
                        text.push(self.char)
                    }
                    FormatItem::None => {
                        self.current = Text(self.char.to_string())
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

    fn parsed(&self) -> &FormattedText {
        &self.parsed
    }
}

#[test]
pub fn format() {
    let mut formatter = DefaultFormatter::new("thius is a test &ff9020ff sting".to_string());
    formatter.parse_all();

    println!("{:?}", formatter.parsed);
}
