use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::mem;
use std::ops::Add;
use std::time::Instant;

use glfw::{Action, Key, Modifiers, MouseButton};
use rand::{Rng, thread_rng};

use crate::components::context::context;
use crate::components::framework::animation::{AnimationRef, AnimationRegistry, AnimationType};
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::state::State;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::FontRenderData;
use crate::components::render::font::format::{Alignment, FormatItem, Text};
use crate::components::render::font::format::FormatItem::{AlignH, Size};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::gl_binds::gl11::Translatef;
use crate::text;

const CHUNK_SIZE: usize = 128; // 1024*2
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

    lines: Vec<(usize, usize, usize)>,
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

        self.lines.clear();

        let mut line_start_ind = start_index;
        let mut i = start_index;
        let mut prev_char = '_';
        for c in chunk.str.chars() {
            self.end.x += 1;
            i += 1;
            if c == '\n' {
                let w = if prev_char == '\r' {
                    2
                } else {
                    1
                };
                self.lines.push((line_start_ind, i, w));
                line_start_ind = i;
                self.end.y += 1;
                self.end.x = 0;
            }
            prev_char = c;
        }
        self.lines.push((line_start_ind, i, 0));
        self.lines.shrink_to_fit();
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
        let str = str.as_ref().replace("\r\n", "\n");
        let mut editor = Editor {
            cursors: vec![],
            chunks: vec![],
            chunk_info: vec![],
            changes: Default::default(),
        };

        Editor::create_chunks_from(str, 0, &mut editor.chunks, &mut editor.chunk_info);

        editor
    }

    pub fn correct_cursors(&mut self, move_down: bool) {
        let mut cursor_line_widths = vec![];
        let mut i = 0;
        for c in &self.cursors {
            let (w1, w2) = if c.pos.y == c.select_pos.y {
                let w = self.line(c.pos.y).0;
                (w, w)
            } else {
                (self.line(c.pos.y).0, self.line(c.select_pos.y).0)
            };
            println!("cursor line widht: {} {:?} {:?}", w1, w2, c.pos);
            cursor_line_widths.push((i, w1, w2));
            i += 1;
        }

        for (cursor_index, line_width_1, line_width_2) in cursor_line_widths {
            let c = &mut self.cursors[cursor_index];
            if c.pos.x > line_width_1 {
                if move_down {
                    c.pos.y += 1;
                    c.pos.x = 0;
                } else {
                    c.pos.x = line_width_1;
                }
                println!("correct pos to {:?}", c.pos);
            }
            if c.select_pos.x > line_width_2 {
                if move_down {
                    c.select_pos.y += 1;
                    c.select_pos.x = 0;
                } else {
                    c.pos.x = line_width_2;
                }
                println!("correct select_pos to {:?}", c.select_pos);
            }
        }
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

    pub fn line(&self, line_index: usize) -> (usize, usize, usize) { // width, start, end
        // TODO fix issue where the line is not counted properly if there is a \r on a previous chunk with the \n on the next chunk
        let mut current_line_start = 0;
        let mut chunk_index = 0;
        println!("lw at {}", line_index);
        for ci in &self.chunk_info {
            let mut i = 0;
            println!("{:?} {:?}", ci, self.chunks[chunk_index].str);
            let mut current_line = ci.start.y;
            for line in &ci.lines {
                let (ind_start, ind_end, new_line) = *line;
                let w = ind_end - ind_start;

                println!("{:?} {:?} {:?} {:?} {:?} {:?}", ind_start, ind_end, w, current_line_start, current_line, &self.chunks[chunk_index].str[ind_start-ci.ind_start..ind_end-ci.ind_start]);
                //
                if chunk_index == self.chunk_info.len()-1 {
                    return (ind_end - current_line_start, current_line_start, ind_end);
                }
                if new_line > 0 {
                    if current_line == line_index {
                        return ((ind_end - current_line_start).max(new_line) - new_line, current_line_start, ind_end);
                    }
                    current_line += 1;
                    current_line_start = ind_end;
                }
                i += 1;
            }
            chunk_index += 1;
        }
        (0, 0, 0)
    }

    /// Returns the global
    pub fn pos_index(&self, pos: impl Into<Vec2<usize>>) -> (usize, usize) {
        let pos = pos.into();

        let mut char_index = 0;
        let mut i = 0;
        // println!("check_pos {:?}", pos);
        'chunk: for ci in &self.chunk_info {
            let mut line_ind = ci.start.y;
            // TODO a loop could probably be skipped by finding the specific line using the pos.y and ci.start.y?
            println!("{:?}", ci);
            for (line_start, line_end, new_line) in &ci.lines {
                let column_min = if line_ind == ci.start.y {
                    ci.start.x
                } else {
                    0
                };
                let column_max = if line_ind == ci.end.y {
                    ci.end.x
                } else {
                    (line_end - line_start) + column_min // line width
                };
                println!("ln {} {} {} {} {} {:?} {:?} {:?}", line_ind, column_min, column_max, line_start, line_end, new_line, pos, self.chunks[i].str);
                if pos.y == line_ind && (pos.x >= column_min && pos.x < column_max) || (pos.x == column_min && pos.x == column_max) {

                    // char_index = line_index_offset + pos.x + line_ind;
                    char_index = *line_start + (pos.x - column_min);
                    println!("found chunk: char_index {} {:?} i {} pos.x {} ci.ind_start {} line_start {} line_end {} line_ind {}", char_index, self.chunks[i].str, i, pos.x, ci.ind_start, line_start, line_end, line_ind);
                    // break 'chunk;
                    return (char_index, i);
                }
                line_ind += 1;
            }
            i += 1;
        }

        println!("no chunk found {:?}", pos);
        (char_index, i)
    }

    pub fn add_change(&mut self, change: Change) {
        for c in &self.cursors {
            let (start_index, start_chunk) = self.pos_index(c.start_pos());
            let (mut end_index, end_chunk) = self.pos_index(c.end_pos());
            // end_index += 1;
            // println!("checked {} {} {} {} {:?}", start_index, end_index, start_chunk, end_chunk, change);

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
                            let chunk_size = chunk_info.ind_end - chunk_info.ind_start;
                            // let max = chunk_info
                            let min = if start_index > chunk_info.ind_start {
                                0
                            } else {
                                chunk_info.ind_start.max(start_index) - start_index
                            };
                            let max = min + (chunk_info.ind_end - start_index.max(chunk_info.ind_start));
                            // println!("c {} {} {} {} {}", min, max, chunk_info.ind_start, chunk_info.ind_end, start_index);
                            if i >= min &&
                                (i < max || chunk_index == end_chunk.min(self.chunks.len()-1)) {
                                sub.push(c);
                                max_reached = i;
                            }
                            i += 1;
                        }
                        Change::Add(sub)
                    }
                };
                // println!("chunk {:?} {:?} {:?} {:?} {:?}", chunk_change, chunk_info, start_index, end_index, self.chunks[chunk_index].str);
                let start_index = start_index.max(chunk_info.ind_start);
                let end_index = end_index.min(chunk_info.ind_end);

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
        // println!("checking {:?}", self.chunks[chunk].str);
        if chunk_len > CHUNK_SIZE {
            let mut tmp = mem::take(&mut self.chunks[chunk].str);
            let (c1_str, c2_str) = tmp.split_at(chunk_len/2);
            let (chunk_1, chunk_info_1) = self.create_chunk(chunk, c1_str);
            // println!("chunk_1 {:?} {:?}", chunk_1, chunk_info_1);

            // replace old chunk efficiently
            std::mem::replace(&mut self.chunks[chunk], chunk_1);
            std::mem::replace(&mut self.chunk_info[chunk], chunk_info_1);

            let (chunk_2, chunk_info_2) = self.create_chunk(chunk+1, c2_str);
            // println!("chunk_2 {:?} {:?}", chunk_2, chunk_info_2);
            // insert split half
            self.insert_chunk(chunk + 1, chunk_2, chunk_info_2);

            // recursively ensure no chunk is above the max size
            self.correct_chunk_size(chunk + 1);
            self.correct_chunk_size(chunk);

            // println!("corrected {:?} {:?} | {:?} {:?}", self.chunks[chunk], self.chunk_info[chunk], self.chunks[chunk+1], self.chunk_info[chunk+1]);
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
        // println!("CHANGED {:?}", changed_chunks);
        if changed_chunks.len() == 0 {
            return;
        }

        for i in (0..changed_chunks.len()).rev() {
            let chunk = changed_chunks[i];
            if self.correct_chunk_size(chunk) {
                for j in i..changed_chunks.len() {
                    changed_chunks[j] += 1;
                }
            }
        }

        let (mut ind, mut pos) = if changed_chunks[0] == 0 {
            (0,Vec2::new(0,0))
        } else {
            let ci = &self.chunk_info[(changed_chunks[0].max(1)-1)];
            (ci.ind_end, ci.end.clone())
        };
        for chunk in changed_chunks[0]..self.chunks.len() {
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
            c: Color::from_hsv(thread_rng().random::<f32>(), 0.6, 1.0),
        }
    }
}

