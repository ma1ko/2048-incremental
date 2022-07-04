fn main() {
    let mut s = Sudoku::default();

    // s.rows[0].set([0, 4, 0, 6, 0, 8, 0, 0, 0]);
    // s.rows[1].set([5, 6, 0, 9, 0, 0, 0, 2, 0]);
    // s.rows[2].set([1, 9, 7, 2, 4, 0, 3, 0, 0]);

    // s.rows[3].set([0, 8, 0, 0, 9, 7, 0, 0, 1]);
    // s.rows[4].set([0, 3, 0, 1, 0, 6, 0, 0, 5]);
    // s.rows[5].set([0, 0, 9, 5, 0, 3, 4, 6, 0]);

    // s.rows[6].set([0, 0, 0, 3, 5, 0, 1, 0, 8]);
    // s.rows[7].set([0, 0, 0, 0, 6, 0, 0, 4, 3]);
    // s.rows[8].set([0, 7, 3, 0, 0, 9, 6, 0, 2]);
    //
    //

    // s.rows[0].set([0, 0, 0, 0, 1, 5, 6, 0, 0]);
    // s.rows[1].set([0, 0, 0, 0, 6, 0, 8, 5, 0]);
    // s.rows[2].set([0, 0, 0, 0, 0, 0, 0, 0, 3]);

    // s.rows[3].set([2, 0, 8, 0, 0, 0, 0, 0, 4]);
    // s.rows[4].set([0, 4, 0, 3, 9, 0, 0, 0, 2]);
    // s.rows[5].set([6, 0, 1, 2, 0, 0, 0, 0, 0]);

    // s.rows[6].set([4, 8, 0, 6, 7, 0, 0, 0, 0]);
    // s.rows[7].set([0, 0, 0, 8, 0, 0, 0, 9, 0]);
    // s.rows[8].set([0, 0, 6, 0, 4, 0, 0, 0, 0]);
    //
    //
    // s.rows[0].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[1].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[2].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // s.rows[3].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[4].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[5].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // s.rows[6].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[7].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // s.rows[8].set([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // s.rows[0].set([0, 7, 0, 0, 0, 0, 0, 0, 8]);
    // s.rows[1].set([0, 0, 5, 0, 0, 4, 9, 7, 0]);
    // s.rows[2].set([0, 0, 0, 9, 0, 0, 0, 0, 6]);

    // s.rows[3].set([0, 0, 0, 0, 0, 0, 0, 8, 0]);
    // s.rows[4].set([0, 0, 6, 0, 2, 0, 0, 0, 0]);
    // s.rows[5].set([0, 2, 0, 1, 0, 0, 7, 4, 0]);

    // s.rows[6].set([3, 0, 0, 0, 0, 5, 0, 0, 0]);
    // s.rows[7].set([0, 6, 0, 4, 0, 0, 1, 2, 0]);
    // s.rows[8].set([0, 0, 0, 0, 0, 0, 0, 0, 9]);

    s.rows[0].set([0, 9, 3, 0, 7, 0, 0, 6, 0]);
    s.rows[1].set([0, 8, 0, 0, 0, 0, 0, 0, 0]);
    s.rows[2].set([0, 0, 0, 6, 0, 0, 0, 0, 1]);

    s.rows[3].set([8, 0, 0, 0, 0, 0, 0, 3, 0]);
    s.rows[4].set([0, 3, 4, 0, 0, 9, 0, 0, 5]);
    s.rows[5].set([1, 0, 0, 0, 4, 0, 0, 0, 0]);

    s.rows[6].set([0, 0, 0, 0, 0, 5, 2, 0, 0]);
    s.rows[7].set([0, 6, 7, 0, 9, 0, 0, 1, 0]);
    s.rows[8].set([4, 0, 0, 0, 0, 0, 0, 0, 0]);

    println!("{}", s);
    println!("");
    let mut x = true;
    while x {
        println!("Check!");
        while x { x = s.check() }
        println!("Only!");
        x |= s.only();
    }

    println!("{}", s);
}

use std::rc::Rc;

use std::convert::identity as TRUE;

type Board = [Line; 9];
struct Sudoku {
    rows: Board,
    cols: Board,
    blocks: Board,
}

