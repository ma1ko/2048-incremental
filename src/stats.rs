use std::{ops::Add, borrow::BorrowMut};

use crate::{*, model::UpgradeableBoard};

#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Clone)]
pub struct Stats {
    pub points: usize,
    pub largest_harvest: usize,
    enable: bool
}
impl Stats {
    pub fn points(&mut self, points: usize) {
        self.points += points;
    }
    pub fn harvest(&mut self, harvest: usize) {
        self.largest_harvest = self.largest_harvest.max(harvest);
    }
    pub fn enable(&mut self) {
        self.enable = true
    }
}
impl Store for Stats {
    fn new() -> Self {
        yewdux::storage::load(yewdux::storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

#[function_component(Statistics)]
pub fn statistics() -> html {
    // let mut test = use_state(|| Stats::new());
    // test.harvest(3);
    // test.set(5);
    // let test = test.borrow_mut();
    let (stats, _) = use_store::<Stats>();
    let harvest = 1 << stats.largest_harvest;

    if !stats.enable {
        return html!{};

    }
    html! {
        <div>

       <h1> {"All Time Stats"} </h1>
       <p> {"Points: "} {stats.points} </p>
       <p> {"Largest Harvest: "} {harvest} </p>

        </div>
    }
}
