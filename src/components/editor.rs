use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::rc::Rc;
use std::time::Instant;
use glfw::{Action, Key, Modifiers, MouseButton};
use rand::{Rng, thread_rng};
use winapi::um::winuser::GetKeyboardState;
use crate::components::spatial::vec4::Vec4;
use crate::components::context::context;
use crate::components::framework::animation::{Animation, AnimationRef, AnimationRegistry, AnimationType};
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::spatial::vec2::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::format::{DefaultFormatter, FormatItem, FormattedText};
use crate::components::render::font::renderer::FontRenderer;
use crate::gl_binds::gl11::Translatef;

const CHUNK_SIZE: usize = 256; // 1024*2
// static SHIFT_MAP: HashMap<char, char> = ;

pub fn get_shifted(c: char) -> char {

    let shifted: HashMap<char, char> = [
        ('1', '!'), ('2', '@'), ('3', '#'), ('4', '$'), ('5', '%'), ('6', '^'), ('7', '&'), ('8', '*'), ('9', 's'), ('0', ')'), ('=', '+'),
        ('-', '_'), ('`', '~'), (',', '<'), ('.', '>'), (';', ':'), ('/', '?'), ('\'', '"'), ('[', '{'), (']', '}'), ('\\', '|')
    ].iter().cloned().collect();
    *shifted.get(&c).unwrap_or(&c.to_ascii_uppercase())
}

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
            // println!("ins");
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
                    // println!("I {:?} C {:?}", i, changes);
                    apply_changes_at(&changes, i, &self.string, &mut new);
                }
            }
            i += 1;
        }
        for (k, v) in &self.changes {
            apply_changes_at(v, *k, &self.string, &mut new);
        }
        self.changes.clear();
        self.changes.shrink_to_fit();
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
        let length = string.chars().count();
        loop {
            let end_index = (i+CHUNK_SIZE).min(length);
            let mut sub = String::new();
            for (ind, char) in string.char_indices() {
                if ind < i {
                    continue
                }
                if ind >= end_index {
                    break
                }
                sub.push(char);
            }
            // let sub = string.char.get(i..end_index).unwrap().to_string();
            // changed.insert(chunks.len());
            chunks.push(Chunk::new((i, end_index), sub));

            if end_index == length {
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
        // println!("c_index {c_index}");
        let chunk = self.chunks.get_mut(c_index).unwrap();
        chunk.apply();
    }

    pub fn apply_all_changes(&mut self) -> Vec<usize> {
        let changed = std::mem::take(&mut self.changed);
        let mut min_changed = self.chunks.len().max(1)-1;
        for c in &changed {
            self.apply_changes(*c);
            min_changed = min_changed.min(*c);
        }
        let mut min_added = self.chunks.len().max(1)-1;
        let mut changed_rev: Vec<usize> = changed.iter().map(|v| *v).collect();
        changed_rev.sort();
        let mut total_changes = Vec::new();
        for c in changed_rev.iter() {
            let c = *c;
            let chunk = match self.chunks.get_mut(c) {
                None => continue,
                Some(v) => v,
            };
            total_changes.push(c);
            if chunk.string.len() > CHUNK_SIZE {
                let (start, mid, end) = (chunk.range.0, (chunk.range.0 + chunk.range.1) / 2, chunk.range.1);
                let mut chunk0 = Chunk::new((start, mid), chunk.string.get(0..(chunk.string.len()/2)).unwrap().to_string());
                let chunk1 = Chunk::new((mid, end), chunk.string.get((chunk.string.len()/2)..chunk.string.len()).unwrap().to_string());
                std::mem::swap(chunk, &mut chunk0); // replaced the old chunk in the list with chunk0
                min_added = min_added.min(c+1);
                total_changes.push(c+1);
                self.chunks.insert(c+1, chunk1);
            }
            else if chunk.string.len() == 0 && self.chunks.len() > 1 {
                min_added = min_added.min(c+1);
                self.chunks.remove(c);
                self.chunks.get_mut(0).unwrap().range.0 = 0;
            }
        }
        let mut index = self.chunks.get(min_changed.min(self.chunks.len().max(1)-1)).unwrap().range.0;
        for i in min_changed..self.chunks.len() {
            let c = self.chunks.get_mut(i).unwrap();
            c.range = (index, index + c.string.len());
            index += c.string.len();
        }
        for i in min_added..self.chunks.len() {
            total_changes.push(i);
        }
        self.changed.clear();
        self.chunks.shrink_to_fit();
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
                new.push_str(&addition);
                add_char_at(index, current, new);
                // println!("added {}", addition);
            }
        }
    }
}

