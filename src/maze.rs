#![allow(dead_code)]
use std::{collections::HashMap, fmt::Display};

use rand::Rng;

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
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
        E: de::Error,
    {
        let (x, y) = v.split_once(',').unwrap();
        Ok((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
    }
}
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let res = deserializer.deserialize_str(PointVisitor)?;
        Ok(Point { x: res.0, y: res.1 })
    }
}
#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone, Properties)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn random(max_x: usize, max_y: usize) -> Point {
        let mut rng = rand::thread_rng();
        Point::new(rng.gen_range(0..max_x), rng.gen_range(0..max_y))
    }
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
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
}

impl<T: 'static> Board<T> {
    pub fn insert(&mut self, point: Point, v: T) {
        self.board.insert(point, v);
    }
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let mut p = Point::new(0, 0);
        let f = std::iter::from_fn(move || {
            let ret = p;
            if self.board.get(&ret).is_some() {
                p = p.right();
                return Some(ret);
            }
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
            let iter =
                BoardIterMut::new(me, Box::new(|p, _| p.left()), Point::new(self.max.x - 1, y));
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
            let iter =
                BoardIterMut::new(me, Box::new(|p, _| p.up()), Point::new(x, self.max.y - 1));
            columns.push(iter);
            x += 1;
        }
        columns.into_iter()
    }

    pub fn new(max: Point) -> Self {
        Board {
            max,
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
