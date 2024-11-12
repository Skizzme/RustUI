use std::hash::{Hash, Hasher};

use num_traits::{NumCast};

use crate::components::position::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::FormatItem::{Offset, Size, Text};

pub trait Formatter {
    fn parse(&mut self) -> bool;
    fn parsed(&self) -> &FormattedText;
    fn parse_all(&mut self);
}

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

#[derive(Debug, Clone)]
pub enum FormatItem {
    Color(Color),
    Size(f32),
    Text(String),
    Offset(Vec2),
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
            FormatItem::None => state.write(&[0u8])
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
        DefaultFormatter {
            index: 0,
            char: text.as_bytes()[0] as char,
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
        loop {
            if self.char == '&' {
                self.finish(); // finish the possible previous token
                self.next();
                let mut color = String::new();
                for _ in 0..8 {
                    color.push(self.char);
                    self.next();
                }

                self.current = FormatItem::Color(Color::from_u32(u32::from_str_radix(&color, 16).unwrap()));
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