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
use crate::text;

const CHUNK_SIZE: usize = 2; // 1024*2
// static SHIFT_MAP: HashMap<char, char> = ;

pub fn get_shifted(c: char) -> char {
    let shifted: HashMap<char, char> = [
        ('1', '!'), ('2', '@'), ('3', '#'), ('4', '$'), ('5', '%'), ('6', '^'), ('7', '&'), ('8', '*'), ('9', '('), ('0', ')'), ('=', '+'),
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

    // pub fn ind_of_pos(&self, pos: Vec2<usize>) -> usize {
    //     println!("ind of {:?}", pos);
    //     let line = pos.y();
    //     let local_line = line - self.start.y;
    //     let (_, ln_end) = self.lines[local_line.min(self.lines.len().max(1)-1)];
    //
    //     (pos.x()).min(ln_end) + local_line
    // }
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
pub struct Editor {
    cursors: Vec<Cursor>,
    chunks: Vec<Chunk>,
    chunk_info: Vec<ChunkInfo>,
    changes: HashMap<usize, (usize, usize, Change)>,
}

impl Editor {

    pub fn new<T: AsRef<str>>(str: T) -> Self {
        let mut editor = Editor {
            cursors: vec![],
            chunks: vec![],
            chunk_info: vec![],
            changes: Default::default(),
        };

        Editor::create_chunks_from(str, 0, &mut editor.chunks, &mut editor.chunk_info);

        editor
    }


    pub fn create_chunks_from<T: AsRef<str>>(str: T, insert_at: usize, chunks: &mut Vec<Chunk>, infos: &mut Vec<ChunkInfo>) {
        let str = str.as_ref();
        let mut index = 0;
        let mut pos = Vec2::new(0,0);
        let mut i = 0;
        while index < str.len() {
            let chunk_str = &str[index..(index +CHUNK_SIZE).min(str.len())];
            let chunk = Chunk::new(chunk_str.to_string());

            let chunk_info = ChunkInfo::new(&chunk, index, pos.clone());
            pos = chunk_info.end.clone();

            chunks.insert(i + insert_at, chunk);
            infos.insert(i + insert_at, chunk_info);

            index += CHUNK_SIZE;
            i += 1;
        }
    }

    pub fn add_cursor(&mut self, pos: impl Into<Vec2<usize>>) {
        self.cursors.push(Cursor::new(pos.into()));
    }

    /// Returns the global
    pub fn pos_index(&self, pos: impl Into<Vec2<usize>>) -> (usize, usize) {
        let pos = pos.into();

        let mut index = 0;
        let mut i = 0;
        'chunk: for ci in &self.chunk_info {
            println!("checking {:?} {:?}", pos, ci);

            let mut line_ind = ci.start.y;
            for (line_start, line_end) in &ci.lines {
                let column_min = if line_ind == ci.start.y {
                    ci.start.x
                } else {
                    0
                };
                let column_max = if line_ind == ci.end.y {
                    ci.end.x
                } else {
                    (line_end - line_start) // line width
                };
                println!("{} {} {} {:?} {} {}", line_ind, line_start, line_end, pos, column_min, column_max);
                if pos.y == line_ind && pos.x >= column_min && pos.x < column_max {
                    let mut line_index_offset = if line_ind == ci.start.y {
                        0
                    } else {
                        *line_start
                    };

                    index = line_index_offset + pos.x + line_ind;
                    println!("found chunk {} {:?}", index, self.chunks[index].str);
                    break 'chunk;
                }
                line_ind += 1;
            }
            i += 1;
        }

        (index, i)
    }

    pub fn add_change(&mut self, change: Change) {
        for c in &self.cursors {
            let (start_index, start_chunk) = self.pos_index(c.start_pos());
            let (mut end_index, end_chunk) = self.pos_index(c.end_pos());
            // end_index += 1;
            println!("checked {} {} {} {} {:?}", start_index, end_index, start_chunk, end_chunk, change);

            let mut max_reached = 0;
            for chunk_index in start_chunk..=end_chunk.min(self.chunks.len()-1) {
                let chunk_info = &self.chunk_info[chunk_index];

                let chunk_change = match &change {
                    Change::Delete => Change::Delete,
                    Change::Add(str) => {
                        let mut sub = String::new();
                        let mut i = 0;
                        for c in str.chars() {
                            // let max = (chunk_info.ind_end - chunk_info.ind_start) - (start_index.max(chunk_info.ind_start) - chunk_info.ind_start);
                            let max = (chunk_info.ind_end - chunk_info.ind_start) + (chunk_info.ind_start.max(start_index) - start_index.max(chunk_info.ind_start) - chunk_info.ind_start);
                            println!("c {} {} {} {}", chunk_info.ind_start.max(start_index) - start_index, max, (chunk_info.ind_end - chunk_info.ind_start), (start_index.max(chunk_info.ind_start) - chunk_info.ind_start));
                            if i >= chunk_info.ind_start.max(start_index) - start_index &&
                                (i < max || chunk_index == end_chunk.min(self.chunks.len()-1)) {
                                sub.push(c);
                                max_reached = i;
                            }
                            i += 1;
                        }
                        Change::Add(sub)
                    }
                };
                let start_index = start_index.max(chunk_info.ind_start);
                let end_index = end_index.min(chunk_info.ind_end);
                println!("chunk {:?} {:?} {:?} {:?} {:?}", chunk_change, chunk_info, start_index, end_index, self.chunks[chunk_index].str);

                self.changes.insert(start_index, (end_index - start_index, chunk_index, chunk_change));
            }
        }
    }

    fn insert_chunk(&mut self, chunk_index: usize, chunk: Chunk, info: ChunkInfo) {
        self.chunks.insert(chunk_index, chunk);
        self.chunk_info.insert(chunk_index, info);
    }

    fn create_chunk<T: AsRef<str>>(&self, chunk_index: usize, str: T) -> (Chunk, ChunkInfo) {
        let str = str.as_ref();
        let chunk = Chunk::new(str.to_string());

        let (text_index, pos) = if chunk_index == 0 {
            (0, Vec2::new(0, 0))
        } else {
            let ci = &self.chunk_info[chunk_index-1];
            (ci.ind_end, ci.end.clone())
        };
        let info = ChunkInfo::new(&chunk, text_index, pos);

        (chunk, info)
    }

    fn remove_chunk(&mut self, index: usize) -> (Chunk, ChunkInfo) {
        let c = self.chunks.remove(index);
        let i = self.chunk_info.remove(index);
        (c, i)
    }

    fn correct_chunk_size(&mut self, chunk: usize) -> bool {
        let chunk_len = self.chunks[chunk].str.len();
        println!("checking {:?}", self.chunks[chunk].str);
        if chunk_len > CHUNK_SIZE {

            let ((chunk_1, chunk_info_1),(chunk_2, chunk_info_2)) = {
                let (c1_str, c2_str) = self.chunks[chunk].str.split_at(chunk_len/2);
                (self.create_chunk(chunk, c1_str), self.create_chunk(chunk+1, c2_str))
            };

            // replace old chunk efficiently
            std::mem::replace(&mut self.chunks[chunk], chunk_1);
            std::mem::replace(&mut self.chunk_info[chunk], chunk_info_1);

            // insert split half
            self.insert_chunk(chunk + 1, chunk_2, chunk_info_2);

            // recursively ensure no chunk is above the max size
            self.correct_chunk_size(chunk + 1);
            self.correct_chunk_size(chunk);
            return true;
        }
        false
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

        for c in &changed_chunks {
            self.chunks[*c].update();
        }

        changed_chunks.sort();
        for i in changed_chunks.len()-1..=0 {
            let chunk = changed_chunks[i];
            if self.correct_chunk_size(chunk) {
                for j in i..changed_chunks.len() {
                    changed_chunks[j] += 1;
                }
            }
        }

        let (mut ind, mut pos) = (0, Vec2::new(0,0));
        for chunk in changed_chunks {
            let c = &mut self.chunks[chunk];
            let ci = &mut self.chunk_info[chunk];
            ci.update_info(c, ind, pos.clone());
            ind = ci.ind_end;
            pos = ci.end;
        }
    }
}

pub struct RenderChunk {
    chunk: usize,
    chunk_changed: bool,
    text: FontRenderData,
    c: Color,
}

impl RenderChunk {
    pub fn new(chunk: usize) -> Self {
        RenderChunk {
            chunk,
            chunk_changed: true,
            text: FontRenderData::default(),
            c: Color::from_hsv(thread_rng().random::<f32>(), 1.0, 1.0),
        }
    }
}

pub struct Textbox {
    editor: Editor,
    render_chunks: Vec<RenderChunk>,
}

impl Textbox {
    pub fn new(font: impl ToString, text: String) -> Self {
        let mut textbox = Textbox {
            editor: Editor::new(text),
            render_chunks: vec![],
        };
        for i in 0..textbox.editor.chunks.len() {
            textbox.render_chunks.push(RenderChunk::new(i));
        }
        textbox
    }
}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        if event.is_render(RenderPass::Main) {
            let fr = context().fonts().font("main").unwrap();

            let mut end_pos = Vec2::new(0.,0.);
            for r_chunk in &mut self.render_chunks {
                let e_chunk = &self.editor.chunks[r_chunk.chunk];
                let i_chunk = &self.editor.chunk_info[r_chunk.chunk];

                // TODO make it so that text can be rendered using just the VAO
                r_chunk.text = fr.draw_string_offset((8., e_chunk.str.clone(), r_chunk.c), (10., 10.), end_pos);

                // add y-position while setting the x to the x of the last
                end_pos.y += r_chunk.text.end_char_pos().y;
                end_pos.x = r_chunk.text.end_char_pos().x;
            }

            for cursor in &self.editor.cursors {
                let (index, chunk) = self.editor.pos_index(cursor.pos);
                let r_chunk = &self.render_chunks[chunk];
                let i_chunk = &self.editor.chunk_info[chunk];
                let chunk_index = index - i_chunk.ind_start;

                // let char_pos = r_chunk.text.char_positions()
            }
        }
        match event {
            _ => {},
            Event::Keyboard(key, action, mods) => {

            }
        }
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
    // let mut str = include_str!("../../test_2.js");
    let mut str = "THIS IS A NOT SO LONG STIRNG\nAND THIS is another line";;
    // let mut str = "12345678";
    // "abcdefgh678"
    // "abcdefgh"
    let st = Instant::now();
    let mut editor = Editor::new(str);
    let d = st.elapsed();
    // println!("editor {:?}", d);

    let add = "abcdefgh";
    // editor.add_cursor((0,0));
    editor.add_cursor((3,1));
    // editor.cursors[0].position((7,1), true);

    println!("{:?}", editor);
    editor.add_change(Change::Add(add.to_string()));
    // println!("{:?}", editor);
    let st = Instant::now();
    editor.apply_changes();
    let d = st.elapsed();
    println!("{:?}", editor);

    for c in editor.chunks {
        print!("{}", c.str);
        // break;
    }
    println!();
    println!("{:?} {}", d, str.len());
}