use std::collections::HashSet;
use std::cmp;
use std::ops::Range;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::time::Duration;

#[derive(Debug)]
enum BoardReadError {
    UnknownCharacter(i32, i32, char),
    IoError(io::Error)
}

impl From<io::Error> for BoardReadError {
    fn from(error: io::Error) -> BoardReadError {
        BoardReadError::IoError(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Alive,
    Dead
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Board {
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

    pub fn cell_state(&self, x: i32, y: i32) -> State {
        if self.living.contains(&(x, y)) {
            State::Alive
        } else {
            State::Dead
        }
    }

    pub fn neighbor_count(&self, x: i32, y: i32) -> u8 {
        let mut result = 0;
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

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            x: 0,
            ys: 0..self.dim_y,
            board: self
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.dim_x {
            for y in 0..self.dim_y {
                let c = match self.cell_state(x, y) {
                    State::Alive => 'X',
                    State::Dead => '_'
                };
                try!(fmt::Write::write_char(fmt, c));
            }
            if x + 1 != self.dim_x {
                try!(fmt::Write::write_char(fmt, '\n'));
            }
        }
        Ok(())
    }
}

struct Iter<'a> {
    x: i32,
    ys: Range<i32>,
    board: &'a Board
}

impl<'a> Iterator for Iter<'a> {
    type Item = (i32, i32, State);
    fn next(&mut self) -> Option<(i32, i32, State)> {
        loop {
            if self.x == self.board.dim_x {
                return None;
            }
            match self.ys.next() {
                Some(y) => {
                    let state = self.board.cell_state(self.x, y);
                    return Some((self.x, y, state))
                },
                None => {
                    self.ys = 0..self.board.dim_y;
                    self.x += 1;
                }
            }
        }
    }
}

fn next_state(current_state: State, neighbor_count: u8) -> State {
    use State::{Dead, Alive};
    match (current_state, neighbor_count) {
        (Dead, 0...2) => Dead,
        (Dead, 3) => Alive,
        (Dead, 4...8) => Dead,
        (Alive, 0...1) => Dead,
        (Alive, 2...3) => Alive,
        (Alive, 4...8) => Dead,
        _ => unreachable!()
    }
}

fn tick(board: &Board) -> Board {
    // Totally not right, needs a product
    let cells = board.iter()
        .filter_map(|(x, y, state)| {
            if next_state(state, board.neighbor_count(x, y)) == State::Alive {
                Some((x, y))
            } else {
                None
            }
        });
    Board::new(board.dim_x, board.dim_y, cells)
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut board = {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        Board::from_lines(reader.lines()).unwrap()
    };
    let mut buffer = Vec::new();
    for _ in 0..100000 {
        writeln!(buffer, "{}", board).unwrap();
        //println!("{}", board);
        buffer.clear();
        //println!("=========================");
        board = tick(&board);
        //std::thread::sleep(Duration::new(2, 0));
    }
}

#[test]
fn test_new_board() {
    Board::new(5, 5, [(0, 0), (1, 1)].iter().cloned());
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

#[test]
fn test_lonely() {
    // Any live cell with fewer than two live neighbours dies, as if caused by
    // under-population.
    assert_eq!(next_state(State::Alive, 0), State::Dead);
    assert_eq!(next_state(State::Alive, 1), State::Dead);
}

#[test]
fn test_status_quo() {
    // Any live cell with two or three live neighbours lives on to the next
    // generation.
    assert_eq!(next_state(State::Alive, 2), State::Alive);
    assert_eq!(next_state(State::Alive, 3), State::Alive);
}

#[test]
fn test_crowding() {
    // Any live cell with more than three live neighbours dies, as if by
    // over-population.
    assert_eq!(next_state(State::Alive, 4), State::Dead);
    assert_eq!(next_state(State::Alive, 5), State::Dead);
    assert_eq!(next_state(State::Alive, 6), State::Dead);
    assert_eq!(next_state(State::Alive, 7), State::Dead);
    assert_eq!(next_state(State::Alive, 8), State::Dead);
}

#[test]
fn test_birth() {
    // Any dead cell with exactly three live neighbours becomes a live cell, as
    // if by reproduction.
    assert_eq!(next_state(State::Dead, 3), State::Alive);
}

#[test]
fn test_dead_otherwise_dead() {
    assert_eq!(next_state(State::Dead, 0), State::Dead);
    assert_eq!(next_state(State::Dead, 1), State::Dead);
    assert_eq!(next_state(State::Dead, 2), State::Dead);
    assert_eq!(next_state(State::Dead, 4), State::Dead);
    assert_eq!(next_state(State::Dead, 5), State::Dead);
    assert_eq!(next_state(State::Dead, 6), State::Dead);
    assert_eq!(next_state(State::Dead, 7), State::Dead);
    assert_eq!(next_state(State::Dead, 8), State::Dead);
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
fn test_iter_empty() {
    let board = Board::new(0, 0, vec![]);
    let cells: Vec<_> = board.iter().collect();
    assert_eq!(cells, vec![]);
}

#[test]
fn test_iter_single() {
    let board = Board::new(1, 1, vec![]);
    let cells: Vec<_> = board.iter().collect();
    assert_eq!(cells, vec![(0, 0, State::Dead)]);
}

#[test]
fn test_iter_many() {
    use State::{Alive, Dead};
    let board = Board::from_lines(vec![Ok("____"),
                                       Ok("_XX_"),
                                       Ok("_XX_"),
                                       Ok("____")]).unwrap();

    let target_cells = vec![(0, 0, Dead), (0, 1, Dead), (0, 2, Dead), (0, 3, Dead),
                            (1, 0, Dead), (1, 1, Alive), (1, 2, Alive), (1, 3, Dead),
                            (2, 0, Dead), (2, 1, Alive), (2, 2, Alive), (2, 3, Dead),
                            (3, 0, Dead), (3, 1, Dead), (3, 2, Dead), (3, 3, Dead)];
    let cells: Vec<_> = board.iter().collect();
    assert_eq!(cells, target_cells);
}

#[test]
fn test_tick() {
    let board = Board::new(1, 1, vec![]);
    let next_board = tick(&board);
    let cells: Vec<_> = next_board.iter().collect();
    assert_eq!(cells, vec![(0, 0, State::Dead)]);
}

#[test]
fn test_tick_block() {
    let board = Board::from_lines(vec![Ok("____"),
                                       Ok("_XX_"),
                                       Ok("_XX_"),
                                       Ok("____")]).unwrap();
    let next_board = tick(&board);
    assert_eq!(board, next_board);
}

#[test]
fn test_tick_blinker() {
    let board = Board::from_lines(vec![Ok("_____"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("_____")]).unwrap();
    let next_board = tick(&board);
    let target_board = Board::from_lines(vec![Ok("_____"),
                                              Ok("_____"),
                                              Ok("_XXX_"),
                                              Ok("_____"),
                                              Ok("_____")]).unwrap();
    assert_eq!(next_board, target_board);
}

#[test]
fn test_display_board() {
    use std::fmt::Write;
    let board = Board::from_lines(vec![Ok("_____"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("_____")]).unwrap();
    let mut buffer = String::new();
    write!(buffer, "{}", board).unwrap();
    assert_eq!(&buffer, concat!("_____\n",
                                "__X__\n",
                                "__X__\n",
                                "__X__\n",
                                "_____"));
}
