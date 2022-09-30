use std::{fmt::Display, ops::Deref};

use crate::*;

#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Points {
    points: usize,
    cost_fn: CostFn,
}
impl Deref for Points {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.points
    }
}
impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points)
    }
}

impl Points {
    pub fn sub(&mut self, points: usize) {
        self.points -= points;
    }
    pub fn add(&mut self, points: usize) {
        let points = self.cost_fn.apply(points);
        let stats = Dispatch::<Stats>::new();
        stats.reduce_mut(|stats| stats.points(points));
        self.points += points;
    }
    pub fn set_log(&mut self) {
        self.cost_fn = CostFn::NlogN;
    }
    pub fn get(&self) -> usize {
        self.points
    }
}
impl Store for Points {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}


/*
#[derive(Default,Clone, Serialize, Deserialize, PartialEq)]
pub struct Bonus {
    points: Vec<Action>,
}
impl Bonus {
    fn points(&self, amount: usize) {



    }
    fn shuffles(&self, amount: usize) {}
}
impl Store for Bonus {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}
*/

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Copy)]
pub enum CostFn {
    Static,
    NlogN,
}

impl CostFn {
    pub fn apply(&self, points: usize) -> usize {
        let ret = match self {
            CostFn::Static => points,
            // CostFn::NlogN => points * (0usize.leading_zeros() - points.leading_zeros()) as usize,
            CostFn::NlogN => (points as f64 * (points as f64).log10()) as usize
        };
        // log::info!("{:?}: Points: {} to {}", self, points, ret);
        ret
    }
}
impl Default for CostFn {
    fn default() -> Self {
        CostFn::Static
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Shuffles {
    amount: f64,
    enabled: bool,
}
impl Shuffles {
    pub fn get(&self) -> f64 {
        self.amount
    }
    pub fn enable(&mut self) {
        self.enabled = true;
        self.amount = 1.0;
    }
    pub fn sub(&mut self, amount: f64) {
        self.amount -= amount;
    }
}

impl Store for Shuffles {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[function_component(ShowShuffles)]
pub fn show_shuffles() -> Html {
    let (shuffles, _) = use_store::<Shuffles>();

    if !shuffles.enabled {
        Default::default()
    } else {
        html! {<p> {"Remaining Shuffles:"} {shuffles.amount} </p>}
    }
}
