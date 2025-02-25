use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::Add;
use std::time::Instant;

use glfw::{Action, Key, Modifiers, MouseButton};
use rand::{Rng, thread_rng};

use crate::components::context::context;
use crate::components::framework::animation::{AnimationRef, AnimationRegistry, AnimationType};
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::render::color::Color;
use crate::components::render::font::FontRenderData;
use crate::components::render::font::format::{FormatItem, Text};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::gl_binds::gl11::Translatef;

const CHUNK_SIZE: usize = 1024; // 1024*2
// static SHIFT_MAP: HashMap<char, char> = ;

pub fn get_shifted(c: char) -> char {
    let shifted: HashMap<char, char> = [
        ('1', '!'), ('2', '@'), ('3', '#'), ('4', '$'), ('5', '%'), ('6', '^'), ('7', '&'), ('8', '*'), ('9', 's'), ('0', ')'), ('=', '+'),
        ('-', '_'), ('`', '~'), (',', '<'), ('.', '>'), (';', ':'), ('/', '?'), ('\'', '"'), ('[', '{'), (']', '}'), ('\\', '|')
    ].iter().cloned().collect();
    *shifted.get(&c).unwrap_or(&c.to_ascii_uppercase())
}

#[derive(Clone, Debug)]
pub enum Change {
    Delete,
    Add(String),
}

#[derive(Debug)]
pub struct Chunk {
    str: String,
    updated: String,
}

impl Chunk {
    pub fn new(str: String) -> Self {
        let mut c = Chunk {
            str,
            updated: "".to_string(),
        };
        c
    }

    pub fn calculate_changes(&mut self, changes: &mut HashMap<usize, (usize, usize, Change)>, info: &ChunkInfo)  {
        let mut new = String::new();

        let mut i = info.ind_start;
        let mut skip = 0;
        for c in self.str.chars() {
            match changes.remove(&i) {
                None => {},
                Some((length, _, change)) => {
                    skip = length;
                    match change {
                        Change::Delete => {
                            skip = skip.max(1);
                        }
                        Change::Add(add) => {
                            new.push_str(&add);
                            // new.push(c);
                        }
                    }
                }
            }
            if skip == 0 {
                new.push(c)
            } else {
                skip -= 1;
            }
            i += 1;
        }
        self.updated = new;
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.updated, &mut self.str);
        self.updated = String::new();
    }
}

#[derive(Debug)]
pub struct Cursor {
    pos: Vec2<usize>,
    select_pos: Vec2<usize>,
}

impl Cursor {
    pub fn new(position: Vec2<usize>) -> Self {
        Cursor {
            pos: position.clone(),
            select_pos: position,
        }
    }


    pub fn start_pos(&self) -> Vec2<usize> {
        self.pos.min(self.select_pos)
    }

    pub fn end_pos(&self) -> Vec2<usize> {
        self.pos.max(self.select_pos)
    }

    pub fn position(&mut self, pos: impl Into<Vec2<usize>>, expand: bool) {
        let pos = pos.into();
        if expand {
            self.pos = pos;
        } else {
            self.pos = pos.clone();
            self.select_pos = pos;
        }
    }

    pub fn left(&mut self, expand: bool) {
        self.position((self.pos.x()-1, self.pos.y()), expand);
    }
    pub fn right(&mut self, expand: bool) {
        self.position((self.pos.x()+1, self.pos.y()), expand);
    }
    pub fn down(&mut self, expand: bool) {
        self.position((self.pos.x(), self.pos.y()+1), expand);
    }
    pub fn up(&mut self, expand: bool) {
        self.position((self.pos.x(), self.pos.y()-1), expand);
    }

}

#[derive(Debug)]
pub struct ChunkInfo {
    ind_start: usize,
    ind_end: usize,

    start: Vec2<usize>,
    end: Vec2<usize>,

    lines: Vec<(usize, usize)>,
}

impl ChunkInfo {
    pub fn new(chunk: &Chunk, index: usize, start: Vec2<usize>) -> Self {
        let mut inf = ChunkInfo {
            ind_start: 0,
            ind_end: 0,
            start: Default::default(),
            end: Default::default(),
            lines: vec![],
        };
        // TODO optimize this so that creating the editor doesn't take so long
        inf.update_info(chunk, index, start);
        inf
    }

    pub fn update_info(&mut self, chunk: &Chunk, start_index: usize, start: Vec2<usize>) {
        self.ind_start = start_index;
        self.ind_end = start_index+chunk.str.len();

        self.start = start;
        self.end = start;

        let mut line_start_ind = start_index;
        let mut i = start_index;
        for c in chunk.str.chars() {
            if c == '\n' {
                self.lines.push((line_start_ind, i));
                line_start_ind = i;
                self.end.y += 1;
                self.end.x = 0;
            }
            self.end.x += 1;
            i += 1;
        }
        self.lines.push((line_start_ind, i));
        // self.end.y += 1;
    }