impl Sudoku {
    fn init_block(&mut self, x: usize, y: usize) {
        for i in 0..3 {
            for j in 0..3 {
                let row = &self.rows[x + i][y + j];
                // println!( "Setting block {} index {}, with {},{}", x + y / 3, i * 3 + j, row.x, row.y);
                self.blocks[x + y / 3][i * 3 + j] = row.clone();
            }
        }
    }
    fn check(&mut self) -> bool {
        let mut change = self.rows.iter_mut().map(|x| x.check()).any(TRUE);
        change |= self.cols.iter_mut().map(|x| x.check()).any(TRUE);
        change |= self.blocks.iter_mut().map(|x| x.check()).any(TRUE);
        change
    }
    fn only(&mut self) -> bool {
        let mut change = self.rows.iter_mut().map(|x| x.only()).any(TRUE);
        if change {
            self.check();
        }
        change |= self.cols.iter_mut().map(|x| x.only()).any(TRUE);
        if change {
            self.check();
        }
        change |= self.blocks.iter_mut().map(|x| x.only()).any(TRUE);
        if change {
            self.check();
        }
        change
    }
}
impl Default for Sudoku {
    fn default() -> Self {
        let mut cells: Vec<Rc<Field>> = Vec::new();
        for x in 0..9 {
            for y in 0..9 {
                cells.push(Rc::new(Field::new(x, y)));
            }
        }
        let mut rows: Board = Default::default();
        let mut cols: Board = Default::default();
        let blocks: Board = Default::default();

        for x in 0..9 {
            for y in 0..9 {
                rows[x][y] = cells[x * 9 + y].clone();
                cols[y][x] = cells[x * 9 + y].clone();
            }
        }
        let mut s = Sudoku { rows, cols, blocks };
        for i in 0..3 {
            for j in 0..3 {
                s.init_block(i * 3, j * 3);
            }
        }
        s
    }
}

#[derive(Default)]
struct Line {
    fields: [Rc<Field>; 9],
}
impl Line {
    fn check(&mut self) -> Changed {
        let vals = self.fields.iter().map(|x| x.get()).flatten(); // all set fields
        vals.map(|val| {
            self.fields
                .iter()
                .filter(|field| !field.has_value())
                .map(|field| field.remove_candidate(val))
                .any(TRUE)
        })
        .any(TRUE)
    }
    fn only(&mut self) -> Changed {
        let mut counts: [(usize, Rc<Field>); 10] = Default::default();
        let fields = self.fields.iter().filter(|x| !x.has_value());
        fields.for_each(|field| {
            let f = field.cand.borrow();
            f.iter().for_each(|cand| {
                let count = counts[*cand].0;
                counts[*cand] = (count + 1, field.clone());
            });
        });
        assert!(counts[0].0 == 0); // no 0 in sudoku
        counts
            .iter()
            .enumerate()
            .skip(1)
            .map(|(number, count)| {
                let (count, field) = (count.0, &count.1);
                if count == 1 {
                    println!("Only: Setting ({}, {}) to {}", field.x, field.y, number);
                    field.set(number);
                    return true;
                }
                return false;
            })
            .any(TRUE)
    }
    fn set(&mut self, values: [usize; 9]) {
        for i in 0..9 {
            if values[i] > 0 {
                self.fields[i].set(values[i])
            }
        }
    }
}

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeSet as Set;
struct Field {
    val: Cell<Option<usize>>,
    cand: RefCell<Set<usize>>,
    x: usize,
    y: usize,
}

type Changed = bool;

impl Default for Field {
    fn default() -> Self {
        // shouldn't be used, just simpler for init
        Field::new(999, 999)
    }
}
impl Field {
    fn new(x: usize, y: usize) -> Self {
        Field {
            val: Cell::new(None),
            cand: RefCell::new(Set::from([1, 2, 3, 4, 5, 6, 7, 8, 9])),
            x,
            y,
        }
    }
    fn get(&self) -> Option<usize> {
        self.val.clone().into_inner()
    }
    fn set(&self, val: usize) {
        self.val.replace(Some(val));
    }
    fn has_value(&self) -> bool {
        self.val.get().is_some()
    }
    fn _is_candidate(&self, val: usize) -> bool {
        assert!(!self.has_value());
        let cand = self.cand.borrow();
        cand.contains(&val)
    }
    fn _add_candidate(&self, val: usize) {
        assert!(!self.has_value());
        let mut cand = self.cand.borrow_mut();
        cand.insert(val);
    }
    fn remove_candidate(&self, val: usize) -> Changed {
        assert!(!self.has_value());
        let mut cand = self.cand.borrow_mut();
        cand.remove(&val);
        if cand.len() == 1 {
            // set since only value
            // ugly...
            let value = *cand.iter().take(1).collect::<Vec<&usize>>()[0];
            self.set(value);
            println!("Set ({},{}) to {}", self.x, self.y, value);
            return true;
        }
        if cand.len() == 0 {
            println!("This is insolvable!!!");
        }

        false
    }
}

// Rather useless definitions
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.val.get().clone() {
            write!(f, "{}", val)?
        } else {
            write!(f, " ")?
        }
        write!(f, "|")
    }
}

use std::fmt::Display;
impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self
            .rows
            .iter()
            .map(|row| {
                writeln!(f, "-------------------")?;
                write!(f, "|")?;
                let _ = row
                    .iter()
                    .map(|cell| write!(f, "{}", cell))
                    .collect::<Vec<_>>();
                writeln!(f)
            })
            .collect::<Vec<_>>();
        writeln!(f, "-------------------")
    }
}

use std::ops::Index;
use std::ops::IndexMut;
impl Index<usize> for Line {
    type Output = Rc<Field>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}
impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}
use std::ops::Deref;
impl Deref for Line {
    type Target = [Rc<Field>; 9];
    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}
