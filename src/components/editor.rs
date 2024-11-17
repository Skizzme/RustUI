use std::collections::{HashMap, HashSet};
use std::time::Instant;
use glfw::{Action, Key, Modifiers};
use rand::{Rng, thread_rng};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::element::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::position::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::{DefaultFormatter, FormatItem, FormattedText};
use crate::components::render::font::renderer::FontRenderer;

const CHUNK_SIZE: usize = 8;

#[derive(Debug)]
pub struct Chunk {
    string: String,
    range: (usize, usize),
    changes: HashMap<usize, Vec<Change>>,
}

impl Chunk {
    pub fn new(range: (usize, usize), string: String) -> Self {
        Chunk {
            string,
            range,
            changes: HashMap::new(),
        }
    }

    pub fn get_to_index(&self, str_index: usize) -> String {
        self.string.get(0..(str_index.max(self.range.0)-self.range.0)).unwrap_or("").to_string()
    }

    pub fn add_change(&mut self, index: usize, change: Change) {
        if !self.changes.contains_key(&index) {
            self.changes.insert(index, vec![]);
            println!("ins");
        }
        self.changes.get_mut(&index).unwrap().push(change);
    }

    pub fn apply(&mut self) {
        let mut new = String::new();
        let mut i = 0;
        for c in self.string.chars() {
            match self.changes.remove(&i) {
                None => add_char_at(i, &self.string, &mut new),
                Some(changes) => {
                    apply_changes_at(&changes, i, &self.string, &mut new);
                }
            }
            i += 1;
        }
        for (k, v) in &self.changes {
            // println!("{:?} {:?}", k, v);
            apply_changes_at(v, *k, &self.string, &mut new);
        }
        self.changes.clear();
        self.string = new;
        self.range.1 = self.range.0 + self.string.len();
    }
}

#[derive(Debug)]
pub struct StringEditor {
    chunks: Vec<Chunk>,
    changed: HashSet<usize>,
    current_chunk: usize,
}

impl StringEditor {
    pub fn new(string: String) -> Self {
        let mut chunks = Vec::new();
        let mut changed = HashSet::new();
        let mut i = 0;
        loop {
            let end_index = (i+CHUNK_SIZE).min(string.len());
            let sub = string.get(i..end_index).unwrap().to_string();
            changed.insert(chunks.len());
            chunks.push(Chunk::new((i, end_index), sub));

            if end_index == string.len() {
                break;
            }
            i += CHUNK_SIZE;
        }

        StringEditor {
            chunks,
            changed,
            current_chunk: 0,
        }
    }

    pub fn search_chunk(&self, str_index: usize) -> usize {
        for i in 0..self.chunks.len() {
            let c = &self.chunks.get(i).unwrap();
            let (min, max) = c.range;
            // println!("MIN {min} MAX {max} IND {str_index} CHU {i}");
            if str_index >= min && str_index < max {
                return i;
            }
        }
        // println!("NO FOUND");
        self.chunks.len()-1
    }

    pub fn set_chunk(&mut self, c_index: usize) {
        self.current_chunk = c_index;
    }

    pub fn add_change(&mut self, str_index: usize, change: Change) {
        self.set_chunk(self.search_chunk(str_index));
        // println!("IND {} CHUNK {}", str_index, self.current_chunk);
        let chunk = self.chunks.get_mut(self.current_chunk).unwrap();
        let index = str_index.max(chunk.range.0) - chunk.range.0;
        self.changed.insert(self.current_chunk);
        chunk.add_change(index, change);
    }

    pub fn apply_changes(&mut self, c_index: usize) {
        println!("c_index {c_index}");
        let chunk = self.chunks.get_mut(c_index).unwrap();
        chunk.apply();
    }

    pub fn apply_all_changes(&mut self) -> Vec<usize> {
        let changed = std::mem::take(&mut self.changed);
        for c in &changed {
            self.apply_changes(*c);
        }
        let mut changed_rev: Vec<usize> = changed.iter().map(|v| *v).collect();
        changed_rev.sort();
        let mut total_changes = Vec::new();
        for c in changed_rev.iter() {
            let c = *c;
            let chunk = self.chunks.get_mut(c).unwrap();
            total_changes.push(c);
            if chunk.string.len() > CHUNK_SIZE {
                let (start, mid, end) = (chunk.range.0, (chunk.range.0 + chunk.range.1) / 2, chunk.range.1);
                let mut chunk0 = Chunk::new((start, mid), chunk.string.get(0..(chunk.string.len()/2)).unwrap().to_string());
                let chunk1 = Chunk::new((mid, end), chunk.string.get((chunk.string.len()/2)..chunk.string.len()).unwrap().to_string());
                std::mem::swap(chunk, &mut chunk0); // replaced the old chunk in the list with chunk0
                total_changes.push(c+1);
                self.chunks.insert(c+1, chunk1);
            }
            else if chunk.string.len() == 0 && self.chunks.len() > 1 {
                self.chunks.remove(c);
            }
        }
        self.changed.clear();
        total_changes
    }

    pub fn string(&self) -> String {
        let mut str = String::new();
        for c in &self.chunks {
            str.push_str(&c.string);
        }
        str
    }

    pub fn str_len(&self) -> usize {
        self.chunks.last().unwrap().range.1
    }

    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

}

fn apply_changes_at(changes: &Vec<Change>, index: usize, current: &String, new: &mut String) {
    for change in changes {
        match change {
            Change::Delete => {}
            Change::Add(addition) => {
                add_char_at(index, current, new);
                println!("added {}", addition);
                new.push_str(&addition)
            }
        }
    }
}