pub struct Textbox {
    editor: Editor,
    render_chunks: Vec<RenderChunk>,
    changed: bool,
}

impl Textbox {
    pub fn new(font: impl ToString, text: String) -> Self {
        let mut textbox = Textbox {
            editor: Editor::new(text),
            render_chunks: vec![],
            changed: true,
        };
        for i in 0..textbox.editor.chunks.len() {
            textbox.render_chunks.push(RenderChunk::new(i));
        }
        textbox.editor.cursors.push(Cursor::new(Vec2::new(0,0)));

        // for i in 0..textbox.editor.chunks.len() {
        //     println!("{:?} {:?}", textbox.editor.chunks[i].str, textbox.editor.chunk_info[i]);
        // }

        textbox
    }

    fn move_left(&mut self) {
        for i in 0..self.editor.cursors.len() {
            if self.editor.cursors[i].pos.x == 0 {
                let mut pos = self.editor.cursors[i].pos.clone();

                pos.y -= 1;
                pos.x = self.editor.line(pos.y).0;

                self.editor.cursors[i].position(pos, false);
            } else {
                self.editor.cursors[i].left(false)
            }

        }
        self.correct_cursor(true);
    }

    fn apply_changes(&mut self) {
        self.changed = true;
        self.editor.apply_changes();
    }

