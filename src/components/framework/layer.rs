use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use gl::DrawArraysInstanced;
use crate::components::context::context;
use crate::components::framework::ui_traits::{TickResult, UIHandler};
use crate::components::framework::event::RenderPass;
use crate::components::render::color::ToColor;
use crate::components::render::stack::State::Texture2D;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::buffer::{Buffer, VertexArray};
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::types::{GLsizei, GLuint};
use crate::gl_binds::gl20::*;
use crate::gl_binds::gl30::BindVertexArray;
use crate::gl_binds::gl41::{BindFramebuffer, FRAMEBUFFER};

pub struct Layer {
    // Values: Framebuffer ID, Grid State, Draw vertices, Rendered State
    framebuffers: HashMap<RenderPass, (u32, Vec<Vec<u8>>, VertexArray, bool)>,
    elements: Vec<Box<dyn UIHandler>>,

    grid_size: Vec2<usize>,
}

impl Layer {
    pub fn new(grid_size: impl Into<Vec2<usize>>) -> Self {
        Layer {
            framebuffers: HashMap::new(),
            elements: vec![],
            grid_size: grid_size.into(),
        }
    }

    pub unsafe fn fb(&mut self, render_pass: &RenderPass) -> &mut Framebuffer {
        if !self.framebuffers.contains_key(render_pass) {
            let width = self.grid_size.x.max(1);
            let height = self.grid_size.y.max(1);
            self.framebuffers.insert(render_pass.clone(), (context().fb_manager().create_fb(RGBA).unwrap(), vec![vec![0; width]; height], VertexArray::new(), false));
            self.build_grid_vao(render_pass, self.grid_size.x.max(1), self.grid_size.y.max(1));
        }
        let fb_id = self.framebuffers.get(render_pass).unwrap().0;
        context().fb_manager().fb(fb_id)
    }

    pub unsafe fn tick(&mut self, render_pass: &RenderPass) -> TickResult {
        for e in &mut self.elements {
            let has_animated = match e.animations() {
                None => false,
                Some(reg) => reg.has_changed(),
            };
            if has_animated {
                return TickResult::RedrawLayout;
            } else {
                let v = e.tick(render_pass);
                if !v.is_valid() {
                    return v;
                }
            }
        }
        TickResult::Valid
    }

    pub unsafe fn mark_dirty(&mut self, pass: &RenderPass, area: impl Into<Vec4>) {
        if !self.framebuffers.contains_key(pass) { return; }
        let (fb_id, grid, _, _) = self.framebuffers.get_mut(pass).unwrap();
        let fb_id = *fb_id;
        let area = area.into();

        let fb_res = context().fb_manager().fb(fb_id).size();

        let grid_res_x = fb_res.x as f32 / grid.first().unwrap().len() as f32;
        let grid_res_y = fb_res.y as f32 / grid.len() as f32;
        let grid_area = (
            (area.left() / grid_res_x).floor() as usize,
            (area.top() / grid_res_y).floor() as usize,
            (area.right() / grid_res_x).floor() as usize,
            (area.bottom() / grid_res_y).floor() as usize,
        );

        for y in grid_area.1..=grid_area.3.min(grid.len()-1) {
            for x in grid_area.0..=grid_area.2.min(grid.first().unwrap().len()-1) {
                grid[y][x] = 1;
            }
        }
    }

    pub unsafe fn build_grid_vao(&mut self, pass: &RenderPass, grid_len_x: usize, grid_len_y: usize) {
        let (fb_id, grid, vao, _) = self.framebuffers.get_mut(pass).unwrap();
        let fb = context().fb_manager().fb(*fb_id);
        let fb_res = fb.size();
        let grid_res_x = fb_res.x as f32 / grid.first().unwrap().len() as f32;
        let grid_res_y = fb_res.y as f32 / grid.len() as f32;

        let shader = &mut context().renderer().layer_blend;

        vao.bind();

        let mut t_indices: Vec<u32> = Vec::with_capacity(grid.len() * grid.first().unwrap().len());
        // for i in 0..20 {
        //     t_indices.push(i * 2);
        // }

        let mut p_buf = Buffer::new(ARRAY_BUFFER);
        p_buf.set_values(&t_indices);
        p_buf.attribIPointer(shader.get_attrib_location("index") as u32, 1, UNSIGNED_INT, 1);

        let mut t_buf = Buffer::new(gl::ARRAY_BUFFER);
        t_buf.set_values(&vec![0u8, 1u8, 2u8, 0u8, 2u8, 3u8]);
        t_buf.attribPointer(shader.get_attrib_location("ind") as GLuint, 1, gl::UNSIGNED_BYTE, gl::FALSE, 0);

        vao.add_buffer(p_buf);
        vao.add_buffer(t_buf);

        VertexArray::unbind();
    }

    pub unsafe fn copy_bind_rects(&mut self, pass: &RenderPass, target_fb: u32, target_tex: u32, clear: bool) {
        let (fb_id, grid, vao, rendered) = self.framebuffers.get_mut(pass).unwrap();
        let fb = context().fb_manager().fb(*fb_id);
        let fb_res = fb.size();
        let fb_tex = fb.texture_id();
        let grid_res_x = fb_res.x as f32 / grid.first().unwrap().len() as f32;
        let grid_res_y = fb_res.y as f32 / grid.len() as f32;

        let mut active_indices = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid.first().unwrap().len() {
                // if grid[y][x] == 1 {
                    active_indices.push((x + y * grid.first().unwrap().len()) as u32);
                // }
            }
        }

        vao.get_buffer(0).set_values(&active_indices);

        let shader = &mut context().renderer().layer_blend;

        Disable(BLEND);
        BindFramebuffer(FRAMEBUFFER, target_fb);

        shader.bind();
        shader.u_put_int("u_bottom_tex", vec![2]);
        shader.u_put_int("u_top_tex", vec![1]);
        shader.u_put_float("rect_size", vec![grid_res_x, grid_res_y]);
        shader.u_put_float("uv_rect_size", vec![1.0 / grid.first().unwrap().len() as f32, 1.0 / grid.len() as f32]);
        shader.u_put_int("grid_dims", vec![grid.first().unwrap().len() as u32, grid.len() as u32]);

        ActiveTexture(TEXTURE2);
        BindTexture(TEXTURE_2D, target_tex);

        ActiveTexture(TEXTURE1);
        BindTexture(TEXTURE_2D, fb_tex);

        ActiveTexture(TEXTURE0);

        Texture::unbind();

        // TODO this needs to be in a way where empty grids are ignored. so in other words not a texture mask, since that still renders all...

        vao.bind();

        DrawArraysInstanced(gl::TRIANGLES, 0, 6, active_indices.len() as GLsizei);

        VertexArray::unbind();
        Shader::unbind();
        Enable(BLEND);

        *rendered = true;
    }

    pub fn pre_render_pass(&mut self, pass: &RenderPass) {
        let (_, grid, _, rendered) = self.framebuffers.get_mut(pass).unwrap();
        if *rendered {
            for y in 0..grid.len() {
                for x in 0..grid.first().unwrap().len() {
                    grid[y][x] = 0;
                }
            }
        }
    }

    pub unsafe fn add<H: UIHandler + 'static>(&mut self, el: H) {
        self.elements.push(Box::new(el));
    }

    pub fn elements(&mut self) -> &mut Vec<Box<dyn UIHandler>> {
        &mut self.elements
    }
}