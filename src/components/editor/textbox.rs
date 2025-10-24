use std::collections::HashMap;
use std::mem;
use std::time::{Duration, Instant};

use glfw::{Action, Key, Modifiers, MouseButton};
use rand::{Rng, thread_rng};

use crate::components::context::context;
use crate::components::editor::{Change, Cursor, Editor, get_shifted};
use crate::components::framework::animation::{AnimationRef, AnimationRegistry, Easing};
use crate::components::framework::element::ui_traits::UIHandler;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::render::color::{solid, Color, ToColor};
use crate::components::render::font::FontRenderData;
use crate::components::render::font::format::{Alignment, FormatItem, Text};
use crate::components::render::font::format::FormatItem::{AlignH, Size};
use crate::components::render::renderer::Renderable;
use crate::components::render::renderer::shapes::Rect;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::gl_binds::gl11::{Enable, Finish};
use crate::gl_binds::gl30::FRAMEBUFFER_SRGB;
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
    offset: Vec2<f32>,

    anim_registry: AnimationRegistry,
    scroll: (AnimationRef, AnimationRef),
    target_scroll: Vec2<f32>,

    text_size: f32,

    line_texts: HashMap<usize, Text>,
    debug: bool,

    cursor_rects: Vec<Rect>,
}

impl Textbox {
    pub fn new(font: impl ToString, text: &String) -> Self {
        let mut animations = AnimationRegistry::new();
        let scroll = (animations.new_anim(), animations.new_anim());
        let st = Instant::now();
        let ed = Editor::new(1024 / 1, text);
        let d = st.elapsed();
        println!("created editor in {:?}", d);
        let mut textbox = Textbox {
            editor: ed,
            render_chunks: vec![],
            changed: true,
            text_size: 16.0,

            offset: Default::default(),
            anim_registry: animations,
            scroll,
            target_scroll: Default::default(),
            line_texts: Default::default(),

            debug: false,
            cursor_rects: Vec::new(),
        };
        for i in 0..textbox.editor.chunks.len() {
            textbox.render_chunks.push(RenderChunk::new(i));
        }
        textbox.editor.cursors.push(Cursor::new(Vec2::new(0,0)));

        textbox
    }

    fn move_left(&mut self, check_expanded: bool, expand: bool) {
        for i in 0..self.editor.cursors.len() {
            if check_expanded && self.editor.cursors[i].is_expanded() {
                continue;
            }
            if self.editor.cursors[i].pos.x == 0 {
                let mut pos = self.editor.cursors[i].pos.clone();

                pos.y -= 1;
                pos.x = self.editor.line(pos.y).0;

                self.editor.cursors[i].position(pos, expand);
            } else {
                self.editor.cursors[i].left(expand)
            }

        }
        self.correct_cursor(true);
    }

    fn apply_changes(&mut self) {
        self.changed = true;
        let st = Instant::now();
        for c in self.editor.apply_changes() {
            self.render_chunks[c].chunk_changed = true;
        }
        let d = st.elapsed();
        println!("edited in {:?}", d);
    }

    fn correct_cursor(&mut self, move_down: bool) {
        self.changed = true;
        self.editor.correct_cursors(move_down);
    }

    unsafe fn move_cursors_right(&mut self) {
        for c in &mut self.editor.cursors {
            c.right(context().keyboard().shift());
        }
        self.correct_cursor(true);
    }

    fn offset(&self) -> Vec2<f32> {
        Vec2::new(40., 10.)
    }