    fn correct_cursor(&mut self, move_down: bool) {
        self.changed = true;
        self.editor.correct_cursors(move_down);
    }
}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        if event.is_render(RenderPass::Main) {
            let fr = context().fonts().font("main").unwrap();
            let size = 12.;
            let fr_height = fr.get_sized_height(size);
            let mut offset = Vec2::new(40., 10.);

            // println!("{} {}", self.render_chunks.len(), self.editor.chunks.len());
            for i in self.render_chunks.len()..self.editor.chunks.len() {
                self.render_chunks.push(RenderChunk::new(i));
            }

            let (mut start_line, mut end_line) = (usize::MAX,0);
            let mut end_pos = Vec2::new(0.,0.);
            for r_chunk in &mut self.render_chunks {
                let e_chunk = &self.editor.chunks[r_chunk.chunk];
                let i_chunk = &self.editor.chunk_info[r_chunk.chunk];

                if i_chunk.start.y < start_line {
                    start_line = i_chunk.start.y;
                }

                if i_chunk.end.y > end_line {
                    end_line = i_chunk.end.y;
                }

                // println!("{} {:?}", r_chunk.chunk, e_chunk.str);

                // TODO make it so that text can be rendered using just the VAO
                r_chunk.text = fr.draw_string_offset((size, &e_chunk.str, r_chunk.c), offset, end_pos);
                // r_chunk.text.bounds().debug_draw(r_chunk.c * 0x20ffffff.to_color());

                // add y-position while setting the x to the x of the last
                end_pos.y += r_chunk.text.end_char_pos().y;
                end_pos.x = r_chunk.text.end_char_pos().x;
            }

            // println!("first {:?} {:?}", self.editor.chunk_info[0], self.editor.chunks[0].str);
            for cursor in &self.editor.cursors {
                let (index, chunk) = self.editor.pos_index(cursor.pos);
                // print!("index {} chunk {} pos {:?}", index, chunk, cursor.pos,);
                if chunk == self.editor.chunks.len() {
                    // println!(" !! end chunk");
                    continue;
                }

                let r_chunk = &self.render_chunks[chunk];
                let i_chunk = &self.editor.chunk_info[chunk];
                let chunk_index = index - i_chunk.ind_start;

                if r_chunk.text.char_positions().len() == 0 {
                    // println!(" !! no char pos");
                    continue;
                }
                let ind = chunk_index.max(1)-1;
                let mut char_pos = r_chunk.text.char_positions()[ind];
                if cursor.pos.x == 0 {
                    char_pos[0] = offset.x;
                    char_pos[2] = 0.;
                }
                // println!();
                // println!(" chunk_index {} {:?} {:?}",  chunk_index, &(&self.editor.chunks[chunk].str)[ind..ind+1], char_pos);

                let mut cursor_draw = Vec4::xywh(char_pos[0]+char_pos[2], cursor.pos.y as f32 * fr_height, 1., fr_height);
                cursor_draw.set_y(cursor_draw.y() + offset.y);
                context().renderer().draw_rect(cursor_draw, 0xffffffff);
            }

            offset.offset((-10., 0.));

            for line in start_line..end_line {
                let text = text!(
                    AlignH(Alignment::Right),
                    Size(size),
                    FormatItem::Color(0xff909090.to_color()),
                    format!("{}", line),
                );
                fr.draw_string(text, offset + (0, line as f32 * fr_height));
            }
        }
        match event {
            Event::Keyboard(key, action, mods) => {
                if action == &Action::Release {
                    return false;
                }

                match key.get_name() {
                    None =>
                        match key {
                            Key::Up => {
                                for c in &mut self.editor.cursors {
                                    c.up(false)
                                }
                                self.correct_cursor(false);
                            }
                            Key::Down => {
                                for c in &mut self.editor.cursors {
                                    c.down(false)
                                }
                                self.correct_cursor(false);
                            }
                            Key::Right => {
                                for c in &mut self.editor.cursors {
                                    c.right(false)
                                }
                                self.correct_cursor(true);
                            }
                            Key::Left => {
                                self.move_left();
                            }

                            Key::Space => {
                                self.editor.add_change(Change::Add(" ".to_string()));
                                self.apply_changes();
                                for c in &mut self.editor.cursors {
                                    c.right(false)
                                }
                                self.correct_cursor(true);
                            }
                            Key::Enter => {
                                self.editor.add_change(Change::Add("\n".to_string()));
                                self.apply_changes();

                                for c in &mut self.editor.cursors {
                                    c.right(false)
                                }
                                self.correct_cursor(true);
                            }
                            Key::Backspace => {
                                self.move_left();

                                self.editor.add_change(Change::Delete);
                                self.apply_changes();
                            }
                            Key::Delete => {
                                self.editor.add_change(Change::Delete);
                                self.apply_changes();
                            }
                            _ => {}
                        },
                    Some(pressed) => {
                        let st = Instant::now();
                        self.editor.add_change(Change::Add(pressed));
                        self.apply_changes();
                        let d = st.elapsed();
                        for c in &mut self.editor.cursors {
                            c.right(false);
                        }
                        self.correct_cursor(true);
                        println!("{:?}", d);
                    }
                }
                // println!("{:?}", self.editor.cursors[0].pos);
            }
            Event::PostRender => {
                if self.changed {
                    self.changed = false;
                }
            }
            _ => {},
        }
        false
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        self.changed
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        None
    }
}

