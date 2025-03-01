use std::collections::HashMap;
use std::time::Instant;

use glfw::{Action, Key};
use rand::{Rng, thread_rng};

use crate::components::context::context;
use crate::components::editor::{Change, Cursor, Editor};
use crate::components::framework::animation::{AnimationRef, AnimationRegistry, Easing};
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::FontRenderData;
use crate::components::render::font::format::{Alignment, FormatItem, Text};
use crate::components::render::font::format::FormatItem::{AlignH, Size};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::text;

pub struct RenderChunk {
    chunk: usize,
    chunk_changed: bool,
    last_scroll: Vec2<f32>,
    text: FontRenderData,
    c: Color,
}

impl RenderChunk {
    pub fn new(chunk: usize) -> Self {
        RenderChunk {
            chunk,
            chunk_changed: true,
            last_scroll: Default::default(),
            text: FontRenderData::default(),
            c: Color::from_hsv(thread_rng().random::<f32>(), 0.6, 1.0),
        }
    }
}

pub struct Textbox {
    editor: Editor,
    render_chunks: Vec<RenderChunk>,
    changed: bool,

    anim_registry: AnimationRegistry,
    scroll: (AnimationRef, AnimationRef),
    target_scroll: Vec2<f32>,

    line_texts: HashMap<usize, Text>,
}