    unsafe fn screen_to_text_pos(&self, screen_pos: &Vec2<f32>) -> Vec2<usize> {
        let fr = context().fonts().font("main").unwrap();
        let fr_height = fr.get_sized_height(self.text_size);
        let mut offset = self.offset();

        if self.editor.chunks.len() == 0 {
            return Vec2::new(0, 0);
        }

        let screen_line = ((screen_pos.y() - self.scroll.1.borrow().value() + offset.y()) / fr_height) as usize - 1;;
        let (.., start_chunk, end_chunk) = self.editor.line(screen_line);
        let mut closest = (f32::MAX, 0);
        for i in start_chunk..=end_chunk {
            let r_chunk = &self.render_chunks[i];
            let e_chunk = &self.editor.chunks[i];
            let i_chunk = &self.editor.chunk_info[i];

            let mut line_index = i_chunk.start.y;
            for ln in &i_chunk.lines {
                let (mut start, mut end, new_line) = *ln;
                start += i_chunk.ind_offset;
                end += i_chunk.ind_offset;

                if line_index == screen_line {
                    for i in start..end {
                        let char_index = i - i_chunk.ind_start;
                        let char = &r_chunk.text.char_positions()[char_index];
                        let char_x_pos = char[0];

                        let dist = (screen_pos.x() - char_x_pos).abs();
                        if dist < closest.0 {
                            let column_min = if line_index == i_chunk.start.y {
                                i_chunk.start.x
                            } else {
                                0
                            };
                            let column = i - start + column_min;
                            closest = (dist, column);
                        }
                    }
                }
                if new_line > 0 {
                    line_index += 1;
                }
            }
        }

        Vec2::new(closest.1, screen_line)
    }

    unsafe fn cursor_to_mouse(&mut self, expand: bool) {

        let mouse_pos = context().window().mouse().pos();

        let pos = if mouse_pos.x - self.offset.x() > 0. {
            self.screen_to_text_pos(&mouse_pos)
        } else {
            Vec2::new(0,0)
        };

        self.editor.cursors[0].position(pos, expand);
        self.correct_cursor(false);
    }

    unsafe fn cursor_bounds(&self, pos: &Vec2<usize>, scroll: &Vec2<f32>, fr_height: f32) -> (Vec4, usize, usize, bool){
        let (mut end_index, end_chunk) = self.editor.pos_index(*pos);
        if end_chunk == self.editor.chunks.len() {
            return (Vec4::xywh(0,0,0,0), 0, 0, false);
        }

        let r_chunk = &self.render_chunks[end_chunk];
        let i_chunk = &self.editor.chunk_info[end_chunk];
        let local_index = end_index - i_chunk.ind_start;

        let ind = local_index;
        let mut char_pos = if pos.x == 0 {
            [self.offset.x, 0., 0., 0.]
        } else {
            if ind >= r_chunk.text.char_positions().len() {
                (0, 0, false);
            }
            r_chunk.text.char_positions()[ind]
        };


        let cursor_width = 1.;
        let mut cursor_draw = Vec4::xywh(char_pos[0] - cursor_width/2. + 1., pos.y as f32 * fr_height, cursor_width, fr_height);
        cursor_draw.set_y(cursor_draw.y() + self.offset.y - 2.);
        cursor_draw.offset(*scroll);
        // context().renderer().draw_rect(cursor_draw, 0xffffffff);
        // self.cursor_rect.set_bounds(&cursor_draw);

        (cursor_draw, end_index, end_chunk, true)
    }
}

impl UIHandler for Textbox {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        let fr = context().fonts().font("main").unwrap();
        let fr_height = fr.get_sized_height(self.text_size);
        let shift_pressed = context().keyboard().shift();

        let scroll = Vec2::new(self.scroll.0.borrow().value(), self.scroll.1.borrow().value());
        let mut offset = self.offset();
        self.offset = offset.clone();
        match event {
            Event::PreRender => {
                let mut cursors = mem::take(&mut self.cursor_rects);

                if cursors.len() != self.editor.cursors.len() {
                    for i in 0..self.editor.cursors.len() {
                        cursors.push(Rect::new(Vec4::xywh(0,0,0,0), solid(0xff909090)));
                    }
                }

                let mut i = 0;
                for cursor in &self.editor.cursors {
                    let (bounds, ..) = self.cursor_bounds(&cursor.pos, &scroll, fr_height);
                    cursors.get_mut(i).unwrap().set_bounds(&bounds);
                    i += 1;
                }

                mem::swap(&mut cursors, &mut self.cursor_rects);
            },
            _ => {}
        }
        if event.is_render(RenderPass::Main) {

            // Finish();
            // let st = Instant::now();
            for i in self.render_chunks.len()..self.editor.chunks.len() {
                self.render_chunks.push(RenderChunk::new(i));
            }

            while self.render_chunks.len() > self.editor.chunks.len() {
                self.render_chunks.pop();
            }


            // let mut t_render = Duration::from_micros(0);
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
                let color = if self.debug {
                    r_chunk.c
                    // Color::from_hsv(thread_rng().random::<f32>(), 0.6, 1.0)
                } else {
                    0xffbbbbbb.to_color()
                };
                // if r_chunk.chunk_changed {
                //     Finish();
                //     let st = Instant::now();
                    r_chunk.text = fr.draw_string_offset((self.text_size, &e_chunk.str, color), offset + scroll, end_pos);
                    // Finish();
                    // t_render += Instant::now() - st;
                    r_chunk.chunk_changed = false;
                // }
                r_chunk.last_scroll = scroll;
                if self.debug {
                    r_chunk.text.bounds().debug_draw(r_chunk.c * 0x20ffffff.to_color());
                }

                // add y-position while setting the x to the x of the last
                end_pos.y += r_chunk.text.end_char_pos().y;
                end_pos.x = r_chunk.text.end_char_pos().x;

                if end_pos.y + scroll.y > context().window().height as f32 + 100. {
                    break;
                }
            }