// #[test]
pub fn editor() {
    let mut str = include_str!("../../test.js").to_string();
    let (str, _) = str.split_at(400);
    // let mut str = "THIS IS A NOT SO LONG STIRNG\nAND THIS is another line";;

    // let mut str = "12345678";
    // "abcdefgh678"
    // "abcdefgh"
    let st = Instant::now();
    let mut editor = Editor::new(&str);
    let d = st.elapsed();
    // println!("editor {:?}", d);
    //
    // let w=  editor.line_width(1);
    // let st = &str[0..w];
    // println!("{}", w);
    for i in 0..20 {
        let (w, s, e) = editor.line(i);
        let st = &str[s..e];
        println!("{} {:?}", w, st);
    }
    // let add = "abcdefgh";
    // // editor.add_cursor((0,0));
    // editor.add_cursor((0,1));
    // editor.cursors[0].position((4,1), true);
    //
    // println!("{:?}", editor);
    // editor.add_change(Change::Add(add.to_string()));
    //
    // let st = Instant::now();
    // editor.apply_changes();
    // let d = st.elapsed();
    // // editor.cursors[0].position((8,1), true);
    // // editor.add_change(Change::Add("AND ".to_string()));
    // //
    // // editor.apply_changes();
    //
    // // println!("{:?}", editor);
    //
    // // for i in 0..editor.chunks.len() {
    // //     println!("{:?} {:?}", editor.chunk_info[i], editor.chunks[i].str )
    // // }
    //
    // for c in editor.chunks {
    //     print!("{}", c.str);
    //     // break;
    // }
    // println!();
    // println!("{:?} {}", d, str.len());
}