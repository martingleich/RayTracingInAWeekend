use crate::vec2::Vec2f;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Size2i {
    pub width: i32,
    pub height: i32,
}

impl Size2i {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }
    pub fn iterf(self) -> Size2iIterGrid2f {
        Size2iIterGrid2f {
            size: self,
            x: self.width - 1,
            y: -1,
            sx: 1.0 / ((self.width - 1) as f32),
            sy: 1.0 / ((self.height - 1) as f32),
        }
    }
    pub fn count(self) -> usize {
        self.width as usize * self.height as usize
    }
}

pub struct Size2iIterGrid2f {
    size: Size2i,
    x: i32,
    y: i32,
    sx: f32,
    sy: f32,
}

impl Iterator for Size2iIterGrid2f {
    type Item = Vec2f;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.size.width - 1 {
            if self.y == self.size.height - 1 {
                return None;
            }
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        Some(Vec2f {
            x: self.x as f32 * self.sx,
            y: self.y as f32 * self.sy,
        })
    }
}
