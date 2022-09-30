use gloo::timers::callback::Interval;

use crate::*;
#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Stats {
    pub points: usize,
    pub largest_harvest: usize,
    enable: bool,
    // pub avg: f64,
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
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[function_component(Statistics)]
pub fn statistics() -> Html {
    let (stats, _) = use_store::<Stats>();
    let _ = use_store::<Timer>();
    let points_on_board = Dispatch::<UpgradeableBoard>::new().get().points.get();

    if !stats.enable {
        return html! {
            // calculate but don't show yet
            <CalcAverage/>
        };
    }
    html! {
        <div>

       <h1> {"All Time Stats"} </h1>
       <p> {"Points: "} {stats.points} </p>
       <p> {"Largest Harvest: "} {stats.largest_harvest} </p>
       <p> {"Points on the board: "} {points_on_board} </p>
       <CalcAverage/>
       <ShowAverage/>
       <ShowShuffles/>

        </div>
    }
}
pub struct Timer {
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

#[derive(Copy, Clone, PartialEq)]
pub struct Avg {
    last: f64,
    avg: f64,
}
impl Avg {
    fn avg(&mut self) -> f64 {
        let dispatch = Dispatch::<UpgradeableBoard>::new().get();
        let points = dispatch.points.get() as f64;
        let last = self.last;
        let avg = self.avg;

        let new = (points - last).max(0.0);
        let avg = avg - (avg / 100.0);
        let avg = avg + (new / 100.0);
        self.avg = avg;
        self.last = points;
        avg
    }
    pub fn get_avg(&self) -> f64 {
        self.avg * 10.0
    }
    pub fn harvested(&mut self, value: usize) {
        self.last -= value as f64
    }
    pub fn manually_added(&mut self, value: usize) {
        self.last += value as f64
    }
}
impl Store for Avg {
    fn new() -> Self {
        let dispatch = Dispatch::<UpgradeableBoard>::new().get();
        let points = dispatch.points.get() as f64;
        Avg {
            last: points,
            avg: 0.0,
        }
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[function_component(CalcAverage)]
fn average() -> Html {
    let (_, _) = use_store::<Timer>();
    let dispatch = Dispatch::<Avg>::new();
    dispatch.reduce_mut(|avg| avg.avg());
    html! {}
}

#[function_component(ShowAverage)]
fn average() -> Html {
    let (_, _) = use_store::<Timer>();
    // let stats = Dispatch::<Stats>::new();

    let dispatch = Dispatch::<Avg>::new();
    let avg = dispatch.get().get_avg();
    html! {
        <p>
            {format!("Avg Points/s: {0:.2}", avg)}
        </p>

    }
}
