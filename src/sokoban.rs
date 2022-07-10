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
    // println!("\u{001B}[?1049h");

    let field = "\
..WWW...
.WWFW...
.W.BWWWW
WW...BPW
WF.BBFWW
WWWW.WW.
...WFW..
...WWW..";
    let _field = "\
WWW....
WFWWWWW
WFF...W
W.BBBPW
W....WW
WWWWWW";
    let mut player = (0,0);
    let board: Vec<Vec<Rc<Field>>> = field.lines().enumerate().map(|(i,line)| {
        line.chars().enumerate().map(|(j, c)| {
                let mut field = Field::new(i, j);
                match c {
                    'W' => field.set_wall(),
                    'B' => field.set_box(),
                    '.' => {},
                    'F' => field.set_finish(),
                    'P' => player = (i,j),
                    _ => panic!("Got character {}", c),

                }

                Rc::new(field)
        }).collect()
    }).collect();
    let player = Player::new(board[player.0][player.1].clone()); //head
    let mut game = Game {
        board,
        player: player.clone(),
    };
    // game.board[3][2].set_box();
    // game.board[3][4].set_box();
    // game.board[0][0].set_box();

    println!("{}", game);

    // let mut term = console::Term::stdout();
    // let stdin_channel = spawn_stdin_channel();
    let raw_term = std::io::stdout().into_raw_mode().unwrap();
    let term = console::Term::stdout();
    loop {
        // std::thread::sleep(Duration::from_millis(500));
        let key = term.read_key().unwrap();
        let dir = key_to_direction(key.clone());
            use console::Key::*;
        match key {
                ArrowUp | ArrowDown | ArrowRight | ArrowLeft => {
                    game.move_to(dir);
                },
                Char('w') | Char('a') | Char('s') | Char('d') => {
                    game.move_to(dir);
                },
                Char('q') => {
                    raw_term.suspend_raw_mode().unwrap();
                    panic!("Quit")
                },
                _ => {}, 
            }
        // if !player1moved {
        //     game.move_to(Last, snake1.clone());
        // }
        // player1moved = false;
        // if !player2moved {
        //     game.move_to(Last, snake2.clone());
        // }
        // player2moved = false;
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

const SIZE: usize = 10;

struct Game {
    board: Board,
    player: Rc<Player>,
}
impl Game {
    fn coordinates_from_direction(
        &self,
        direction: Direction,
        x: usize,
        y: usize,
    ) -> (usize, usize) {
        let (x, y) = match direction {
            Up => (x - 1, y),
            Down => (x + 1, y),
            Left => (x, y - 1),
            Right => (x, y + 1),
            Nowhere => (x, y),
        };
        (x, y)
    }
    fn move_to(&mut self, direction: Direction) -> bool {
        let (x, y) = {
            let field = self.player.field.borrow();
            let (x, y) = (field.x, field.y);
            self.coordinates_from_direction(direction, x, y)
        };
        let new_field = self.board[x][y].clone();
        if new_field.is_wall() {
            return false;
        };
        if new_field.has_box() {
            let (x, y) = self.coordinates_from_direction(direction, x, y);
            let next_field = self.board[x][y].clone();
            if !next_field.is_wall() && !next_field.has_box() {
                new_field.move_box(next_field);
            }
            else {
                return false;
            }
        }
        let _spawn = self.player.try_move_to(new_field);
        return true;
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

type Board = Vec<Line>;
type Line = Vec<Rc<Field>>;

#[derive(Default)]
struct Field {
    x: usize,
    y: usize,
    player: RefCell<Option<Rc<Player>>>,
    boxx: Cell<bool>,
    wall: bool,
    finish: bool,
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref player) = *self.player.borrow() {
            write!(f, "{}", player)?
        } else if self.boxx.get() && self.finish {
            write!(f, "\u{001b}[32mO\u{001b}[0m")?
        } else if self.boxx.get() {
            write!(f, "\u{001b}[34mO\u{001b}[0m")?
        } else if self.finish {
            write!(f, "\u{001b}[34m.\u{001b}[0m")?
        } else if self.wall {
            write!(f, "\u{001b}[34mW\u{001b}[0m")?
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
    fn set_wall(&mut self) {
        self.wall = true;
    }
    fn set_finish(&mut self) {
        self.finish = true
    }
    fn is_occupied(&self) -> bool {
        self.player.borrow().is_some()
    }
    fn occupy(&self, player: Rc<Player>) {
        if self.player.borrow().is_some() {
            // println!("\u{001B}[?1049l");
            panic!("You died");
        }
        self.player.replace(Some(player));
    }
    fn unoccupy(&self) -> Rc<Player>{
        self.player.replace(None).unwrap()
    }
    fn set_box(&self) {
        self.boxx.set(true);
    }
    fn has_box(&self) -> bool {
        self.boxx.get()
    }
    fn move_box(&self, to: Rc<Field>) {
        self.boxx.set(false);
        to.boxx.set(true);
    }
    fn is_wall(&self) -> bool {
        self.wall
    }
}

struct Player {
    field: RefCell<Rc<Field>>,
}

impl Player {
    fn new(field: Rc<Field>) -> Rc<Player> {
        let player = Rc::new(Player {
            field: RefCell::new(field.clone()),
        });
        field.occupy(player.clone());
        player
    }
    fn try_move_to(&self, field: Rc<Field>) -> bool {
        let player = self.field.borrow_mut().unoccupy();
        field.occupy(player);
        let old_field = self.field.replace(field.clone());
        if field.has_box() {
            return true; // ate fruit
        }
        return false;
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\u{001b}[32mH\u{001b}[0m")
    }
}
