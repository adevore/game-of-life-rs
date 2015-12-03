use std::io::prelude::*;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;

use types::{State, BoardView};
use board::Board;
use transition::next_state;

mod window;
mod types;
mod board;
mod transition;


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
    let mut sink = std::io::stdout();
    //let mut sink = std::io::sink();
    for _ in 0..100000 {
        writeln!(sink, "{}", board).unwrap();
        writeln!(sink, "=========================").unwrap();
        board = tick(&board);
        std::thread::sleep(Duration::from_millis(200));
    }
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
    use types::State::{Alive, Dead};
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

#[test]
fn test_window() {
    let board = Board::from_lines(vec![Ok("_____"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("__X__"),
                                       Ok("_____")]).unwrap();
    board.window(2, 2, 4, 2);
}
