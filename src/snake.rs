struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        let raw_term = std::io::stdout().into_raw_mode().unwrap();
        raw_term.suspend_raw_mode().unwrap();
        // println!("\u{001B}[?1049l");
        println!("Left term");
    }
}
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
    Nowhere,
    Last,
}
use Direction::*;
fn key_to_direction(key: Key) -> Direction {
    use console::Key::*;
    use Direction::*;
    match key {
        ArrowUp => Up,
        ArrowDown => Down,
        ArrowLeft => Left,
        ArrowRight => Right,
        Char('w') => Up,
        Char('a') => Left,
        Char('s') => Down,
        Char('d') => Right,
        _ => Nowhere,
    }
}
fn main() {
    let _cleanup = Cleanup;
    println!("\u{001B}[?1049h");
    let mut board: Board = Default::default();
    for i in 0..SIZE {
        for j in 0..SIZE {
            board[i][j] = Rc::new(Field::new(i, j));
        }
    }
    let snake1 = Snake::new(board[2][2].clone(), true); //head
    let snake2 = Snake::new(board[5][2].clone(), true); //head
    let mut game = Game {
        board,
        snake1: snake1.clone(),
        snake2: snake2.clone(),
    };
    game.board[3][2].set_fruit();
    game.board[3][4].set_fruit();
    game.board[0][0].set_fruit();

    println!("{}", game);

    let mut term = console::Term::stdout();
    let stdin_channel = spawn_stdin_channel();
    let raw_term = std::io::stdout().into_raw_mode().unwrap();
    let mut player1moved = true;
    let mut player2moved = true;
    loop {
        std::thread::sleep(Duration::from_millis(500));
        while let Some(key) = match stdin_channel.try_recv() {
            Ok(key) => Some(key),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        } {
            let dir = key_to_direction(key.clone());
            use console::Key::*;
            match key {
                ArrowUp | ArrowDown | ArrowRight | ArrowLeft => {
                    if !player1moved {
                        player1moved = true;
                        game.move_to(dir, snake1.clone());
                    }
                }
                Char('w') | Char('a') | Char('s') | Char('d') => {
                    if !player2moved {
                        player2moved = true;
                        game.move_to(dir, snake2.clone())
                    }
                }
                Char('q') => {
                    raw_term.suspend_raw_mode().unwrap();
                    panic!("Quit")
                }
                _ => {}
            }
        }
        if !player1moved {
            game.move_to(Last, snake1.clone());
        }
        player1moved = false;
        if !player2moved {
            game.move_to(Last, snake2.clone());
        }
        player2moved = false;
        println!("{}", game);
    }
    // raw_term.suspend_raw_mode().unwrap();
    // println!("{:?}", buf);
}
fn spawn_stdin_channel() -> Receiver<Key> {
    let term = console::Term::stdout();
    let (tx, rx) = channel::<Key>();
    std::thread::spawn(move || loop {
        let key = term.read_key().unwrap();
        tx.send(key).unwrap();
    });
    rx
}
use console::Key;
use rand::Rng;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::default::Default;
use std::fmt::Display;
use std::io::Read;
use std::rc::Rc;
use std::sync::mpsc::*;
use std::time::Duration;
use termion::raw::IntoRawMode;
const SIZE: usize = 10;

