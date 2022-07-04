fn main() {
    let mut board: Board = Default::default();
    for i in 0..SIZE {
        for j in 0..SIZE {
            board[i][j] = Rc::new(Field::new(i, j));
        }
    }
    let head = Snake::new(board[2][2].clone(), true); //head
    let mut game = Game { board, head };
    game.board[3][2].set_fruit();
    game.board[3][4].set_fruit();
    game.board[0][0].set_fruit();

    println!("{}", game);

      let mut buf = [1; 3];
    let term = console::Term::stdout();
    loop {
        let key = term.read_key().unwrap();
        use console::Key::*;
        match key {
            ArrowUp => game.move_to(-1, 0),
            ArrowDown => game.move_to(1, 0),
            ArrowRight => game.move_to(0, 1),
            ArrowLeft => game.move_to(0, -1),
            _ => {}
        }
        println!("{}", game);
    }
    // println!("{:?}", buf);
}

use rand::Rng;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::default::Default;
use std::fmt::Display;
use std::rc::Rc;
const SIZE: usize = 15;

struct Game {
    board: Board,
    head: Rc<Snake>,
}
impl Game {
    fn move_to(&mut self, dx: isize, dy: isize) {
        let (x, y) = {
            let field = self.head.field.borrow();
            (field.x as isize + dx, field.y as isize + dy)
        };
        let new_field = self.board[(x.rem_euclid(SIZE as isize) as usize)]
            [y.rem_euclid(SIZE as isize) as usize]
            .clone();
        let spawn = self.head.move_to(new_field);
        if spawn {
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
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wall = "-".repeat(SIZE * 2 + 1);
        let _ = self
            .board
            .iter()
            .map(|row| {
                writeln!(f, "{}", &wall)?;
                write!(f, "|")?;
                let _ = row
                    .iter()
                    .map(|cell| write!(f, "{}", cell))
                    .collect::<Vec<_>>();
                writeln!(f)
            })
            .collect::<Vec<_>>();
        writeln!(f, "{}", wall)
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
}

impl Snake {
    fn new(field: Rc<Field>, is_head: bool) -> Rc<Snake> {
        let snake = Rc::new(Snake {
            next: RefCell::new(None),
            field: RefCell::new(field.clone()),
            is_head,
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
