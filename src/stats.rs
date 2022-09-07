use std::{borrow::BorrowMut, ops::Add};

use gloo::timers::callback::Interval;

use crate::{model::UpgradeableBoard, *};

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Stats {
    pub points: usize,
    pub largest_harvest: usize,
    enable: bool,
    pub avg: f64,
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
    // let harvest = 1 << stats.largest_harvest;

    if !stats.enable {
        return html! {};
    }
    html! {
        <div>

       <h1> {"All Time Stats"} </h1>
       <p> {"Points: "} {stats.points} </p>
       <p> {"Largest Harvest: "} {stats.largest_harvest} </p>
       <Average/>

        </div>
    }
}
struct Timer {
    _interval: Interval,
}
impl Timer {}
impl Store for Timer {
    fn new() -> Self {
        let interval = || {
            Dispatch::<Timer>::new().reduce(|timer| timer);
        };
        Timer {
            _interval: Interval::new(100, interval),
        }
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

#[function_component(Average)]
fn average() -> Html {
    let (_, _) = use_store::<Timer>();
    let stats = Dispatch::<Stats>::new();

    let dispatch = Dispatch::<UpgradeableBoard>::new().get();

    let points = dispatch.points.get() as f64;

    let store = use_mut_ref(|| points);
    let last = *store.borrow();
    let avg = stats.get().avg;

    // rolling moving average calculation over 10s
    let new = (points - last).max(0.0);
    let avg = avg - (avg / 100.0);
    let avg = avg + (new / 100.0);

    stats.reduce_mut(|stats| stats.avg = avg);
    // info!("Points: {}, last: {}, avg: {}", points, last, avg);
    store.replace(points);

    html! {
        <p>
            {format!("Avg Points/s: {0:.2}", avg * 10.0)}
        </p>

    }
}
