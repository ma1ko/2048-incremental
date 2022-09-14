
use crate::maze::Direction::*;
use crate::maze::*;
use crate::*;
use rand::Rng;
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
pub type CombineFn = dyn Fn(usize, usize) -> (usize, Option<usize>);

fn iter_board(iter: std::vec::IntoIter<BoardIterMut<Number>>, f: Box<CombineFn>) -> bool {
    iter.map(|row| iter_row(row, f.as_ref()))
        .fold(false, |state, x| x || state)
}

fn iter_row(iter: BoardIterMut<Number>, f: &CombineFn) -> bool {
    // etremely ugly piece of code, I haven't found a better solution :(
    let mut fields = iter.collect::<Vec<&mut Number>>();
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
            let (v1, v2) = f(fields[current].value.unwrap(), fields[i].value.unwrap());
            fields[current].value = Some(v1);
            fields[i].value = v2;
            // let value = fields[i].value.take().unwrap();
            // fields[current].value = Some(value + 1);
            change = true;
            if fields[i].value.is_some() {
                // we combined but have to continue due to bonus fields
                current += 1;
                i -= 1;
            }
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
    let mut board: Board<Number> = Board::new(max);
    for i in 0..max.x {
        for j in 0..max.y {
            board.insert(Point::new(i, j), 0.into())
        }
    }
    board.insert(Point::new(0, 0), 2.into());
    println!("{}", board);

    let term = console::Term::stdout();
    loop {
        print!("{}\r\n", board);
        let key = term.read_key().unwrap();
        let direction: Direction = key.into();
        let _won = board.play(direction, Box::new(|a, _| (a + 1, None)));
        // if won {
        //     break;
        // }
    }
}
impl Board<Number> {
    pub fn random_empty_replace(&mut self, new_field: Number) -> usize {
        // Timeout in case we're full
        for _ in 0..10 {
            let point = Point::random(self.max.x, self.max.y);
            if let Some(field) = self.board.get(&point) {
                if field.is_none() {
                    self.board.insert(point, new_field);
                    return new_field.value();
                }
            }
        }
        return 0;
    }
    pub fn play_random(&mut self, f: Box<CombineFn>) -> usize {
        let mut rng = rand::thread_rng();
        let dir: Direction = rng.gen_range(0..4).into();
        self.play(dir, f)
    }
    pub fn play(&mut self, direction: Direction, f: Box<CombineFn>) -> usize {
        let change = match direction {
            Left => iter_board(self.rows_mut(), f),
            Right => iter_board(self.rows_mut_rev(), f),
            Down => iter_board(self.columns_mut_rev(), f),
            Up => iter_board(self.columns_mut(), f),
            Nowhere => return 0,
        };

        if !change {
            return 0;
        }
        if self.iter().all(|field| field.is_some()) {
            // log::info!("You lost");
            return 0;
        }
        // spawn random 2
        loop {
            let p = Point::random(self.max.x, self.max.y);
            if self.board.get(&p).unwrap().is_none() {
                self.board.insert(p, 2.into());
                return 2;
            }
        }
    }
}

