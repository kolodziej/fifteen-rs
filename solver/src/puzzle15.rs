
#[derive(Clone, Copy)]
pub enum PossibleMoves {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn column(zero_field: u64) -> u64 {
    zero_field % 4
}

fn row(zero_field: u64) -> u64 {
    zero_field / 4
}

pub fn can_move_up(zero_field: u64) -> bool {
    row(zero_field) < 3
}

pub fn can_move_down(zero_field: u64) -> bool {
    row(zero_field) > 0
}
pub fn can_move_left(zero_field: u64) -> bool {
    column(zero_field) < 3
}
pub fn can_move_right(zero_field: u64) -> bool {
    column(zero_field) > 0
}

pub fn get_right_field(zero_field: u64) -> u64 {
    zero_field + 1
}

pub fn get_left_field(zero_field: u64) -> u64 {
    zero_field - 1
}

pub fn get_up_field(zero_field: u64) -> u64 {
    zero_field - 4
}

pub fn get_down_field(zero_field: u64) -> u64 {
    zero_field + 4
}
