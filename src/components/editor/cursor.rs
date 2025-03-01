use crate::components::spatial::vec2::Vec2;

#[derive(Debug)]
pub struct Cursor {
    pub(super) pos: Vec2<usize>,
    pub(super) select_pos: Vec2<usize>,
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
        self.position((self.pos.x().max(1)-1, self.pos.y()), expand);
    }
    pub fn right(&mut self, expand: bool) {
        self.position((self.pos.x()+1, self.pos.y()), expand);
    }
    pub fn down(&mut self, expand: bool) {
        self.position((self.pos.x(), self.pos.y()+1), expand);
    }
    pub fn up(&mut self, expand: bool) {
        self.position((self.pos.x(), self.pos.y().max(1)-1), expand);
    }

}