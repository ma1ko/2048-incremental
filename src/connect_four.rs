use console::Key;
use std::{borrow::BorrowMut, cell::Cell, convert::identity, fmt::Display};
use termion::raw::IntoRawMode;
fn main() {
    let raw_term = std::io::stdout().into_raw_mode().unwrap();
    let mut board = Board::new();
    board.play();
}
const SIZE: usize = 10;
type Line<T> = [T; SIZE];
struct Board<T> {
    _a: Cleanup,
    board: [Line<T>; SIZE],
    pointer: usize,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Chip {
    Red,
    Yellow,
}
use Chip::*;
#[derive(Default, Clone)]
struct Field {
    x: usize,
    y: usize,
    content: Cell<Option<Chip>>,
}
impl Field {
    fn full(&self) -> bool {
        self.content.get().is_some()
    }
    fn color(&self) -> Option<Chip> {
        self.content.get().clone()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self.content.get();
        match content.as_ref() {
            Some(Red) => write!(f, "\u{001b}[34mO\u{001b}[0m"),
            Some(Yellow) => write!(f, "\u{001b}[32mX\u{001b}[0m"),
            None => write!(f, " "),
        }
    }
}
impl Board<Field> {
    fn column(&self, index: usize) -> Vec<Field> {
        let mut col = Vec::new();
        for i in 0..SIZE {
            col.push(self.board[i][index].clone());
        }
        col
    }
    fn diag(&self, index: usize) -> Vec<Vec<Field>> {
        let board = &self.board;
        let mut col = Vec::new();
        for i in 0..SIZE {
            let mut pos1 = Vec::new();
            let mut pos2 = Vec::new();
            let mut neg1 = Vec::new();
            let mut neg2 = Vec::new();
            // positive
            for j in 0..SIZE {
                if i + j >= SIZE {
                    break;
                }
                // println!("Pos: {} {}", i, i + j);
                // println!("Neg: {} {}", SIZE - 1 - j, SIZE - 1 - i - j);
                pos1.push(board[i + j][j].clone());
                pos2.push(board[j][i + j].clone());
                neg1.push(board[SIZE - j - 1][i + j].clone());
                neg2.push(board[SIZE - 1 - i - j][j].clone());
            }
            col.push(pos1);
            col.push(pos2);
            col.push(neg1);
            col.push(neg2);
        }
        col
    }
    fn check(&self) -> bool {
        // check lines
        self.board
            .iter()
            .map(|line| line.windows(4).any(Self::win))
            .any(|x| x == true)
            // check columns
            || (0..SIZE)
                .into_iter()
                .map(|i| {
                    let column = self.column(i);
                    column.windows(4).any(Self::win)
                })
                .any(|x| x == true)
            // check diagonals
            || (0..SIZE)
                .into_iter()
                .map(|i| {
                    let column = self.diag(i);
                    column
                        .iter()
                        .map(|column| column.windows(4).any(Self::win))
                        .any(|x| x == true)
                })
                .any(|x| x == true)
    }
    fn win(win: &[Field]) -> bool {
        win.iter().all(|x| x.color() == Some(Red)) || win.iter().all(|x| x.color() == Some(Yellow))
    }
    fn new() -> Self {
        // println!("\u{001B}[?1049h");
        Board {
            board: Default::default(),
            _a: Cleanup {},
            pointer: 0,
        }
    }
    fn put(&mut self, c: Chip) -> bool {
        if self.board[0][self.pointer].full() {
            return false;
        }

        for i in 0..SIZE {
            if i == SIZE - 1 || self.board[i + 1][self.pointer].full() {
                let field = self.board[i][self.pointer].content.borrow_mut();
                field.replace(Some(c));
                return true;
            }
        }
        unreachable!();
    }
    fn play(&mut self) {
        let term = console::Term::stdout();
        let mut chip = Yellow;
        while !self.check() {
            println!("{}", self);
            let key = term.read_key().unwrap();
            let key = key_to_direction(key);
            match key {
                Right => self.pointer = (self.pointer + 1) % SIZE,
                Left => self.pointer = self.pointer.wrapping_sub(1).min(SIZE - 1),
                Enter => {
                    if !self.put(chip) {
                        continue;
                    }
                    chip = match chip {
                        Yellow => Red,
                        Red => Yellow,
                    };
                }
                Nowhere => continue,
            }
        }
        println!("{}", self);
        println!("{:?} wins", chip);
    }
}
impl Display for Board<Field> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\u{001b}[2J")?;
        write!(f, "{} |\r\n", " ".repeat(self.pointer))?;
        let _ = self
            .board
            .iter()
            .map(|line| {
                write!(f, "|")?;
                let _ = line
                    .iter()
                    .map(|field| write!(f, "{}", field))
                    .collect::<Vec<_>>();
                write!(f, "|\r\n")
            })
            .collect::<Vec<_>>();
        write!(f, "\r\n")
    }
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        let raw_term = std::io::stdout().into_raw_mode().unwrap();
        raw_term.suspend_raw_mode().unwrap();
        // println!("\u{001B}[?1049l");
        // println!("Left term");
    }
}

fn key_to_direction(key: Key) -> Direction {
    use console::Key::*;
    use Direction::*;
    match key {
        console::Key::Enter => Direction::Enter,
        ArrowLeft => Left,
        ArrowRight => Right,
        Char('a') => Left,
        Char('d') => Right,
        _ => Nowhere,
    }
}
use Direction::*;
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Enter,
    Nowhere,
}
