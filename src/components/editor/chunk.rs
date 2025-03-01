use std::collections::HashMap;

use crate::components::editor::Change;
use crate::components::spatial::vec2::Vec2;

#[derive(Debug)]
pub struct ChunkInfo {
    pub(super) ind_start: usize,
    pub(super) ind_end: usize,

    pub(super) start: Vec2<usize>,
    pub(super) end: Vec2<usize>,

    pub(super) lines: Vec<(usize, usize, usize)>,
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
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub(super) str: String,
    pub(super) updated: String,
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