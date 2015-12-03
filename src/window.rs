use std::fmt;
use std::ops::Range;
use std::iter::IntoIterator;

use types::{State, BoardView};
use board::Board;

#[derive(Clone, Copy)]
pub struct Window<'a> {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    board: &'a Board
}

impl<'a> Window<'a> {
    pub fn new(board: &'a Board, min_x: i32, min_y: i32,
               max_x: i32, max_y: i32) -> Window<'a> {
        Window {
            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y,
            board: board
        }
    }
}

impl<'a> IntoIterator for Window<'a> {
    type Item = (i32, i32, State);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Iter<'a> {
        Iter {
            window: self,
            x: self.min_x,
            // TODO: Change to self.min_y...self.max_y when available
            ys: self.min_y..self.max_y + 1
        }
    }
}

impl<'a> fmt::Display for Window<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Change this to self.min_x...self.max_x when available
        for x in self.min_x..self.max_x + 1 {
            for y in self.min_y..self.max_y + 1 {
                let c = match self.board.cell_state(x, y) {
                    State::Alive => 'X',
                    State::Dead => '_'
                };
                try!(fmt::Write::write_char(fmt, c));
            }
            if x != self.max_x {
                try!(fmt::Write::write_char(fmt, '\n'));
            }
        }
        Ok(())
    }
}

pub struct Iter<'a> {
    window: Window<'a>,
    x: i32,
    ys: Range<i32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (i32, i32, State);
    fn next(&mut self) -> Option<(i32, i32, State)> {
        loop {
            if self.x == self.window.max_x + 1 {
                return None;
            }
            match self.ys.next() {
                Some(y) => {
                    let state = self.window.cell_state(self.x, y);
                    return Some((self.x, y, state))
                },
                None => {
                    self.ys = 0..self.window.max_y + 1;
                    self.x += 1;
                }
            }
        }
    }
}

impl<'a> BoardView for Window<'a> {
    fn cell_state(&self, x: i32, y: i32) -> State {
        self.board.cell_state(x, y)
    }

    fn as_window<'b>(&'b self) -> Window<'b> {
        *self
    }

    fn iter<'b>(&'b self) -> Iter<'b> {
        Iter {
            window: *self,
            x: self.min_x,
            ys: self.min_y..self.max_y + 1
        }
    }


    fn window<'b>(&'b self, min_x: i32, min_y: i32, max_x: i32, max_y: i32)
                  -> Window<'b> {
        Window {
            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y,
            board: self.board
        }
    }

    fn window_offsets<'b>(&'b self, offset_x: i32, offset_y: i32,
                          dim_x: i32, dim_y: i32) -> Window<'b> {
        Window {
            min_x: self.min_x + offset_x,
            min_y: self.min_y + offset_y,
            max_x: self.min_x + offset_x + dim_x,
            max_y: self.min_y + offset_y + dim_y,
            board: self.board
        }
    }
}
