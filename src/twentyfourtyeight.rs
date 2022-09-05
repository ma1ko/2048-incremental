use std::fmt::Display;

use crate::maze::Direction::*;
use crate::maze::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
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
        // let first = row.next().unwrap();
        // let (_, moved) = row.fold((first, false), |(acc, mov), field| {
        // let moved = field.combine(acc);
        // return (field, moved || mov);
        // });
        // moved
        iter_row(row)
    })
    .fold(false, |state, x| x || state)
}
// // compacts towards the back
// fn iter_compact(mut iter: BoardIterMut<Field>) -> bool {
//     let mut change = false;
//     let mut current = iter.next().unwrap();
//     while let Some(mut field) = iter.next() {
//         if field.value.is_none() {
//             field.value = current.take();

//         }

//     }

// }
fn iter_row(iter: BoardIterMut<Field>) -> bool {
    // etremely ugly piece of code, I haven't found a better solution :(
    let mut fields = iter.collect::<Vec<&mut Field>>();
    let mut change = false;
    let mut current = 0;
    let mut i = 0;
    while i < fields.len() - 1 {
        i += 1;
        if i == current {
            continue;
        }
        if fields[i].value.is_none() {
            continue;
        }
        if fields[current].value.is_none() {
            // move
            let value = fields[i].value.take();
            fields[current].value = value;
            change = true;
            continue;
        }
        // both field have value, try combining
        if fields[i].value == fields[current].value {
            let value = fields[i].value.take().unwrap();
            fields[current].value = Some(value + 1);
            change = true;
        } else {
            // couldn't combine, move on to next field
            current += 1;
            i -= 1;
        }
    }
    change
}

pub fn _main() {
    let _ = Cleanup {};
    #[cfg(target_arch = "x86_64")]
    let _raw_term = std::io::stdout().into_raw_mode().unwrap();
    let max = Point::new(4, 4);
    let mut board: Board<Field> = Board::new(max);
    for i in 0..max.x {
        for j in 0..max.y {
            board.insert(Point::new(i, j), Field::new(None));
        }
    }
    board.insert(Point::new(0, 0), Field::new(Some(2)));
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
            let point = Point::random(self.max.x, self.max.y);
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
        // let mut any_change = false;
        // loop {
        let change = match direction {
            Left => iter_board(self.rows_mut()),
            Right => iter_board(self.rows_mut_rev()),
            Down => iter_board(self.columns_mut_rev()),
            Up => iter_board(self.columns_mut()),
            Nowhere => return false,
        };
        // any_change = any_change || change;
        // if !change {
        // break;
        // }
        // }
        if !change {
            return false;
        }
        if self.iter().all(|field| field.value.is_some()) {
            // log::info!("You lost");
            return true;
        }
        // spawn random 2
        loop {
            let p = Point::random(self.max.x, self.max.y);
            if self.board.get(&p).unwrap().value.is_none() {
                self.board.insert(p, Field::new(Some(1)));
                break;
            }
        }
        return false;
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
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
                    *value = *value + 1;
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
