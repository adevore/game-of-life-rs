use ::window::{Window, Iter};

pub trait BoardView {
    fn cell_state(&self, x: i32, y: i32) -> State;
    fn as_window<'a>(&'a self) -> Window<'a>;
    fn window<'a>(&'a self, min_x: i32, min_y: i32, max_x: i32, max_y: i32)
                  -> Window<'a>;
    fn iter<'a>(&'a self) -> Iter<'a>;
    fn window_offsets<'a>(&'a self, offset_x: i32, offset_y: i32,
                          dim_x: i32, dim_y: i32) -> Window<'a>;
    fn neighbor_count(&self, x: i32, y: i32) -> u8 {
        let mut result = 0;
        // TODO: Switch to -1...1 when inclusive range syntax is added
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                if (x_offset, y_offset) == (0, 0) {
                    continue;
                }
                result += match self.cell_state(x + x_offset, y + y_offset) {
                    State::Alive => 1,
                    State::Dead => 0
                };
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead
}

