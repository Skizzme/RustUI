use std::hash::{Hash, Hasher};
use crate::components::position::Pos;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::FormatItem::{Size, Text};

pub trait Formatter {
    fn parse(&mut self) -> bool;
    fn parsed(&self) -> &FormattedText;
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

impl<T: ToColor> Into<FormattedText> for (f32, String, T) {
    fn into(&self) -> FormattedText {
        let (size, text, color) = self;
        let color = color.to_color();
        let mut ft = FormattedText::new();
        ft.push(Size(*size));
        ft.push(FormatItem::Color(color));
        ft.push(Text(text.clone()));
        ft
    }
}

impl Into<FormattedText> for FormattedText {
    fn into(&self) -> FormattedText {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum FormatItem {
    Color(Color),
    Size(f32),
    Text(String),
    Offset(Pos),
    None,
}

impl Hash for FormatItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FormatItem::Color(v) | Text(v) | FormatItem::Offset(v) => v.hash(state),
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
        println!("{}", self.index);
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
        println!("FIN {:?}", current);
        self.parsed.push(current);

        self.next();

        true
    }
}

impl Formatter for DefaultFormatter {
    fn parse(&mut self) -> bool { // should only parse 1 token at a time
        loop {
            if self.char == '&' {
                self.finish(); // finish the possible previous token
                if self.char == 'f' {
                    self.current = FormatItem::Color(Color::from_u32(0xffffffff));
                    self.finish();
                    break;
                }
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

    fn parsed(&self) -> &FormattedText {
        &self.parsed
    }
}

#[test]
pub fn format() {
    let mut formatter = DefaultFormatter::new("thius is a test &f sting".to_string());
    while !formatter.parse() {}
    println!("{:?}", formatter.parsed);
}