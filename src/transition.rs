use types::State;

pub fn next_state(current_state: State, neighbor_count: u8) -> State {
    use types::State::{Dead, Alive};
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
