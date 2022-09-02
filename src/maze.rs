#![allow(dead_code)]
use std::collections::HashSet;
use std::{collections::HashMap, fmt::Display};

use console::Key;
use rand::Rng;
fn main() {
    let max = Point::new(50, 20);
    let mut board: Board<Field> = Board::new(max);
    for i in 0..max.x {
        for j in 0..max.y {
            board.board.insert(Point::new(i, j), Field::new(Point::new(i, j)));
        }
    }

    board.aldous_broder();
    println!("{}", board);

    let mut rng = rand::thread_rng();
    let p = Point::new(rng.gen_range(0..10), rng.gen_range(0..10));
    board.visit(p, 0);
    println!("{}", board.longest_path);
    println!("{}", board);
}

impl Serialize for Point{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&format!("{},{}", self.x, self.y))
    }
}
use serde::de::{self, Visitor};
use yew::Properties;
struct PointVisitor;
use std::fmt;
impl<'de> Visitor<'de> for PointVisitor {
    type Value = (usize, usize);

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("it crashed!!!")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
           let (x,y) = v.split_once(',').unwrap();
                Ok((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))

    }
}
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let res = deserializer.deserialize_str(PointVisitor)?;
        Ok(Point{x: res.0, y: res.1})
    }

}
#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone, Properties)]
pub struct Point{pub x: usize, pub y: usize}

impl Point {
    pub fn random(max_x: usize, max_y: usize) -> Point {
        let mut rng = rand::thread_rng();
        Point::new(rng.gen_range(0..max_x), rng.gen_range(0..max_y))

    }
    pub fn new(x: usize, y: usize) -> Point {
        Point {x,y}

    }
    pub fn right(self) -> Self {
        Point::new(self.x + 1, self.y)
    }
    pub fn left(self) -> Self {
        Point::new(self.x.wrapping_sub(1), self.y)
    }
    pub fn down(self) -> Self {
        Point::new(self.x, self.y + 1)
    }
    pub fn up(self) -> Self {
        Point::new(self.x, self.y.wrapping_sub(1))
    }
    pub fn go(self, d: Direction) -> Self {
        match d {
            Left => self.left(),
            Right => self.right(),
            Up => self.up(),
            Down => self.down(),
            Nowhere => self,
        }
    }
}

