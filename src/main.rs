use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;
use clap::Parser;
use std::clone::Clone;
use std::fmt;
use rand::{Rng, thread_rng};
use std::collections::{VecDeque, HashSet};

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short, long)]
    algorithm: String,

    #[arg(short, long)]
    game_path: String
}

#[derive(Clone)]
struct GameState {
    state: u64,
    empty_field: u8,
}

struct GameStateNode {
    game_state: GameState,
    moves: RefCell<[Option<Rc<GameStateNode>>; 4]>,
}

impl GameStateNode {
    const UP: usize = 0;
    const DOWN: usize = 1;
    const LEFT: usize = 2;
    const RIGHT: usize = 3;

    fn new(game_state: GameState) -> Rc<GameStateNode> {
        Rc::new(GameStateNode {
            game_state,
            moves: RefCell::new([
                None, None, None, None
            ])
        })
    }

    fn get_move(&self, direction: usize) -> Option<Rc<GameStateNode>> {
        let mut moves = self.moves.borrow_mut();
        if moves[direction].is_none() && self.game_state.can_move(direction) {
            moves[direction] = Some(GameStateNode::new(self.game_state.movement(direction)));
        }
        moves[direction].as_ref().cloned()
    }
}

struct GameStateGraph {
    init_state: Rc<GameStateNode>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..=15 {
            let field = i;
            if i % 4 < 3 {
                write!(f, "{} ", self.get_field(field))?;
            } else {
                write!(f, "{}\n", self.get_field(field))?;
            }
        }
        write!(f, "empty field: {}\n", self.empty_field)?;

        Ok(())
    }
}

impl GameState {
    fn from_string(str: &str) -> GameState {
        let mut game_state: GameState = GameState {state: 0, empty_field: 0};
        // let pieces = str.split_whitespace().collect();
        for (index, piece) in str.split_whitespace().enumerate() {
            let value = piece.parse::<u8>().unwrap();
            if value <= 15 {
                game_state.set_field(index as u8, value);
            }
        }

        game_state
    }

    fn create_sorted() -> GameState {
        let mut game_state = GameState {state: 0, empty_field: 0};
        for i in 0..=15 {
            game_state.set_field(i, i+1);
        }

        game_state
    }

    fn create_random() -> GameState {
        let mut rng = rand::thread_rng();
        let mut game_state = GameState {state: 0, empty_field: 16};
        let mut value = 0;
        loop {
            let place: u8 = (rng.gen::<u32>() % 16) as u8;
            if game_state.get_field(place) == 0 && game_state.empty_field != place {
                game_state.set_field(place, value);
                value += 1;
            }

            if value == 16 {
                break;
            }
        }

        game_state
    }

    fn is_sorted(&self) -> bool {
        self.state == GameState::create_sorted().state
    }

    fn set_field(&mut self, field: u8, value: u8) {
        let next_value = value % 16;
        // set zero on 4 bits of field
        self.state = self.state & (u64::MAX ^ (0b1111 << (4 * (field as u64))));
        // set field value
        self.state = self.state ^ ((next_value as u64) << (4 * (field as u64)));

        if next_value == 0 {
            self.empty_field = field;
        }
    }

    fn get_field(&self, field: u8) -> u8 {
        ((self.state >> ((4 * field) as u64)) & 0b1111) as u8
    }

    fn is_empty(&self, field: u8) -> bool {
        self.get_field(field) == 0
    }

    fn row_index(field: u8) -> u8 {
        field / 4
    }

    fn column_index(field: u8) -> u8 {
        field % 4
    }

    fn indexes(field: u8) -> (u8, u8) {
        (GameState::row_index(field), GameState::column_index(field))
    }

    fn can_be_swapped(&self, f1: u8, f2: u8) -> bool {
        /* Fields can be swapped if they are in the same row and column index differs by one OR they are in the same column and row index differs by one. */
        let (row1, col1) = GameState::indexes(f1);
        let (row2, col2) = GameState::indexes(f2);
        println!("can_be_swapped({}, {})? row1: {}, row2: {}, col1: {}, col2: {}", f1, f2, row1, row2, col1, col2);

        (row1 == row2 && (i32::abs((col1 as i32) - (col2 as i32)) == 1)) || (col1 == col2 && (i32::abs((row1 as i32) - (row2 as i32)) == 1))
    }

