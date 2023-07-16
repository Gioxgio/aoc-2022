use crate::movee::Direction;

#[derive(Clone, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}
impl Position {
    pub fn new() -> Self {
        Self {
            x: 0 as f32,
            y: 0 as f32,
        }
    }

    pub fn mov(&mut self, direction: &Direction) {
        match direction {
            Direction::UP => self.y += 1 as f32,
            Direction::RIGHT => self.x += 1 as f32,
            Direction::DOWN => self.y -= 1 as f32,
            Direction::LEFT => self.x -= 1 as f32,
        }
    }

    pub fn x(&self) -> &f32 {
        &self.x
    }
    pub fn y(&self) -> &f32 {
        &self.y
    }
}
