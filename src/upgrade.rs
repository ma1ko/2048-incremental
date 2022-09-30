use crate::model::UpgradeableBoard;
use crate::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::marker::PhantomData;

macro_rules! run {
    ($a:ident, $b:ident) => {{
        // log::info!("Running ",);
        let dispatch = Dispatch::<$a>::new();
        dispatch.reduce(|state| {
            state.$b();
            state
        })
    }};

    ($a:ident, $b:ident, $c:expr) => {{
        let dispatch = Dispatch::<$a>::new();
        dispatch.reduce(|state| {
            state.$b($c);
            state
        })
    }};
}

macro_rules! _get {
    ($a:ident, $b:ident) => {{
        let dispatch = Dispatch::<$a>::new().get();
        dispatch.$b()
    }};
    ($a:ident, $b:ident, $c:expr) => {{
        let dispatch = Dispatch::<$a>::new().get();
        dispatch.$b($c)
    }};
}

#[derive(Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum UpgradeCosts {
    CostsMultiply,
    CostsOnetime,
    CostsStatic,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum UpgradeType {
    SliderPoint,
    AutoShuffle,
    Shuffle,
    Place,
    ExtendX,
    ExtendY,
    AutoMove,
    AutoSave,
    AutoHarvest,
    RandomPlace,
    Reset,
    Harvest,
    ScientificNotation,
    EnableStatistics,
    NlogNCost,
    BonusTile,
    Cheat,
    Test,
}
pub use UpgradeType::*;
impl UpgradeType {
    pub fn run(&self, level: usize) {
        match self {
            EnableStatistics => Dispatch::<Stats>::new().reduce_mut(|s| s.enable()),
            // Place => run!(UpgradeableBoard, random_place, 2 << (level - 1)),
            Place => Dispatch::<Sliders>::new().get().run(&RandomPlace),
            Harvest => run!(UpgradeableBoard, harvest),
            RandomPlace => run!(UpgradeableBoard, random_place, 2 << (level)),
            ScientificNotation => run!(UpgradeableBoard, scientific_notation),
            BonusTile => run!(UpgradeableBoard, set_combine_fn, CombineFn::Bonus(64, 16)),
            Reset => reset(),
            AutoSave => {
                Timeout::new(1, move || {
                    do_save();
                }).forget();
            }
            AutoMove => run!(UpgradeableBoard, mv),
            AutoShuffle => run!(UpgradeableBoard, shuffle),
            SliderPoint => Dispatch::<SliderPoints>::new().reduce_mut(|p| p.add(1)),
            NlogNCost => Dispatch::<Points>::new().reduce_mut(Points::set_log),
            _ => {}
        }
    }
}

/*
fn board(f: &dyn Fn(&UpgradeableBoard)) {
    run::<UpgradeableBoard>(f)
}

fn run<S: Store>(f: &dyn Fn(&S)) {
    let x = Dispatch::<S>::new();
    x.reduce(|dispatch| {
        f(dispatch.as_ref());
        dispatch
    });
}
*/

fn enable_stats() {
    Dispatch::<Stats>::new().reduce_mut(|stats| stats.enable());
}
use gloo::timers::callback::Timeout;
use gloo_storage::Storage;
pub fn reset() {
    if !gloo::dialogs::confirm("Perform hard reset?") {
        return;
    }
    log::info!("Reseting game!");
    gloo::storage::LocalStorage::clear();
    // do_save();
    web_sys::window().unwrap().location().reload().unwrap();
}
#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum CombineFn {
    Standard,
    Bonus(usize, usize),
}