            let mut i = 0;
            for cursor in &self.editor.cursors {
                let (_, mut start_index, mut start_chunk, drawn) = self.cursor_bounds(&cursor.pos, &scroll, fr_height);
                // if !drawn {
                //     continue;
                // }
                // self.cursor_rect.render();
                self.cursor_rects.get(i).unwrap().render();

//                self.draw_cursor_pos(&cursor.select_pos, &scroll, fr_height)
                let (mut end_index, mut end_chunk) = self.editor.pos_index(cursor.select_pos);
                // if !drawn {
                //     continue;
                // }

                if end_index < start_index {
                    std::mem::swap(&mut end_index, &mut start_index);
                    std::mem::swap(&mut end_chunk, &mut start_chunk);
                }

                let mut start_pos = {
                    let i_chunk = &self.editor.chunk_info[start_chunk];
                    let index = start_index - i_chunk.ind_start;
                    let char_pos = self.render_chunks[start_chunk].text.char_positions();
                    char_pos[index.min(char_pos.len() - 1)][0]
                };
                let mut end_pos = self.render_chunks[start_chunk].text.char_positions()[0][0];
                let mut line = self.editor.chunk_info[start_chunk].start.y;
                for c in start_chunk..=end_chunk {
                    let i_chunk = &self.editor.chunk_info[c];
                    let r_chunk = &self.render_chunks[c];
                    // for l in &i_chunk.lines {
                    for i in 0..i_chunk.lines.len() {
                        let (start, mut end, new_line) = i_chunk.line_with_offset(i);
                        end = end - new_line;
                        let (local_start, local_end) = (start - i_chunk.ind_start, end - i_chunk.ind_start);

                        let mut index = end.min(end_index).max(start) - i_chunk.ind_start;
                        if index >= r_chunk.text.char_positions().len() {
                            index = r_chunk.text.char_positions().len() - 1 - new_line;
                        }
                        end_pos = r_chunk.text.char_positions()[index][0];
                        // println!("{} {} {} {:?} {:?} {:?}", end, end_index, index + i_chunk.ind_start, index, line, l);
                        if line >= cursor.end_pos().y{
                            break;
                        }
                        if new_line > 0 {
                            if cursor.start_pos().y != cursor.end_pos().y {
                                if line >= cursor.start_pos().y {
                                    // context().renderer().draw_rect(Vec4::xywh(start_pos, fr_height * line as f32 + offset.y + scroll.y - 2., end_pos - start_pos, fr_height), 0x80909090);
                                    start_pos = 0.0;
                                } else {
                                    start_pos = end_pos;
                                }
                            }
                            line += 1;
                        }
                    }
                }
                // context().renderer().draw_rect(Vec4::xywh(start_pos, fr_height * line as f32 + offset.y + scroll.y - 2., end_pos - start_pos, fr_height), 0x80909090);
                i += 1;
            }