    fn swap_fields(&mut self, f1: u8, f2: u8) {
        if self.can_be_swapped(f1, f2) {
            let temp = self.get_field(f1);
            self.set_field(f1, self.get_field(f2));
            self.set_field(f2, temp);
        } else {
            panic!("fields {}, {} cannot be swapped", f1, f2);
        }
    }

    fn can_move(&self, direction: usize) -> bool {
        match direction {
            GameStateNode::UP => self.can_move_up(),
            GameStateNode::DOWN => self.can_move_down(),
            GameStateNode::LEFT => self.can_move_left(),
            GameStateNode::RIGHT => self.can_move_right(),
            _ => panic!("incorrect direction: {}", direction)
        }
    }

    fn can_move_right(&self) -> bool {
        // empty field needs piece on the left
        self.empty_field % 4 > 0
    }

    fn can_move_left(&self) -> bool {
        // empty field needs piece on the right
        self.empty_field % 4 < 3
    }

    fn can_move_up(&self) -> bool {
        // empty field needs a piece below
        self.empty_field / 4 < 3
    }

    fn can_move_down(&self) -> bool {
        // empty field needs a piece above
        self.empty_field / 4 > 0
    }

    fn field_to_move_right(&self) -> u8 {
        self.empty_field - 1
    }

    fn field_to_move_left(&self) -> u8 {
        self.empty_field + 1
    }

    fn field_to_move_up(&self) -> u8 {
        self.empty_field - 4
    }

    fn field_to_move_down(&self) -> u8 {
        self.empty_field + 4
    }

    fn move_up(&self) -> GameState {
        let mut next_state = self.clone();

        let field_down = next_state.empty_field + 4;
        next_state.swap_fields(next_state.empty_field, field_down);

        next_state
    }

    fn move_down(&self) -> GameState {
        let mut next_state = self.clone();

        let field_up = next_state.empty_field - 4;
        next_state.swap_fields(next_state.empty_field, field_up);

        next_state
    }

    fn move_left(&self) -> GameState {
        let mut next_state = self.clone();

        let field_right = next_state.empty_field + 1;
        next_state.swap_fields(next_state.empty_field, field_right);

        next_state
    }

    fn move_right(&self) -> GameState {
        let mut next_state = self.clone();

        let field_left = next_state.empty_field - 1;
        next_state.swap_fields(next_state.empty_field, field_left);

        next_state
    }

    fn movement(&self, direction: usize) -> GameState {
        match direction {
            GameStateNode::UP => self.move_up(),
            GameStateNode::DOWN => self.move_down(),
            GameStateNode::RIGHT => self.move_right(),
            GameStateNode::LEFT => self.move_left(),
            _ => panic!("incorrect direction"),
        }
    }
}

fn bfs_solver(init_state: Rc<GameStateNode>) -> u64 {
    let mut queue: VecDeque<Rc<GameStateNode>> = VecDeque::new();
    let mut visited_states: HashSet<u64> = HashSet::new();
    queue.push_back(init_state);

    let mut n: u64 = 0;

    loop {
        let next_state = queue.pop_front().unwrap();
        if visited_states.contains(&next_state.game_state.state) {
            continue
        }

        visited_states.insert(next_state.game_state.state);

        if next_state.game_state.is_sorted() {
            break;
        }

        for direction in 0..=3 {
            if let Some(S) = next_state.get_move(direction) { queue.push_back(S) }
        }

        if queue.is_empty() {
            println!("queue is empty and board is not sorted :(");
            break;
        }

        n += 1
    }

    n
}

fn main() {
    let args = Arguments::parse();

    println!("selected algorithm: {}", args.algorithm);
    println!("selected game: {}", args.game_path);

    let mut game_file = File::open(args.game_path).expect("Could not open file");
    let mut game_string = String::new();
    game_file.read_to_string(&mut game_string).expect("could not read contents to string");

    println!("game_string: {}",game_string);
    let game_state = GameState::from_string(&game_string);

    println!("game_state: {}", game_state);


    let n1 = GameStateNode::new(GameState::create_sorted());
    println!("n1: {}", n1.game_state);
    let n2 = n1.get_move(GameStateNode::DOWN).unwrap();
    println!("n2: {}", n2.game_state);
    let n3 = n2.get_move(GameStateNode::RIGHT).unwrap();
    println!("n3: {}", n3.game_state);


    let r1 = GameStateNode::new(GameState::create_random());
    println!("r1: {}", r1.game_state);

    let moves = bfs_solver(r1);
    println!("solved r1 in {} moves", moves);
}