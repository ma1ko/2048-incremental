use std::collections::HashSet;
use std::{collections::HashMap, fmt::Display};

use rand::Rng;

fn main() {
    let mut board: Board<Field> = Board::new();
    for i in 0..50 {
        for j in 0..20 {
            board.board.insert(Point(i, j), Field::new(Point(i, j)));
        }
    }

    // let mut iter = board.iter_mut();
    // while let Some(mut row) = iter.next() {
    //     while let Some(field) = row.next() {
    //         println!("{:?}", field);
    //     }
    // }
    // println!("{}", board);
    // let mut iter = board.iter_mut();
    // while let Some(field) = iter.next() {
    //     field.left = false;
    //     // println!("{}", field);
    // }
    board.aldous_broder();
    println!("{}", board);

    let mut rng = rand::thread_rng();
    let p = Point(rng.gen_range(0..10), rng.gen_range(0..10));
    board.visit(p, 0);
    println!("{}", board.longest_path);
    println!("{}", board);
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn right(self) -> Self {
        Point(self.0 + 1, self.1)
    }
    fn left(self) -> Self {
        Point(self.0.wrapping_sub(1), self.1)
    }
    fn down(self) -> Self {
        Point(self.0, self.1 + 1)
    }
    fn up(self) -> Self {
        Point(self.0, self.1.wrapping_sub(1))
    }
    fn go(self, d: Direction) -> Self {
        match d {
            Left => self.left(),
            Right => self.right(),
            Up => self.up(),
            Down => self.down(),
        }
    }
}

struct Board<T> {
    board: HashMap<Point, T>,
    longest_path: usize,
}
impl Board<Field> {
    pub fn visit(&mut self, p: Point, mut steps: usize) {
        // println!("{:?}, steps: {}", p, steps);
        let current = self.board.remove(&p);
        if let Some(mut current) = current {
            if current.steps == 0 || current.steps >= steps {
                current.steps = steps;
                self.longest_path = self.longest_path.max(steps);
                // println!("{:?}, Setting: {}", p, steps);
                steps = steps + 1;
                // if current.path.contains(&Left) {
                if !current.wall.contains(&Down) {
                    self.visit(p.left(), steps);
                }
                if let Some(x) = self.board.get(&p.right()) {
                    if !x.wall.contains(&Down) {
                        // if current.path.contains(&Right) {
                        self.visit(p.right(), steps);
                    }
                }
                if !current.wall.contains(&Right) {
                    // if current.path.contains(&Up) {
                    self.visit(p.up(), steps);
                }
                if let Some(x) = self.board.get(&p.down()) {
                    if !x.wall.contains(&Right) {
                        // if current.path.contains(&Down) {
                        self.visit(p.down(), steps);
                    }
                }
            }
            self.board.insert(p, current);
        }
    }
    pub fn aldous_broder(&mut self) {
        let mut rng = rand::thread_rng();
        let mut left = self.iter().count();
        let mut p = Point(rng.gen_range(0..10), rng.gen_range(0..10));
        let mut current = self.board.remove(&p).unwrap();

        while left > 0 {
            if !current.visited {
                current.visited = true;
                left -= 1;
            }
            let direction: Direction = rng.gen_range(0..4).into();
            let new_p = p.go(direction);
            if let Some(mut new) = self.board.remove(&new_p) {
                if !new.visited {
                    // new.path.insert(direction);
                    // current.path.insert(direction.opposite());
                    match direction {
                        Left => {
                            // Left, rem down
                            self.board
                                .get_mut(&current.p.down())
                                .map(|f| f.wall.remove(&Up));
                            current.wall.remove(&Down);
                        }
                        Right => {
                            // right
                            new.wall.remove(&Down);
                            self.board
                                .get_mut(&new.p.down())
                                .map(|f| f.wall.remove(&Up));
                            // current.right = false;
                        }
                        Up => {
                            // up
                            self.board
                                .get_mut(&current.p.right())
                                .map(|f| f.wall.remove(&Left));
                            current.wall.remove(&Right);
                        }
                        Down => {
                            // down
                            self.board
                                .get_mut(&new.p.right())
                                .map(|f| f.wall.remove(&Left));
                            new.wall.remove(&Right);
                        }
                    }
                }
                self.board.insert(p, current);
                current = new;
                p = new_p;
            }
        }
        self.board.insert(p, current);
    }
}
impl<T: 'static> Board<T> {
    pub fn iter(&self) -> BoardIter<'_, T> {
        let f = |p: Point, board: &Board<T>| {
            if board.get(&p.right()).is_some() {
                p.right()
            } else if board.get(&Point(0, p.1 + 1)).is_some() {
                return Point(0, p.1 + 1);
            } else {
                p.right()
            }
        };
        BoardIter::new(self, Box::new(f), Point(0, 0))
    }
    pub fn iter_mut(&mut self) -> BoardIterMut<'_, T> {
        let f = |p: Point, board: &Board<T>| {
            if board.get(&p.right()).is_some() {
                p.right()
            } else if board.get(&Point(0, p.1 + 1)).is_some() {
                return Point(0, p.1 + 1);
            } else {
                p.right()
            }
        };
        BoardIterMut::new(self, Box::new(f), Point(0, 0))
    }
    pub fn rows(&self) -> std::vec::IntoIter<BoardIter<'_, T>> {
        let mut rows = Vec::new();
        let mut y = 0;
        while let Some(_) = self.board.get(&Point(0, y)) {
            let iter = BoardIter::new(self, Box::new(|p, _| p.right()), Point(0, y));
            rows.push(iter);
            y += 1;
        }
        rows.into_iter()
    }
    pub fn columns(&self) -> std::vec::IntoIter<BoardIter<'_, T>> {
        let mut rows = Vec::new();
        let mut x = 0;
        while let Some(_) = self.board.get(&Point(x, 0)) {
            let iter = BoardIter::new(self, Box::new(|p, _| p.down()), Point(x, 0));
            rows.push(iter);
            x += 1;
        }
        rows.into_iter()
    }
    fn new() -> Self {
        Board {
            board: HashMap::new(),
            longest_path: 0,
        }
    }
    fn get(&self, p: &Point) -> Option<&T> {
        self.board.get(&p)
    }
}