struct Game {
    board: Board,
    snake1: Rc<Snake>,
    snake2: Rc<Snake>,
}
impl Game {
    fn coordinates_from_direction(
        &self,
        direction: Direction,
        x: isize,
        y: isize,
        snake: Rc<Snake>,
    ) -> (isize, isize) {
        let (x, y) = match direction {
            Up => (x - 1, y),
            Down => (x + 1, y),
            Left => (x, y - 1),
            Right => (x, y + 1),
            Nowhere => (x, y),
            Last => self.coordinates_from_direction(snake.last_movement.get(), x, y, snake),
        };
        (x, y)
    }
    fn move_to(&mut self, direction: Direction, snake: Rc<Snake>) {
        if direction != Last {
            snake.last_movement.set(direction);
        }
        let (x, y) = {
            let field = snake.field.borrow();
            let (x, y) = (field.x as isize, field.y as isize);
            self.coordinates_from_direction(direction, x, y, snake.clone())
        };
        let new_field = self.board[(x.rem_euclid(SIZE as isize) as usize)]
            [y.rem_euclid(SIZE as isize) as usize]
            .clone();
        let spawn = snake.move_to(new_field);
        if spawn {
            self.place_fruit()
        }
    }
    fn place_fruit(&mut self) {
        loop {
            let mut rand = rand::thread_rng();
            let x = rand.gen_range(0..SIZE);
            let y = rand.gen_range(0..SIZE);
            let field = &self.board[x][y];
            if !field.is_occupied() && !field.has_fruit() {
                self.board[x][y].set_fruit();
                break;
            }
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wall = "-".repeat(SIZE * 2 + 1);
        let _ = self
            .board
            .iter()
            .map(|row| {
                write!(f, "{}\r\n", &wall)?;
                write!(f, "|")?;
                let _ = row
                    .iter()
                    .map(|cell| write!(f, "{}", cell))
                    .collect::<Vec<_>>();
                write!(f, "\r\n")
            })
            .collect::<Vec<_>>();
        write!(f, "{}\r\n", wall)
    }
}

type Board = [Line; SIZE];
type Line = [Rc<Field>; SIZE];

#[derive(Default)]
struct Field {
    x: usize,
    y: usize,
    snake: RefCell<Option<Rc<Snake>>>,
    fruit: Cell<bool>,
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref snake) = *self.snake.borrow() {
            write!(f, "{}", snake)?
        } else if self.fruit.get() {
            // write!(f, "O")?
            write!(f, "\u{001b}[34mO\u{001b}[0m")?
        } else {
            write!(f, " ")?
        }

        write!(f, "|")
    }
}

impl Field {
    fn new(x: usize, y: usize) -> Field {
        Field {
            x,
            y,
            ..Default::default()
        }
    }
    fn is_occupied(&self) -> bool {
        self.snake.borrow().is_some()
    }
    fn occupy(&self, snake: Rc<Snake>) {
        if self.snake.borrow().is_some() {
            // println!("\u{001B}[?1049l");
            panic!("You died");
        }
        self.snake.replace(Some(snake));
    }
    fn unoccupy(&self) -> Rc<Snake> {
        self.snake.replace(None).unwrap()
    }
    fn set_fruit(&self) {
        self.fruit.set(true);
    }
    fn has_fruit(&self) -> bool {
        self.fruit.get()
    }
    fn rm_fruit(&self) {
        self.fruit.set(false);
    }
}

struct Snake {
    next: RefCell<Option<Rc<Snake>>>,
    field: RefCell<Rc<Field>>,
    is_head: bool,
    last_movement: Cell<Direction>,
}

impl Snake {
    fn new(field: Rc<Field>, is_head: bool) -> Rc<Snake> {
        let snake = Rc::new(Snake {
            next: RefCell::new(None),
            field: RefCell::new(field.clone()),
            is_head,
            last_movement: Cell::new(Nowhere),
        });
        field.occupy(snake.clone());
        snake
    }
    fn move_to(&self, field: Rc<Field>) -> bool {
        let snake = self.field.borrow_mut().unoccupy();
        field.occupy(snake);
        let old_field = self.field.replace(field.clone());
        if field.has_fruit() {
            field.rm_fruit();
            let part = Snake::new(old_field, false);
            let next = self.next.replace(Some(part.clone()));
            part.next.replace(next);
            return true; // ate fruit
        } else if let Some(ref mut next) = *self.next.borrow_mut() {
            next.move_to(old_field);
        }
        return false;
    }
}
impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_head {
            write!(f, "\u{001b}[32mH\u{001b}[0m")
        } else if self.next.borrow().is_some() {
            write!(f, "\u{001b}[31mX\u{001b}[0m")
        } else {
            write!(f, "\u{001b}[33mT\u{001b}[0m")
        }
    }
}
