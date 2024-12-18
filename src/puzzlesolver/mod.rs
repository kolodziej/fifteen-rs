pub enum BoardPtr {
    Pointer(Box<dyn Board>),
    State(u64),
}

pub trait Board {
    fn up(&self) -> Result<BoardPtr, ()>;
    fn down(&self) -> Result<BoardPtr, ()>;
    fn left(&self) -> Result<BoardPtr, ()>;
    fn right(&self) -> Result<BoardPtr, ()>;

    fn solved(&self) -> bool;
}

pub struct IntBoard {
    state: u64
}
