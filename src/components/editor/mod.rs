use std::collections::HashMap;
use std::iter::Iterator;
use std::mem;
use std::ops::Add;

use rand::Rng;

use crate::components::editor::chunk::{Chunk, ChunkInfo};
use crate::components::editor::cursor::Cursor;
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::render::color::ToColor;
use crate::components::spatial::vec2::Vec2;

pub mod textbox;
mod cursor;
mod chunk;

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
pub struct Editor {
    cursors: Vec<Cursor>,
    chunks: Vec<Chunk>,
    chunk_info: Vec<ChunkInfo>,
    changes: HashMap<usize, (usize, usize, Change)>,
}

impl Editor {

    pub fn new<T: AsRef<str>>(str: T) -> Self {
        println!("repl");
        let str = str.as_ref().replace("\r\n", "\n");
        println!("rest");
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
            }
            if c.select_pos.x > line_width_2 {
                if move_down {
                    c.select_pos.y += 1;
                    c.select_pos.x = 0;
                } else {
                    c.pos.x = line_width_2;
                }
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

    pub fn line(&self, line_index: usize) -> (usize, usize, usize, usize, usize) { // width, start, end, start_chunk, end_chunk
        // TODO fix issue where the line is not counted properly if there is a \r on a previous chunk with the \n on the next chunk
        let mut current_line_start = 0;
        let mut chunk_index = 0;
        let mut start_chunk = 0;

        for ci in &self.chunk_info {
            let mut i = 0;

            let mut current_line = ci.start.y;
            for line in &ci.lines {
                let (ind_start, ind_end, new_line) = *line;
                let w = ind_end - ind_start;

                if chunk_index == self.chunk_info.len()-1 {
                    return (ind_end - current_line_start, current_line_start, ind_end, start_chunk, chunk_index);
                }

                if new_line > 0 {
                    if current_line == line_index {
                        return ((ind_end - current_line_start).max(new_line) - new_line, current_line_start, ind_end, start_chunk, chunk_index);
                    }
                    current_line += 1;
                    current_line_start = ind_end;
                    start_chunk = chunk_index;
                }
                i += 1;
            }
            chunk_index += 1;
        }
        (0, 0, 0, 0, 0)
    }

    /// Returns the global
    pub fn pos_index(&self, pos: impl Into<Vec2<usize>>) -> (usize, usize) {
        let pos = pos.into();

        let mut char_index = 0;
        let mut i = 0;

        'chunk: for ci in &self.chunk_info {
            let mut line_ind = ci.start.y;
            // TODO a loop could probably be skipped by finding the specific line using the pos.y and ci.start.y?
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

                if pos.y == line_ind && (pos.x >= column_min && pos.x < column_max) || (pos.x == column_min && pos.x == column_max) {
                    char_index = *line_start + pos.x - column_min;
                    return (char_index, i);
                }
                line_ind += 1;
            }
            i += 1;
        }

        (char_index, i)
    }

    pub fn add_change(&mut self, change: Change) {
        for c in &self.cursors {
            let (start_index, start_chunk) = self.pos_index(c.start_pos());
            let (mut end_index, end_chunk) = self.pos_index(c.end_pos());

            let mut max_reached = 0;
            for chunk_index in start_chunk..=end_chunk.min(self.chunks.len()-1) {
                let chunk_info = &self.chunk_info[chunk_index];

                let chunk_change = match &change {
                    Change::Delete => Change::Delete,
                    Change::Add(str) => {
                        let mut sub = String::new();
                        let mut i = 0;
                        for c in str.chars() {
                            let min = if start_index > chunk_info.ind_start {
                                0
                            } else {
                                chunk_info.ind_start.max(start_index) - start_index
                            };
                            let max = min + (chunk_info.ind_end - start_index.max(chunk_info.ind_start));
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

                let start_index = start_index.max(chunk_info.ind_start);
                let end_index = end_index.min(chunk_info.ind_end);

                self.changes.insert(start_index, (end_index.max(start_index) - start_index, chunk_index, chunk_change));
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
        if chunk_len > CHUNK_SIZE {
            let mut tmp = mem::take(&mut self.chunks[chunk].str);
            let (c1_str, c2_str) = tmp.split_at(chunk_len/2);
            let (chunk_1, chunk_info_1) = self.create_chunk(chunk, c1_str);

            // replace old chunk efficiently
            std::mem::replace(&mut self.chunks[chunk], chunk_1);
            std::mem::replace(&mut self.chunk_info[chunk], chunk_info_1);

            let (chunk_2, chunk_info_2) = self.create_chunk(chunk+1, c2_str);
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
        if changed_chunks.len() == 0 {
            return;
        }

        for i in (0..changed_chunks.len()).rev() {
            let chunk = changed_chunks[i];
            if self.chunks[chunk].str.len() == 0 {
                continue;
            }

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

        for i in (0..changed_chunks.len()).rev() {
            let chunk = changed_chunks[i];
            if self.chunks[chunk].str.len() == 0 {
                self.chunks.remove(chunk);
            }
        }
    }
}