fn add_char_at(index: usize, current: &String, new: &mut String) {
    if index < current.len() && index >= 0 {
        new.push(current.chars().nth(index).unwrap());
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
    text_chunks: Vec<(FormattedText, Vec2, f32)>,
    animations: AnimationRegistry,
    changed: bool,
    index: usize,
    scroll: AnimationRef,
    font_size: AnimationRef,
    holding_ctrl: bool,
}

impl Textbox {
    pub unsafe fn new(fr: FontRenderer, init: String) -> Self {
        // println!("EDIT {:?}", init.chars());
        let mut anims = AnimationRegistry::new();
        let font_size = anims.new_anim();
        font_size.borrow_mut().set_target(16.0);
        let scroll = anims.new_anim();
        Textbox {
            editor: StringEditor::new(init.clone()),
            fr,
            text_chunks: Vec::new(),
            animations: anims,
            changed: true,
            index: 0,
            scroll,
            font_size,
            holding_ctrl: false,
        }
    }

    pub unsafe fn update_chunk(&mut self, index: usize) -> u32 {
        let last_offset = if index > 0 {
            match self.text_chunks.get(index - 1) {
                None => return 1,
                Some(c) => c.1,
            }
        } else { (0, 0).into() };

        if index < self.editor.chunks.len() {
            // println!("{} {} {}", index, last_offset.y + self.scroll.value(), context().window().height as f32);
            if last_offset.y + self.scroll.borrow().value() > context().window().height as f32 + 200f32 {
                return 2;
            }

            let mut text = FormattedText::new();
            // text.push(FormatItem::Offset(last_offset));
            text.push(FormatItem::Size(self.font_size.borrow().value()));
            text.push(FormatItem::Color(Color::from_hsv(thread_rng().random::<f32>(), 1.0, 1.0)));
            text.push(FormatItem::Text(self.editor.chunks().get(index).unwrap().string.clone()));

            //self.fr.add_end_pos(last_offset, self.font_size.borrow().value(), &self.editor.chunks().get(index).unwrap().string)
            let (pos, _) = self.fr.draw_string_o(text.clone(), (0, 0), last_offset);

            let mut chunk = (
                text,
                pos,
                self.font_size.borrow().value(),
            );

            if index < self.text_chunks.len() {
                std::mem::swap(self.text_chunks.get_mut(index).unwrap(), &mut chunk);
            } else {
                self.text_chunks.push(chunk);
            }
        } else {
            self.text_chunks.pop();
        }
        return 0;
    }

    pub unsafe fn update_loaded(&mut self) {
        for i in 0..self.text_chunks.len() {
            self.update_chunk(i);
        }
    }
}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::PreRender => {
                // println!("{} {}", self.scroll.target(), self.scroll.value());
                self.scroll.borrow_mut().animate(10.0, AnimationType::Sin);
                self.font_size.borrow_mut().animate(2.0, AnimationType::Sin);
                false
            },
            Event::PostRender => {
                self.changed = false;
                false
            }
            Event::MouseClick(button, action) => {
                let scroll_amount = self.scroll.borrow().value();
                match button {
                    MouseButton::Button1 => {
                        if action == &Action::Press {
                            let start_pos: Vec2 = (10, 100.0 + self.scroll.borrow().value() * (self.font_size.borrow().value() / 16.0)).into();
                            let mut last_offset: Vec2 = (0, 0).into();
                            // println!("{:?} {:?}", self.text_chunks.len(), start_pos);
                            let mut to_update = Vec::new();
                            let mut i = 0;
                            for (text, t_pos, size) in &self.text_chunks {
                                if t_pos.y + start_pos.y < 0.0 {
                                    last_offset = (t_pos.x, t_pos.y).into();
                                    i += 1;
                                    continue;
                                }

                                let (offset, vec4) = self.fr.draw_string_o(text.clone(), start_pos, last_offset);

                                if context().window().mouse().pos().intersects(&vec4) {
                                    // println!("clicked on {:?}", text);
                                    to_update.push(i);
                                    let chunk = self.editor.chunks.get(i).unwrap();
                                    for c in chunk.string.chars() {

                                    }
                                }

                                last_offset = (offset.x, last_offset.y + offset.y).into();

                                if t_pos.y + start_pos.y > context().window().height as f32 + 100f32 {
                                    break;
                                }
                                i += 1;
                            }
                            for i in to_update {
                                self.update_chunk(i);
                            }
                        }
                    }
                    MouseButton::Button2 => {}
                    _ => {}
                }
                false
            }
            Event::Render(pass) => match pass {
                RenderPass::Main => {
                    let start_pos: Vec2 = (10, 100.0 + self.scroll.borrow().value() * (self.font_size.borrow().value() / 16.0)).into();
                    let mut last_offset: Vec2 = (0, 0).into();
                    // println!("{:?} {:?}", self.text_chunks.len(), start_pos);
                    let mut to_update_indices = Vec::new();
                    let mut i = 0;
                    for (text, t_pos, size) in &self.text_chunks {
                        if t_pos.y + start_pos.y < 0.0 {
                            last_offset = (t_pos.x, t_pos.y).into();
                            i += 1;
                            continue;
                        }
                        if *size != self.font_size.borrow().value() {
                            to_update_indices.push(i);
                        }
                        let (offset, _) = self.fr.draw_string_o(text.clone(), start_pos, last_offset);

                        last_offset = (offset.x, last_offset.y + offset.y).into();

                        if t_pos.y + start_pos.y > context().window().height as f32 + 100f32 {
                            break;
                        }
                        i += 1;
                    }
                    for index in to_update_indices {
                        self.update_chunk(index);
                    }

                    // load any chunks that have come into screen
                    loop {
                        match self.text_chunks.last() {
                            None => break,
                            Some(v) => {
                                if self.text_chunks.len() == self.editor.chunks.len() {
                                    break;
                                }

                                if v.1.y + start_pos.y < context().window().height as f32 + 10f32 {
                                    self.update_chunk(self.text_chunks.len());
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    let current_chunk = self.editor.search_chunk(self.index);
                    let last_chunk_offset = if current_chunk != 0 {
                        match self.text_chunks.get(self.editor.search_chunk(self.index) - 1) {
                            Some((_, offset, _)) => offset.clone(),
                            None => (0, 0).into()
                        }
                    } else {
                        (0, 0).into()
                    };

                    let current = self.editor.chunks().get(self.editor.search_chunk(self.index)).unwrap();
                    let cursor_pos = self.fr.add_end_pos(last_chunk_offset, self.font_size.borrow().value(), current.get_to_index(self.index));

                    Translatef(0.0, self.scroll.borrow().value() * (self.font_size.borrow().value() / 16.0), 0.0);
                    // println!("current ch sec {:?}", current.get_to_index(self.index));
                    context().renderer().draw_rect(Vec4::xywh(10.0 + cursor_pos.x() + 3.0, 100.0 + cursor_pos.y(), 1.0, self.fr.get_sized_height(self.font_size.borrow().value())), 0xffff20a0);

                    Translatef(0.0, -self.scroll.borrow().value() * (self.font_size.borrow().value() / 16.0), 0.0);
                    true
                },
                _ => false
            },
            Event::Keyboard(key, action, mods) => {
                // println!("{:?} {:?} {:?} {:?} {:?}", key, action, mods, key.get_scancode(), key.get_name());
                match key {
                    Key::LeftControl => self.holding_ctrl = action != &Action::Release,
                    _ => {}
                }
                if action == &Action::Release {
                    return false;
                }
                // for (ind, ch) in self.editor.chunks[0].string.char_indices() {
                //     println!("IND {ind}, {ch}");
                // }
                match key.get_name() {
                    None => {
                        match key {
                            Key::Left => if self.index > 0 {
                                self.index = self.index.max(1) - 1;
                            },
                            Key::Right => if self.index < self.editor.str_len() {
                                self.index = self.index + 1;
                            },
                            Key::Backspace => {
                                if self.index > 0 {
                                    self.index = self.index - 1;
                                    self.editor.add_change(self.index, Change::Delete);
                                }
                            },
                            Key::Delete => {
                                for i in 0..1000 {
                                    self.editor.add_change(self.index + i, Change::Delete);
                                }
                            },
                            Key::Enter => {
                                self.editor.add_change(self.index, Change::Add("\n".to_string()));
                                self.index += 1;
                            }
                            Key::Space => {
                                self.editor.add_change(self.index, Change::Add(" ".to_string()));
                                self.index += 1;
                            }
                            _ => {}
                        }
                    }
                    Some(pressed) => {
                        let char = pressed.as_bytes()[0] as char;
                        let char = if mods.contains(Modifiers::Shift) {
                            get_shifted(char)
                        } else { char };
                        self.editor.add_change(self.index, Change::Add(char.to_string()));
                        self.index += 1;
                    }
                }
                println!("INDEX {}", self.index);
                let st = Instant::now();
                // println!("{:?}", self.editor);
                // println!("0");
                let mut changed = self.editor.apply_all_changes();
                // println!("1");
                for i in self.text_chunks.len()..self.editor.chunks.len() {
                    changed.push(i);
                }
                // println!("2");
                for i in self.editor.chunks.len()..self.text_chunks.len() {
                    self.text_chunks.pop();
                }
                // println!("3");
                println!("CHANGED {:?}", changed);
                for c in &changed {
                    self.changed = true;
                    match self.update_chunk(*c) {
                        1 => continue,
                        2 => break,
                        _ => {}
                    }
                }
                println!("applied in {:?} {:?} {:?}", st.elapsed(), self.editor.chunks.len(), self.editor.current_chunk);
                // println!("{:?}", self.editor);
                // println!("{:?}", self.text_chunks);
                true
            },
            Event::Scroll(_, y) => {
                if self.holding_ctrl {
                    let mut anim = self.font_size.borrow_mut();
                    let last_target = anim.target();
                    anim.set_target((y + last_target).max(1.0));
                } else {
                    let mut anim = self.scroll.borrow_mut();
                    let last_target = anim.target();
                    anim.set_target(y * 100.0 + last_target);
                }
                true
            }
            _ => false,
        }
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        self.changed
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        Some(&mut self.animations)
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