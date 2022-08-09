mod maze;
use itertools::{self, Itertools};
use std::fmt::Display;

use maze::Direction::*;
use maze::*;
use rand::Rng;
use termion::raw::IntoRawMode;
struct Cleanup {}
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

fn main() {
    let _ = Cleanup {};
    let raw_term = std::io::stdout().into_raw_mode().unwrap();
    let max = Point(4, 4);
    let mut board: Board<Field> = Board::new(max);
    for i in 0..max.0 {
        for j in 0..max.1 {
            board.insert(Point(i, j), Field::new(None));
        }
    }
    board.insert(Point(0, 0), Field::new(Some(1)));
    println!("{}", board);

    let term = console::Term::stdout();
    loop {
        print!("{}\r\n", board);
        let key = term.read_key().unwrap();
        let direction: Direction = key.into();
        let mut any_change = false;
        loop {
            let change = match direction {
                Right => iter_board(board.rows_mut()),
                Left => iter_board(board.rows_mut_rev()),
                Up => iter_board(board.columns_mut_rev()),
                Down => iter_board(board.columns_mut()),
                Nowhere => continue,
            };
            any_change = any_change || change;
            if !change {
                break;
            }
        }
        if !any_change {
            continue;
        }
        // spawn
        let mut rng = rand::thread_rng();
        if board.iter().all(|field| field.value.is_some()) {
            print!("You lost \r\n");
            break;
        }
        loop {
            let p = Point(rng.gen_range(0..4), rng.gen_range(0..4));
            if board.board.get(&p).unwrap().value.is_none() {
                board.insert(p, Field::new(Some(1)));
                break;
            }
        }
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

#[derive(Eq, PartialEq, Clone, Copy)]
struct Field {
    value: Option<usize>,
}
impl Field {
    fn new(value: Option<usize>) -> Self {
        Self { value }
    }
    fn combine(&mut self, other: &mut Field) -> bool {
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
