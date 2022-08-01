use std::{collections::HashMap, fmt::Display};

use rand::Rng;

fn main() {
    let mut board: Board<Field> = Board::new();
    for i in 0..100 {
        for j in 0..100 {
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
    println!();
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
}

struct Board<T> {
    board: HashMap<Point, T>,
}
impl Board<Field> {
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
            let direction = rng.gen_range(0..4);
            let new_p = match direction {
                0 => p.left(),
                1 => p.right(),
                2 => p.up(),
                3 => p.down(),
                _ => panic!("wtf"),
            };
            if let Some(mut c) = self.board.remove(&new_p) {
                if !c.visited {
                    match direction {
                        0 => {
                            c.right = false;
                            current.left = false;
                        }
                        1 => {
                            c.left = false;
                            current.right = false;
                        }
                        2 => {
                            c.down = false;
                            current.up = false;
                        }
                        3 => {
                            c.up = false;
                            current.down = false;
                        }
                        _ => panic!("fail"),
                    }
                }
                self.board.insert(p, current);
                current = c;
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
        self.current = (self.f)(self.current, self.board);
        self.board.board.get(&self.current)
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

type Wall = bool;

#[derive(Debug)]
struct Field {
    visited: bool,
    p: Point,
    left: Wall,
    right: Wall,
    up: Wall,
    down: Wall,
}
impl Field {
    fn new(p: Point) -> Self {
        Field {
            p,
            left: true,
            right: true,
            up: true,
            down: true,
            visited: false,
        }
    }
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.left, self.up, self.right, self.down) {
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

            (true, false, false, false) => write!(f, " "),
            (false, true, false, false) => write!(f, " "),
            (false, false, true, false) => write!(f, " "),
            (false, false, false, true) => write!(f, " "),
            (false, false, false, false) => write!(f, " "),
            _ => write!(f, " "),
        }
    }

}