            offset.offset((-10., 0.));
            // Draw line numbers to the left side of the text block
            for line in start_line..end_line {
                if !self.line_texts.contains_key(&line) {
                    let text = text!(
                        AlignH(Alignment::Right),
                        Size(self.text_size),
                        FormatItem::Color(0xff909090.to_color()),
                        format!("{}", line),
                    );
                    self.line_texts.insert(line, text);
                }
                let a= self.line_texts.get(&line).unwrap();
                fr.draw_string(a.clone(), offset + (0, line as f32 * fr_height) + (0,scroll.y()));
            }
            // Finish();
            // println!("render: {:?} tex {:?}", st.elapsed(), t_render);
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
                                    c.up(shift_pressed)
                                }
                                self.correct_cursor(false);
                            }
                            Key::Down => {
                                for c in &mut self.editor.cursors {
                                    c.down(shift_pressed)
                                }
                                self.correct_cursor(false);
                            }
                            Key::Right => self.move_cursors_right(),
                            Key::Left => {
                                self.move_left(false, shift_pressed);
                            }

                            Key::Space => {
                                self.editor.add_change(Change::Add(" ".to_string()));
                                self.apply_changes();
                                self.move_cursors_right();
                            }
                            Key::Enter => {
                                self.editor.add_change(Change::Add("\n".to_string()));
                                self.apply_changes();

                                self.move_cursors_right();
                            }
                            Key::Backspace => {
                                self.move_left(true, true);

                                self.editor.add_change(Change::Delete);
                                self.apply_changes();

                                for c in &mut self.editor.cursors {
                                    let pos = c.start_pos();
                                    c.position(pos, shift_pressed);
                                }
                            }
                            Key::Delete => {
                                self.editor.add_change(Change::Delete);
                                self.apply_changes();
                            }
                            Key::End => {
                                for c in &mut self.editor.cursors {
                                    c.position(Vec2::new(usize::MAX, c.pos.y), shift_pressed);
                                }
                                self.correct_cursor(false);
                            }
                            Key::Tab => {
                                self.editor.add_change(Change::Add("\t".to_string()));
                                self.apply_changes();
                                self.move_cursors_right();
                            }
                            Key::Home => {
                                for c in &mut self.editor.cursors {
                                    c.position(Vec2::new(0, c.pos.y), shift_pressed);
                                }
                                self.correct_cursor(false);
                            }
                            _ => {}
                        },
                    Some(mut pressed) => {
                        if mods.difference(Modifiers::Shift).is_empty() {
                            let st = Instant::now();
                            if mods.contains(Modifiers::Shift) {
                                let upper = pressed.to_ascii_uppercase();
                                if upper == pressed {
                                    pressed = get_shifted(pressed.chars().nth(0).unwrap()).to_string()
                                } else {
                                    pressed = upper;
                                }
                            }
                            self.editor.add_change(Change::Add(pressed));
                            self.apply_changes();
                            let d = st.elapsed();

                            self.move_cursors_right();
                            println!("{:?} {:?}", d, self.editor.chunks.len());
                        }
                    }
                }
            }
            Event::PreRender => {
                let (scroll_speed, easing) = (2., Easing::Sin);
                let scroll_mult = 40.;

                self.scroll.0.borrow_mut().animate_to(self.target_scroll.x * scroll_mult, scroll_speed, easing);
                self.scroll.1.borrow_mut().animate_to(self.target_scroll.y * scroll_mult, scroll_speed, easing);
            }
            Event::Scroll(x, y) => {
                if context().keyboard().is_pressed(&Key::LeftControl) {
                    self.text_size += y * 0.1;
                    self.changed = true;
                } else {
                    self.target_scroll += (*x, *y);
                    self.target_scroll.x = self.target_scroll.x.min(0.);
                }
            }
            Event::MouseClick(button, action) => {
                if action == &Action::Release {
                    return false;
                }

                self.cursor_to_mouse(context().keyboard().is_pressed(&Key::LeftShift) || context().keyboard().is_pressed(&Key::RightShift));
            }
            Event::MousePos(_, _) => {
                if context().window().mouse().is_pressed(MouseButton::Left) {
                    self.cursor_to_mouse(true);
                }
            }
            _ => {},
        }
        let d = st.elapsed();
        // if self.changed {
        //     println!("{:?}", d);
        // }
        self.changed = tmp_changed || self.changed;
        match event {
            Event::PostRender => {
                self.changed = false;
            }
            _ => {}
        }
        false
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        self.changed
        // true
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        Some(&mut self.anim_registry)
    }
}