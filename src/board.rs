use std::collections::HashSet;
use std::io::prelude::*;
use std::io;
use std::cmp;
use std::fmt;

use ::types::{BoardView, State};
use ::window::{Window, Iter};

#[derive(Debug)]
pub enum BoardReadError {
    UnknownCharacter(i32, i32, char),
    IoError(io::Error)
}

impl From<io::Error> for BoardReadError {
    fn from(error: io::Error) -> BoardReadError {
        BoardReadError::IoError(error)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    living: HashSet<(i32, i32)>,
    pub dim_x: i32,
    pub dim_y: i32,
}

impl Board {
    pub fn new<I>(dim_x: i32, dim_y: i32, living: I) -> Board
        where I: IntoIterator<Item=(i32, i32)> {
        Board {
            dim_x: dim_x,
            dim_y: dim_y,
            living: living.into_iter().collect()
        }
    }

    pub fn from_lines<'a, I: 'a, S: AsRef<str>>(lines: I) -> Result<Board, BoardReadError> where I: IntoIterator<Item=io::Result<S>> {
        let mut living = HashSet::new();
        let mut dim_x = 0;
        let mut dim_y = 0;
        for (line, x) in lines.into_iter().zip(0..) {
            let line_owned = try!(line);
            let line = line_owned.as_ref();
            dim_x = x + 1;
            // We only accept 'X' and '_', so byte length is okay
            dim_y = cmp::max(dim_y, line.len() as i32);
            for (c, y) in line.chars().zip(0..) {
                match c {
                    'X' => {
                        living.insert((x, y));
                    },
                    '_' => {},
                    _ => return Err(BoardReadError::UnknownCharacter(x, y, c))
                }
            }
        }
        Ok(Board {
            dim_x: dim_x,
            dim_y: dim_y,
            living: living
        })
    }

}

impl BoardView for Board {
    fn cell_state(&self, x: i32, y: i32) -> State {
        if self.living.contains(&(x, y)) {
            State::Alive
        } else {
            State::Dead
        }
    }

    fn as_window<'a>(&'a self) -> Window<'a> {
        Window::new(self, 0, 0, self.dim_x - 1, self.dim_y - 1)
    }

    fn window<'a>(&'a self, min_x: i32, min_y: i32,
                      max_x: i32, max_y: i32) -> Window<'a> {
        Window::new(self, min_x, min_y, max_x, max_y)
    }

    fn window_offsets<'a>(&'a self, offset_x: i32, offset_y: i32,
                              dim_x: i32, dim_y: i32) -> Window<'a> {
        Window::new(self, offset_x, offset_y,
                    dim_x + offset_x, dim_y + offset_y)
    }

    fn iter<'a>(&'a self) -> Iter<'a> {
        self.as_window().into_iter()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.as_window())
    }
}

#[test]
fn test_new_board() {
    Board::new(5, 5, [(0, 0), (1, 1)].iter().cloned());
}

#[test]
fn test_from_lines() {
    let board = Board::from_lines(vec![Ok("____"),
                                       Ok("_XX_"),
                                       Ok("_XX_"),
                                       Ok("____")]).unwrap();
    let target = Board::new(4, 4, vec![(1, 1), (1, 2), (2, 1), (2, 2)]);
    assert_eq!(board, target);
}

#[test]
fn test_cell_state() {
    let board = Board::new(5, 5, [(0, 0), (1, 1)].iter().cloned());
    assert_eq!(board.cell_state(0, 0), State::Alive);
    assert_eq!(board.cell_state(1, 1), State::Alive);
    assert_eq!(board.cell_state(1, 2), State::Dead)
}

#[test]
fn test_neighbor_count() {
    let board = Board::from_lines(vec![Ok("____"),
                                       Ok("_XX_"),
                                       Ok("_XX_"),
                                       Ok("____")]).unwrap();
    assert_eq!(board.neighbor_count(0, 0), 1);
    assert_eq!(board.neighbor_count(0, 1), 2);
    assert_eq!(board.neighbor_count(0, 2), 2);
    assert_eq!(board.neighbor_count(0, 3), 1);
    assert_eq!(board.neighbor_count(1, 0), 2);
    assert_eq!(board.neighbor_count(1, 1), 3);
    assert_eq!(board.neighbor_count(1, 2), 3);
    assert_eq!(board.neighbor_count(1, 3), 2);
    assert_eq!(board.neighbor_count(2, 0), 2);
    assert_eq!(board.neighbor_count(2, 1), 3);
    assert_eq!(board.neighbor_count(2, 2), 3);
    assert_eq!(board.neighbor_count(2, 3), 2);
    assert_eq!(board.neighbor_count(3, 0), 1);
    assert_eq!(board.neighbor_count(3, 1), 2);
    assert_eq!(board.neighbor_count(3, 2), 2);
    assert_eq!(board.neighbor_count(3, 3), 1);
}