use serde::{Deserialize, Serialize};
// use serde_with::serde_as;
#[derive(PartialEq, Eq, Deserialize, Serialize)]
pub struct Board<T> {
    pub board: HashMap<Point, T>,
    pub max: Point,
    longest_path: usize,
}
impl Board<Field> {
    pub fn visit(&mut self, p: Point, mut steps: usize) {
        let current = self.board.remove(&p);
        if let Some(mut current) = current {
            if current.steps == 0 || current.steps >= steps {
                current.steps = steps;
                self.longest_path = self.longest_path.max(steps);
                steps = steps + 1;
                if !current.wall.contains(&Down) {
                    self.visit(p.left(), steps);
                }
                if let Some(x) = self.board.get(&p.right()) {
                    if !x.wall.contains(&Down) {
                        self.visit(p.right(), steps);
                    }
                }
                if !current.wall.contains(&Right) {
                    self.visit(p.up(), steps);
                }
                if let Some(x) = self.board.get(&p.down()) {
                    if !x.wall.contains(&Right) {
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
        let mut p = Point::new(rng.gen_range(0..10), rng.gen_range(0..10));
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
                        Nowhere => panic!("Can't move nowhere"),
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
    pub fn insert(&mut self, point: Point, v: T) {
        self.board.insert(point, v);
    }
    pub fn points(&self) -> impl Iterator<Item=Point> + '_ {
        let mut p = Point::new(0,0);
        let f = std::iter::from_fn(move || { 
            let ret = p;
            if self.board.get(&ret).is_some() { p = p.right(); return Some(ret) }
            if self.board.get(&Point::new(0, p.y + 1)).is_some() {
                let ret = Point::new(0, p.y + 1);
                p = ret.right();
                Some(ret)
            } else {
                None
            }
        });
        f
        // unimplemented!()
        // BoardIter::new(self, Box::new(f), Point::new(0, 0))

    }
    pub fn iter(&self) -> BoardIter<'_, T> {
        let f = |p: Point, board: &Board<T>| {
            if board.get(&p.right()).is_some() {
                p.right()
            } else if board.get(&Point::new(0, p.y + 1)).is_some() {
                return Point::new(0, p.y + 1);
            } else {
                p.right()
            }
        };
        BoardIter::new(self, Box::new(f), Point::new(0, 0))
    }
    pub fn iter_mut(&mut self) -> BoardIterMut<'_, T> {
        let f = |p: Point, board: &Board<T>| {
            if board.get(&p.right()).is_some() {
                p.right()
            } else if board.get(&Point::new(0, p.y + 1)).is_some() {
                return Point::new(0, p.y + 1);
            } else {
                p.right()
            }
        };
        BoardIterMut::new(self, Box::new(f), Point::new(0, 0))
    }
    pub fn rows(&self) -> std::vec::IntoIter<BoardIter<'_, T>> {
        let mut rows = Vec::new();
        let mut y = 0;
        while let Some(_) = self.board.get(&Point::new(0, y)) {
            let iter = BoardIter::new(self, Box::new(|p, _| p.right()), Point::new(0, y));
            rows.push(iter);
            y += 1;
        }
        rows.into_iter()
    }
    pub fn rows_mut(&mut self) -> std::vec::IntoIter<BoardIterMut<'_, T>> {
        let mut rows = Vec::new();
        let mut y = 0;
        let me = self as *mut Self;

        while let Some(_) = self.board.get(&Point::new(0, y)) {
            let me = unsafe { &mut *me };
            let iter = BoardIterMut::new(me, Box::new(|p, _| p.right()), Point::new(0, y));
            rows.push(iter);
            y += 1;
        }
        rows.into_iter()
    }
    pub fn rows_mut_rev(&mut self) -> std::vec::IntoIter<BoardIterMut<'_, T>> {
        let mut rows = Vec::new();
        let mut y = 0;
        let me = self as *mut Self;
        while let Some(_) = self.board.get(&Point::new(0, y)) {
            let me = unsafe { &mut *me };
            let iter = BoardIterMut::new(me, Box::new(|p, _| p.left()), Point::new(self.max.x - 1, y));
            rows.push(iter);
            y += 1;
        }
        rows.into_iter()
    }
    pub fn columns(&self) -> std::vec::IntoIter<BoardIter<'_, T>> {
        let mut rows = Vec::new();
        let mut x = 0;

        while let Some(_) = self.board.get(&Point::new(x, 0)) {
            let iter = BoardIter::new(self, Box::new(|p, _| p.down()), Point::new(x, 0));
            rows.push(iter);
            x += 1;
        }
        rows.into_iter()
    }
    pub fn columns_mut(&mut self) -> std::vec::IntoIter<BoardIterMut<'_, T>> {
        let mut columns = Vec::new();
        let mut x = 0;
        let me = self as *mut Self;

        while let Some(_) = self.board.get(&Point::new(x, 0)) {
            let me = unsafe { &mut *me };
            let iter = BoardIterMut::new(me, Box::new(|p, _| p.down()), Point::new(x, 0));
            columns.push(iter);
            x += 1;
        }
        columns.into_iter()
    }
    pub fn columns_mut_rev(&mut self) -> std::vec::IntoIter<BoardIterMut<'_, T>> {
        let mut columns = Vec::new();
        let mut x = 0;
        let me = self as *mut Self;

        while let Some(_) = self.board.get(&Point::new(x, 0)) {
            let me = unsafe { &mut *me };
            let iter = BoardIterMut::new(me, Box::new(|p, _| p.up()), Point::new(x, self.max.y - 1));
            columns.push(iter);
            x += 1;
        }
        columns.into_iter()
    }

    pub fn new(max: Point) -> Self {
        Board {
            max,
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
            write!(f, "\r\n")
        })
    }
}

type PointFn<T> = dyn Fn(Point, &Board<T>) -> Point;
pub struct BoardIter<'a, T> {
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
pub struct BoardIterMut<'a, T: 'a> {
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
        let item: Option<&mut T> = self.board.board.get_mut(&self.current);
        if let Some(item) = item {
            let item = item as *mut T;
            let item = unsafe { &mut *item };
            self.current = (self.f)(self.current, &self.board);
            return Some(item);
        }
        None
    }
}

use Direction::*;
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
    Nowhere,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
            Nowhere => Nowhere,
        }
    }
}
use console::Key::*;
impl From<Key> for Direction {
    fn from(k: Key) -> Self {
        match k {
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
        write!(
            f,
            "\u{001b}[48;2;{};00;{}m",
            self.steps,
            (self.steps * 50).min(50)
        )?;
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