    pub fn ind_of_pos(&self, pos: Vec2<usize>) -> usize {
        let line = pos.y();
        let local_line = line - self.start.y;
        let (_, ln_end) = self.lines[local_line.min(self.lines.len().max(1)-1)];

        (pos.x()).min(ln_end) + local_line
    }
}

#[derive(Debug)]
pub struct Editor {
    cursors: Vec<Cursor>,
    chunks: Vec<Chunk>,
    chunk_info: Vec<ChunkInfo>,
    changes: HashMap<usize, (usize, usize, Change)>,
}

impl Editor {

    pub fn new<T: AsRef<str>>(str: T) -> Self {
        let str = str.as_ref();
        let mut editor = Editor {
            cursors: vec![],
            chunks: vec![],
            chunk_info: vec![],
            changes: Default::default(),
        };

        let mut i = 0;
        let mut pos = Vec2::new(0,0);
        while i < str.len() {
            let chunk_str = &str[i..(i+CHUNK_SIZE).min(str.len())];
            let chunk = Chunk::new(chunk_str.to_string());

            let chunk_info = ChunkInfo::new(&chunk, i, pos.clone());
            pos = chunk_info.end.clone();

            editor.chunks.push(chunk);
            editor.chunk_info.push(chunk_info);

            i += CHUNK_SIZE;
        }

        editor
    }


    pub fn add_cursor(&mut self, pos: impl Into<Vec2<usize>>) {
        self.cursors.push(Cursor::new(pos.into()));
    }

    pub fn pos_index(&self, pos: impl Into<Vec2<usize>>) -> (usize, usize) {
        let pos = pos.into();

        let mut index = 0;
        let mut i = 0;
        for ci in &self.chunk_info {
            if pos.y() == ci.start.y && pos.x() >= ci.start.x && pos.x() < ci.end.x ||
                pos.y() > 0 && pos.y() > ci.start.y && (pos.y() < ci.end.y-1 || (pos.y() == ci.end.y-1 && pos.x() < ci.end.x)) {
                index = ci.ind_of_pos(pos);
                break;
            }
            i += 1;
        }

        (index, i)
    }

    pub fn add_change(&mut self, change: Change) {
        for c in &self.cursors {
            let (start_index, start_chunk) = self.pos_index(c.start_pos());
            let (end_index, end_chunk) = self.pos_index(c.end_pos());

            for chunk_index in start_chunk..=end_chunk {
                let chunk_info = &self.chunk_info[chunk_index];
                let start_index = start_index.max(chunk_info.ind_start);
                let end_index = end_index.min(chunk_info.ind_end);

                let chunk_change = match &change {
                    Change::Delete => Change::Delete,
                    Change::Add(str) => {
                        let mut sub = String::new();
                        let mut i = 0;
                        for c in str.chars() {
                            if i >= start_index && i < end_index {
                                sub.push(c);
                            }
                            i += 1;
                        }
                        Change::Add(sub)
                    }
                };

                self.changes.insert(start_index, (end_index - start_index, chunk_index, chunk_change));
            }
        }
    }

    pub fn apply_changes(&mut self) {
        let mut changed_chunks = vec![];
        for (_, (_, chunk, _)) in &mut self.changes {
            changed_chunks.push(*chunk);
        }

        for chunk in &changed_chunks {
            let ci = &self.chunk_info[*chunk];
            self.chunks[*chunk].calculate_changes(&mut self.changes, ci);
        }

        let (mut ind, mut pos) = (0, Vec2::new(0,0));
        for chunk in changed_chunks {
            let ci = &mut self.chunk_info[chunk];
            let c = &mut self.chunks[chunk];
            c.update();
            ci.update_info(c, ind, pos.clone());
            ind = ci.ind_end;
            pos = ci.end;
        }
    }
}

pub struct Textbox {
    editor: Editor,

}

impl Textbox {

}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        false
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        true
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        None
    }
}

// #[test]
pub fn editor() {
    let mut str = "THIS IS A NOT SO LONG STIRNG\nAND THIS is another line";
    let mut s = String::new();
    println!("compiuling");
    for i in 0..(1_000_000_000/str.len()) {
        if i % 10000 == 0 {
            println!("{i}")
        }
        s.push_str(str);
    }
    println!("done");
    let st = Instant::now();
    let mut editor = Editor::new(&s);
    let d = st.elapsed();
    println!("editor {:?}", d);

    let add = "this is a not so long";
    editor.add_cursor((0,0));
    editor.cursors[0].position((add.len(),0), true);

    // println!("{:?}", editor);
    editor.add_change(Change::Add(add.to_string()));
    // println!("{:?}", editor);
    let st = Instant::now();
    editor.apply_changes();
    let d = st.elapsed();
    // println!("{:?}", editor);

    for c in editor.chunks {
        print!("{}", c.str);
        break;
    }
    println!();
    println!("{:?} {}", d, s.len());
}