impl From<CombineFn> for Box<dyn Fn(usize, usize) -> (usize, Option<usize>)> {
    fn from(f: CombineFn) -> Self {
        return Box::new(move |target, source| match f {
            CombineFn::Standard => (target + 1, None),
            CombineFn::Bonus(level, amount) => {
                assert!(target == source);
                if target + 1 == level {
                    (target + 1, Some(amount))
                } else {
                    (target + 1, None)
                }
            }
        });
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Upgrades {
    upgrades: HashMap<UpgradeType, Mrc<Upgrade>>,
}

impl Upgrades {
    fn get(&self, t: &UpgradeType) -> &Mrc<Upgrade> {
        self.upgrades
            .get(&t)
            .expect(&format!("Upgrade {:?} not found", t))
    }
    pub fn statics(&self) -> impl Iterator<Item = &Mrc<Upgrade>> {
        self.upgrades
            .values()
            .filter(|u| u.borrow().status == UpgradeStatus::Static)
    }
    pub fn onetime(&self) -> impl Iterator<Item = &Mrc<Upgrade>> {
        self.upgrades
            .values()
            .filter(|u| u.borrow().status != UpgradeStatus::Static)
            .filter(|u| u.borrow().status != UpgradeStatus::Special)
    }
    pub fn is_done(&self, t: UpgradeType) -> bool {
        self.upgrades[&t].borrow().done
    }
    pub fn run(&self, t: UpgradeType) {
        let level = self.upgrades[&t].borrow().level;
        t.run(level);
    }
}
impl Index<&UpgradeType> for Upgrades {
    type Output = Mrc<Upgrade>;
    fn index(&self, index: &UpgradeType) -> &Self::Output {
        self.get(index)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum UpgradeStatus {
    OneTime,
    Static,
    Multiply(usize),
    Special,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Clone)]
pub struct Upgrade {
    max_level: usize,
    pub level: usize,
    pub visible: Cell<bool>,
    pub done: bool,
    pub cost: Condition,
    pub show: Condition,
    pub t: UpgradeType,
    pub status: UpgradeStatus,
    pub text: String,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            upgrades: [
                // Static stuff
                Upgrade::new(Place, Free(), Free(), "Place")
                    .static_()
                    .build(),
                Upgrade::new(Harvest, Free(), Free(), "Harvest")
                    .static_()
                    .build(),
                Upgrade::new(Shuffle, Free(), UpgradeDone(AutoShuffle), "Shuffle")
                    .static_()
                    .build(),
                Upgrade::new(Reset, Free(), Free(), "RESET")
                    .special()
                    .build(),
                // Onetime upgrades
                Upgrade::new(
                    ExtendX,
                    PointsOnBoard(64),
                    PointsOnBoard(32),
                    "extend board horizontally",
                )
                .multiply(8)
                .build(),
                Upgrade::new(ExtendY, 64, 32, "extend board vertically")
                    .multiply(8)
                    .build(),
                Upgrade::new(AutoMove, 32, Free(), "Automove")
                    .multiply(2)
                    .build(),
                Upgrade::new(
                    RandomPlace,
                    Harvested(32),
                    Harvested(16),
                    "Place random number",
                )
                .multiply(4)
                .build(),
                Upgrade::new(
                    ScientificNotation,
                    Harvested(1024),
                    Harvested(256),
                    "ScientificNotation",
                )
                .build(),
                Upgrade::new(AutoSave, AvgPoints(2), AvgPoints(1), "Save the Game").build(),
                Upgrade::new(
                    AutoShuffle,
                    AvgPoints(20),
                    AvgPoints(10),
                    "Enable Shuffling",
                )
                .build(),
                Upgrade::new(EnableStatistics, 128, 32, "Show Statistics").build(),
                Upgrade::new(BonusTile, Harvested(16), Harvested(16), "Bonus Tile").build(),
                Upgrade::new(
                    NlogNCost,
                    Harvested(4096),
                    Harvested(1024),
                    "Bonus Points when Harvesting",
                )
                .build(),
                Upgrade::new(
                    SliderPoint,
                    NumberOnBoard(4),
                    NumberOnBoard(2),
                    "Get a point",
                )
                .multiply(2)
                .build(),
            ]
            .into(),
        }
    }
}
impl Store for Upgrades {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

impl Display for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}

impl Upgrade {
    pub fn build(self) -> (UpgradeType, Mrc<Self>) {
        (self.t, Mrc::new(self))
    }
    pub fn visible(&self) -> bool {
        // don't hide an upgrade after it has been shown once
        if self.visible.get() {
            return true;
        }
        if self.done {
            false
        } else {
            let vis = self.show.check();
            if vis {
                self.visible.set(true)
            }
            vis
        }
    }
    pub fn clickable(&self) -> bool {
        self.cost.check() && self.visible()
    }
    pub fn upgrade(&mut self) {
        // reduce points
        self.level += 1;
        self.cost.fulfilled();
        match self.status {
            UpgradeStatus::Static => {}
            UpgradeStatus::OneTime => {
                self.done = true;
                // Dispatch::<Upgrades>::new().reduce(|x| x);
            }
            UpgradeStatus::Multiply(i) => {
                self.cost = self.cost.multiply(i);
                self.show = self.show.multiply(i);
                self.visible.set(false);
            }
            UpgradeStatus::Special => {}
        }
        if self.level == self.max_level {
            self.done = true;
        }
        // self.run();

        // Dispatch::<Upgrades>::new().reduce(|upgrades| upgrades);
    }
    pub fn run(&self) {
        // self.upgrade();
        self.t.run(self.level);
    }

    pub fn new<C, S, D>(t: UpgradeType, cost: C, show: S, text: D) -> Self
    where
        C: Into<Condition>,
        S: Into<Condition>,
        D: Into<String>,
    {
        Self {
            visible: Cell::new(false),
            done: false,
            cost: cost.into(),
            show: show.into(),
            // action,
            status: UpgradeStatus::OneTime,
            level: 0,
            max_level: 999,
            text: text.into(),
            t,
        }
    }
    pub fn static_(mut self) -> Self {
        self.status = UpgradeStatus::Static;
        self
    }
    pub fn special(mut self) -> Self {
        self.status = UpgradeStatus::Special;
        self
    }
    pub fn multiply(mut self, i: usize) -> Self {
        self.status = UpgradeStatus::Multiply(i);
        self
    }
    pub fn max_level(mut self, i: usize) -> Self {
        self.max_level = i;
        self
    }
}