impl Textbox {
    pub fn new(font: impl ToString, text: String) -> Self {
        let mut animations = AnimationRegistry::new();
        let scroll = (animations.new_anim(), animations.new_anim());
        let mut textbox = Textbox {
            editor: Editor::new(text),
            render_chunks: vec![],
            changed: true,

            anim_registry: animations,
            scroll,
            target_scroll: Default::default(),
            line_texts: Default::default(),
        };
        for i in 0..textbox.editor.chunks.len() {
            textbox.render_chunks.push(RenderChunk::new(i));
        }
        textbox.editor.cursors.push(Cursor::new(Vec2::new(0,0)));

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
        let fr = context().fonts().font("main").unwrap();
        let size = 12.;
        let fr_height = fr.get_sized_height(size);

        let scroll = Vec2::new(self.scroll.0.borrow().value(), self.scroll.1.borrow().value());
        let mut offset = Vec2::new(40., 10.);
        if event.is_render(RenderPass::Main) {

            let st = Instant::now();
            for i in self.render_chunks.len()..self.editor.chunks.len() {
                self.render_chunks.push(RenderChunk::new(i));
            }

            while self.render_chunks.len() > self.editor.chunks.len() {
                self.render_chunks.pop();
            }

            let (mut start_line, mut end_line) = (usize::MAX,0);
            let mut end_pos = Vec2::new(0.,0.);
            for r_chunk in &mut self.render_chunks {
                let e_chunk = &self.editor.chunks[r_chunk.chunk];
                let i_chunk = &self.editor.chunk_info[r_chunk.chunk];

                if r_chunk.text.char_positions().len() > 0 {
                    let first_char = r_chunk.text.char_positions()[0];
                    if first_char[1] + first_char[3] - r_chunk.last_scroll.y() + scroll.y() > context().window().height as f32 {
                        // println!("break at {:?} {:?}", e_chunk, i_chunk);
                        break;
                    }

                    let last_char = r_chunk.text.char_positions().last().unwrap();
                    if last_char[1] + last_char[3] - r_chunk.last_scroll.y() + scroll.y() < offset.y {
                        // println!("continue at {:?} {:?}", e_chunk, i_chunk);
                        end_pos.y += r_chunk.text.end_char_pos().y;
                        end_pos.x = r_chunk.text.end_char_pos().x;
                        continue;
                    }
                }

                if i_chunk.start.y < start_line {
                    start_line = i_chunk.start.y;
                }

                if i_chunk.end.y > end_line {
                    end_line = i_chunk.end.y;
                }

                // TODO make it so that text can be rendered using just the VAO
                r_chunk.text = fr.draw_string_offset((size, &e_chunk.str, r_chunk.c), offset + scroll, end_pos);
                r_chunk.last_scroll = scroll;
                r_chunk.text.bounds().debug_draw(r_chunk.c * 0x20ffffff.to_color());

                // add y-position while setting the x to the x of the last
                end_pos.y += r_chunk.text.end_char_pos().y;
                end_pos.x = r_chunk.text.end_char_pos().x;

                if end_pos.y() + scroll.y() > context().window().height as f32 {
                    break;
                }
            }

            for cursor in &self.editor.cursors {
                let (index, chunk) = self.editor.pos_index(cursor.pos);
                if chunk == self.editor.chunks.len() {
                    continue;
                }

                let r_chunk = &self.render_chunks[chunk];
                let i_chunk = &self.editor.chunk_info[chunk];
                let chunk_index = index - i_chunk.ind_start;

                if r_chunk.text.char_positions().len() == 0 {
                    continue;
                }
                let ind = chunk_index.max(1)-1;
                let mut char_pos = r_chunk.text.char_positions()[ind];
                if cursor.pos.x == 0 {
                    char_pos[0] = offset.x;
                    char_pos[2] = 0.;
                }

                let cursor_width = 1.;
                let mut cursor_draw = Vec4::xywh(char_pos[0]+char_pos[2] - cursor_width/2., cursor.pos.y as f32 * fr_height, cursor_width, fr_height);
                cursor_draw.set_y(cursor_draw.y() + offset.y);
                cursor_draw.offset(scroll);
                context().renderer().draw_rect(cursor_draw, 0xffffffff);
            }

            offset.offset((-10., 0.));

            for line in start_line..end_line {
                if !self.line_texts.contains_key(&line) {
                    let text = text!(
                        AlignH(Alignment::Right),
                        Size(size),
                        FormatItem::Color(0xff909090.to_color()),
                        format!("{}", line),
                    );
                    self.line_texts.insert(line, text);
                }
                let a= self.line_texts.get(&line).unwrap();
                fr.draw_string(a.clone(), offset + (0, line as f32 * fr_height) + (0,scroll.y()));
            }
            println!("render: {:?}", st.elapsed());
        }
        let tmp_changed = self.changed;
        self.changed = false;
        let st = Instant::now();
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
                            Key::End => {
                                for c in &mut self.editor.cursors {
                                    c.position(Vec2::new(usize::MAX, c.pos.y), false);
                                }
                                self.correct_cursor(false);
                            }
                            Key::Home => {
                                for c in &mut self.editor.cursors {
                                    c.position(Vec2::new(0, c.pos.y), false);
                                }
                                self.correct_cursor(false);
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
            }
            Event::PreRender => {
                let (scroll_speed, easing) = (2., Easing::Sin);
                let scroll_mult = 40.;

                self.scroll.0.borrow_mut().animate_to(self.target_scroll.x * scroll_mult, scroll_speed, easing);
                self.scroll.1.borrow_mut().animate_to(self.target_scroll.y * scroll_mult, scroll_speed, easing);
            }
            Event::PostRender => {
                if self.changed {
                    self.changed = false;
                }
            }
            Event::Scroll(x, y) => {
                self.target_scroll += (*x, *y);
            }
            Event::MouseClick(button, action) => {
                if action == &Action::Release {
                    return false;
                }

                let mouse_pos = context().window().mouse().pos();

                let mouse_line = ((mouse_pos.y() - scroll.y() + offset.y()) / fr_height) as usize - 1;

                let (.., start_chunk, end_chunk) = self.editor.line(mouse_line);

                let mut closest = (f32::MAX, 0);
                if mouse_pos.x - offset.x() > 0. {
                    for i in start_chunk..=end_chunk {
                        let r_chunk = &self.render_chunks[i];
                        let e_chunk = &self.editor.chunks[i];
                        let i_chunk = &self.editor.chunk_info[i];

                        let mut line_index = i_chunk.start.y;
                        for ln in &i_chunk.lines {
                            let (start, end, new_line) = *ln;

                            if line_index == mouse_line {
                                for i in start..end {
                                    let char_index = i - i_chunk.ind_start;
                                    let char = &r_chunk.text.char_positions()[char_index];
                                    let char_x_pos = char[0] + char[2] / 2.;

                                    let dist = (mouse_pos.x() - char_x_pos).abs();
                                    if dist < closest.0 {
                                        let column_min = if line_index == i_chunk.start.y {
                                            i_chunk.start.x
                                        } else {
                                            0
                                        };
                                        let column = i - start + column_min + 1;
                                        closest = (dist, column);
                                    }
                                }
                            }
                            if new_line > 0 {
                                line_index += 1;
                            }
                        }
                    }
                }

                // if closest.0 != f32::MAX {
                self.editor.cursors[0].position(Vec2::new(closest.1, mouse_line), false);
                self.correct_cursor(false);
                // }
            }
            _ => {},
        }
        let d = st.elapsed();
        if self.changed {
            println!("{:?}", d);
        }
        self.changed = tmp_changed || self.changed;
        false
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        self.changed
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        Some(&mut self.anim_registry)
    }
}