fn add_char_at(index: usize, current: &String, new: &mut String) {
    if index < current.len() && index >= 0 {
        new.push(current.as_bytes()[index] as char);
    }
}

#[derive(Debug)]
pub enum Change {
    Delete,
    Add(String),
}

pub struct Textbox {
    editor: StringEditor,
    fr: FontRenderer,
    text_chunks: Vec<(FormattedText, Vec2)>,
    index: usize,
}

impl Textbox {
    pub fn new(fr: FontRenderer, init: String) -> Self {
        Textbox {
            editor: StringEditor::new(init.clone()),
            fr,
            text_chunks: Vec::new(),
            index: 0,
        }
    }
}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::Render(pass) => match pass {
                RenderPass::Main => {
                    let start_pos: Vec2 = (10, 100).into();
                    let mut last_offset: Vec2 = (0,0).into();
                    for (text, t_pos) in &self.text_chunks {
                        let offset: Vec2 = self.fr.draw_string_o(text.clone(), start_pos, last_offset).into();
                        last_offset = (offset.x, last_offset.y + offset.y).into();
                        // pos = self.fr.add_end_pos(pos, 16.0, text.clone());
                    }

                    let current_chunk = self.editor.search_chunk(self.index);
                    let last_chunk_offset = if current_chunk != 0 {
                        match self.text_chunks.get(self.editor.search_chunk(self.index)-1) {
                            Some((_, offset)) => offset.clone(),
                            None => (0, 0).into()
                        }
                    } else {
                        (0, 0).into()
                    };

                    let current = self.editor.chunks().get(self.editor.search_chunk(self.index)).unwrap();
                    let cursor_pos = self.fr.add_end_pos(last_chunk_offset, 16.0, current.get_to_index(self.index));

                    // println!("current ch sec {:?}", current.get_to_index(self.index));
                    context().renderer().draw_rect(Bounds::xywh(10.0 + cursor_pos.x() + 3.0, 100.0 + cursor_pos.y(), 1.0, self.fr.get_sized_height(16.0)), 0xffff20a0);
                    true
                },
                _ => false
            },
            Event::Keyboard(key, action, mods) => {
                println!("{:?} {:?} {:?} {:?} {:?}", key, action, mods, key.get_scancode(), key.get_name());
                if action == &Action::Release {
                    return false;
                }
                let current_index = self.index;
                let increment = match key.get_name() {
                    None => {
                        match key {
                            Key::Left => if self.index > 0 {
                                self.index = self.index-1;
                                false
                            } else { false },
                            Key::Right => if self.index < self.editor.str_len() {
                                self.index = self.index + 1;
                                false
                            } else { false },
                            Key::Backspace => {
                                if self.index > 0 {
                                    self.index = self.index-1;
                                    self.editor.add_change(current_index, Change::Delete);
                                }
                                false
                            },
                            Key::Delete => {
                                self.editor.add_change(current_index, Change::Delete);
                                false
                            },
                            Key::Enter => {
                                self.editor.add_change(current_index, Change::Add("\n".to_string()));
                                true
                            }
                            Key::Space => {
                                self.editor.add_change(current_index, Change::Add(" ".to_string()));
                                true
                            }
                            _ => false
                        }
                    }
                    Some(pressed) => {
                        let char = pressed.as_bytes()[0] as char;
                        let char = if mods.contains(Modifiers::Shift) {
                            char.to_ascii_uppercase()
                        } else { char };
                        self.editor.add_change(current_index, Change::Add(char.to_string()));
                        true
                    }
                };
                if increment {
                    self.index += 1;
                }
                let st = Instant::now();
                let changed = self.editor.apply_all_changes();
                println!("CHANGED {:?}", changed);
                for c in &changed {
                    let c = *c;
                    let last_offset = if c > 0 {
                        self.text_chunks.get(c-1).unwrap().1
                    } else { (0,0).into() };

                    if c < self.editor.chunks.len() {
                        let mut text = FormattedText::new();
                        // text.push(FormatItem::Offset(last_offset));
                        text.push(FormatItem::Size(16.0));
                        text.push(FormatItem::Color(Color::from_hsv(thread_rng().random::<f32>(), 1.0, 1.0)));
                        text.push(FormatItem::Text(self.editor.chunks().get(c).unwrap().string.clone()));

                        let mut pos = self.fr.add_end_pos(last_offset, 16.0, &self.editor.chunks().get(c).unwrap().string);

                        let mut chunk = (
                            text,
                            pos
                        );

                        if c < self.text_chunks.len() {
                            std::mem::swap(self.text_chunks.get_mut(c).unwrap(), &mut chunk);
                        } else {
                            self.text_chunks.push(chunk);
                        }
                    } else {
                        self.text_chunks.pop();
                    }
                }
                // println!("applied in {:?} {:?} {:?} {:?}", st.elapsed(), self.editor.chunks.len(), self.editor.current_chunk, changed);
                println!("{:?}", self.editor);
                println!("{:?}", self.text_chunks);
                true
            },
            _ => false,
        }
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        true
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        None
    }
}

#[test]
pub fn editor() {
    let t = include_str!("../../test_3.js").to_string();
    let mut editor = StringEditor::new(t);
    editor.set_chunk(0);
    let st = Instant::now();
    editor.add_change(1024, Change::Add("add36123ed".to_string()));
    let t1 = st.elapsed();
    let st = Instant::now();
    editor.apply_all_changes();
    let t2 = st.elapsed();
    // println!("{:?}", editor);
    println!("{:?} {:?}", t1, t2);
}