impl<T: 'static + Display> Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rows().try_for_each(|mut x| {
            x.try_for_each(|x| write!(f, "{}", x))?;
            writeln!(f, "")
        })
    }
}

type PointFn<T> = dyn Fn(Point, &Board<T>) -> Point;
struct BoardIter<'a, T> {
    current: Point,
    f: Box<PointFn<T>>,
    board: &'a Board<T>,
}
impl<'a, T> BoardIter<'a, T> {
    fn new(board: &'a Board<T>, f: Box<PointFn<T>>, current: Point) -> Self {
        Self { current, board, f }
    }
}
impl<'a, T> Iterator for BoardIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.board.board.get(&self.current);
        self.current = (self.f)(self.current, self.board);
        current
    }
}
struct BoardIterMut<'a, T: 'a> {
    current: Point,
    f: Box<dyn Fn(Point, &Board<T>) -> Point>,
    board: &'a mut Board<T>,
}
impl<'a, T> BoardIterMut<'a, T> {
    fn new(board: &'a mut Board<T>, f: Box<PointFn<T>>, current: Point) -> Self {
        Self { current, board, f }
    }
}
impl<'a, T: 'a> Iterator for BoardIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
        // TODO off by one
        self.current = (self.f)(self.current, &self.board);
        let item: Option<&mut T> = self.board.board.get_mut(&self.current);
        if let Some(item) = item {
            let item = item as *mut T;
            let item = unsafe { &mut *item };
            return Some(item);
        }
        None
    }
}

use Direction::*;
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl From<usize> for Direction {
    fn from(x: usize) -> Self {
        match x {
            0 => Left,
            1 => Up,
            2 => Right,
            3 => Down,
            _ => panic!("Not a direction"),
        }
    }
}
struct Field {
    steps: usize,
    visited: bool,
    p: Point,
    wall: HashSet<Direction>,
    // path: HashSet<Direction>,
}
impl Field {
    fn new(p: Point) -> Self {
        Field {
            p,
            wall: HashSet::from([Left, Right, Up, Down]),
            // path: HashSet::from([]),
            visited: false,
            steps: 0,
        }
    }
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wall = &self.wall;
        write!(f, "\u{001b}[48;2;{};00;{}m", self.steps  , (self.steps * 50).min(50) )?;
        match (
            wall.contains(&Left),
            wall.contains(&Up),
            wall.contains(&Right),
            wall.contains(&Down),
        ) {
            (true, true, true, true) => write!(f, "╬"),
            (false, true, true, true) => write!(f, "╠"),
            (true, false, true, true) => write!(f, "╦"),
            (true, true, false, true) => write!(f, "╣"),
            (true, true, true, false) => write!(f, "╩"),

            (true, true, false, false) => write!(f, "╝"),
            (true, false, true, false) => write!(f, "═"),
            (true, false, false, true) => write!(f, "╗"),
            (false, true, true, false) => write!(f, "╚"),
            (false, true, false, true) => write!(f, "║"),
            (false, false, true, true) => write!(f, "╔"),

            (true, false, false, false) => write!(f, "╴"),
            (false, true, false, false) => write!(f, "╵"),
            (false, false, true, false) => write!(f, "╶"),
            (false, false, false, true) => write!(f, "╷"),
            (false, false, false, false) => write!(f, "x"),
            // _ => write!(f, " "),
        }?;
        write!(f, "\u{001b}[48;2;00;00;00m")
    }
}
