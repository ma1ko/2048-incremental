use std::fmt::Display;

use crate::maze::Direction::*;
use crate::maze::*;
use rand::Rng;
use serde::{Serialize, Deserialize};
#[cfg(target_arch = "x86_64")]
use termion::raw::IntoRawMode;
struct Cleanup {}
#[cfg(target_arch = "x86_64")]
impl Drop for Cleanup {
    fn drop(&mut self) {
        let raw_term = std::io::stdout().into_raw_mode().unwrap();
        raw_term.suspend_raw_mode().unwrap();
        // println!("\u{001B}[?1049l");
    }
}

fn iter_board(iter: std::vec::IntoIter<BoardIterMut<Field>>) -> bool {
    iter.map(|mut row| {
        let first = row.next().unwrap();
        let (_, moved) = row.fold((first, false), |(acc, mov), field| {
            let moved = field.combine(acc);
            return (field, moved || mov);
        });
        moved
    })
    .fold(false, |state, x| x || state)
}

pub fn main() {
    let _ = Cleanup {};
    #[cfg(target_arch = "x86_64")]
    let _raw_term = std::io::stdout().into_raw_mode().unwrap();
    let max = Point(4, 4);
    let mut board: Board<Field> = Board::new(max);
    for i in 0..max.0 {
        for j in 0..max.1 {
            board.insert(Point(i, j), Field::new(None));
        }
    }
    board.insert(Point(0, 0), Field::new(Some(2)));
    println!("{}", board);

    let term = console::Term::stdout();
    loop {
        print!("{}\r\n", board);
        let key = term.read_key().unwrap();
        let direction: Direction = key.into();
        let won = board.play(direction);
        if won {
            break;
        }
    }
}
impl Board<Field> {
    pub fn random_empty_replace(&mut self, new_field: Field) {
        // Timeout in case we're full
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let point = Point(rng.gen_range(0..self.max.0), rng.gen_range(0..self.max.1));
            if let Some(field) = self.board.get(&point) {
                if field.value.is_none() { 
                    self.board.insert(point, new_field);
                    return;
                }
            }
        }
    }
    pub fn play_random(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let dir: Direction = rng.gen_range(0..4).into();
        self.play(dir)
    }
    pub fn play(&mut self, direction: Direction) -> bool {
        let mut any_change = false;
        loop {
            let change = match direction {
                Right => iter_board(self.rows_mut()),
                Left => iter_board(self.rows_mut_rev()),
                Up => iter_board(self.columns_mut_rev()),
                Down => iter_board(self.columns_mut()),
                Nowhere => return false,
            };
            any_change = any_change || change;
            if !change {
                break;
            }
        }
        if !any_change {
            return false;
        }
        // check if lost (doesn't work currently)
        if self.iter().all(|field| field.value.is_some()) {
            log::info!("You lost");
            return true;
        }
        // spawn random 2
        let mut rng = rand::thread_rng();
        loop {
            let max = self.max;
            let p = Point(rng.gen_range(0..max.0), rng.gen_range(0..max.1));
            if self.board.get(&p).unwrap().value.is_none() {
                self.board.insert(p, Field::new(Some(2)));
                break;
            }
        }
        return false;
    }
}
// impl Display for Board<Field> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.rows().try_for_each(|row| {
//             writeln!(f, "")?;
//             row.try_for_each(|field|{
//                 write!(f, "{}", field)

//             })

//         })
//     }

//}

#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub value: Option<usize>,
}
impl Field {
    pub fn new(value: Option<usize>) -> Self {
        Self { value }
    }
    pub fn combine(&mut self, other: &mut Field) -> bool {
        if let Some(value) = &mut self.value {
            if let Some(other_value) = &other.value {
                if value == other_value {
                    *value = *value + *other_value;
                    other.value = None;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            if other.value.is_some() {
                self.value = other.value.take();
                true
            } else {
                false
            }
        }
    }
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.value {
            write!(f, "{}", val)
        } else {
            write!(f, " ")
        }
    }
}
