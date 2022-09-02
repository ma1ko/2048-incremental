use crate::model::UpgradeableBoard;
use crate::*;
use gloo::timers::callback::Interval;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UpgradeCosts {
    CostsDouble,
    CostsOnetime,
    CostsStatic,
}
use UpgradeCosts::*;
#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UpgradeType {
    ExtendX,
    ExtendY,
    EnableAutomove,
    UpgradeAutomove,
    EnableAutoHarvest,
    UpgradeAutoHarvest,
    EnableRandomPlace,
    UpgradeRandomPlace,
    Reset,
    Harvest,
    ScientificNotation,
    EnableStatistics,
}
use UpgradeType::*;

macro_rules! run {
    ($a:ident, $b:ident) => {{
        let dispatch = Dispatch::<$a>::new();
        dispatch.reduce(|state| {
            state.$b();
            state
        });
    }};
}
impl UpgradeType {
    fn run(&self) {
        match self {
            ExtendX => run!(UpgradeableBoard, extend_x),
            ExtendY => run!(UpgradeableBoard, extend_y),
            EnableAutomove => run!(AutoActions, enable_automove),
            EnableRandomPlace => run!(AutoActions, enable_random_place),
            UpgradeAutomove => run!(AutoActions, upgrade_automove),
            UpgradeRandomPlace => run!(AutoActions, upgrade_random_place),
            EnableAutoHarvest => run!(AutoActions, enable_autoharvest),
            UpgradeAutoHarvest => run!(AutoActions, upgrade_autoharvest),
            Reset => reset(),
            Harvest => run!(UpgradeableBoard, harvest),
            ScientificNotation => run!(UpgradeableBoard, scientific_notation),
            EnableStatistics => enable_stats()
        }
    }
}

pub fn get_upgrades() -> Vec<Upgrade> {
    vec![
        Upgrade::new(20, 10, "Extend in X direction", ExtendX, CostsDouble),
        Upgrade::new(256, 100, "Extend in Y direction", ExtendY, CostsDouble),
        Upgrade::new(64, 32, "Enable Automove", EnableAutomove, CostsOnetime),
        Upgrade::new(512, 256, "Upgrade Automove", UpgradeAutomove, CostsDouble),
        Upgrade::new(0, 0, "Harvest", Harvest, CostsStatic),
        // Upgrade::new(256, 256, "Enable Autoharvesting", EnableAutoHarvest, CostsOnetime),
        // Upgrade::new(1024, 512, "Faster Autoharvesting", UpgradeAutoHarvest, CostsDouble),
        Upgrade::new(
            1000,
            750,
            "Scientific Notation",
            ScientificNotation,
            CostsOnetime,
        ),
        Upgrade::new(
            16,
            8,
            "Place a 4 regularly",
            EnableRandomPlace,
            CostsOnetime,
        ),
        Upgrade::new(
            64,
            32,
            "Upgrade Place a 4 regularly",
            UpgradeRandomPlace,
            CostsDouble,
        ),
        Upgrade::new(1024, 512, "Statistics", EnableStatistics, CostsStatic),
        Upgrade::new(0, 0, "HARD RESET", Reset, CostsStatic),
    ]
}

fn enable_stats() {
    Dispatch::<Stats>::new().reduce_mut(|stats| stats.enable());
}
fn reset() {
    log::info!("Reseting game!");
    Dispatch::<Points>::new().set(Default::default());
    Dispatch::<UpgradeableBoard>::new().set(Default::default());
    Dispatch::<AutoActions>::new().set(Default::default());
    Dispatch::<Upgrades>::new().set(Default::default());
    Dispatch::<Stats>::new().set(Default::default());
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Points {
    pub points: usize,
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
    pub fn sub(&self, points: usize) -> Points {
        Points {
            points: self.points - points,
        }
    }
    pub fn add(&self, points: usize) -> Points {
        let stats = Dispatch::<Stats>::new();
        stats.reduce_mut(|stats| stats.points(points));
        Points {
            points: self.points + points,
        }
    }
}
impl Store for Points {
    fn new() -> Self {
        // yewdux::prelude::init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
        // Self { points: 0 }
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upgrades {
    pub upgrades: Vec<Upgrade>,
}
impl Store for Upgrades {
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
    fn new() -> Self {
        log::info!("Loading");
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
}
impl Default for Upgrades {
    fn default() -> Self {
        Upgrades {
            upgrades: get_upgrades(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    pub visible: Cell<bool>,
    pub cost: Cell<usize>,
    pub show_at: Cell<usize>,
    pub text: String,
    pub action: UpgradeType,
    pub costs: UpgradeCosts,
}
// impl Eq for Upgrade {}
impl Upgrade {
    pub fn visible(&self, points: usize) -> bool {
        // show upgrade at threshold, don't hide it again
        if self.visible.get() {
            true
        } else if points >= self.show_at.get() {
            self.visible.set(true);
            true
        } else {
            false
        }
    }
    pub fn clickable(&self, points: usize) -> bool {
        points >= self.cost.get()
    }
    pub fn run(&self) {
        // reduce points
        let points = Dispatch::<Points>::new();
        points.reduce(|points| points.sub(self.cost.get()));
        // change costs for next update level
        match self.costs {
            CostsStatic => self.costs_static(),
            CostsDouble => self.costs_double(),
            CostsOnetime => self.costs_onetime(),
        }
        // check if it should be remain visible
        self.visible.set(false);
        self.visible(points.get().points);

        // run whatever the upgrade is supposed to do
        self.action.run();
    }

    fn new(
        cost: usize,
        show_at: usize,
        text: &'static str,
        action: UpgradeType,
        costs: UpgradeCosts,
    ) -> Self {
        Self {
            visible: Cell::new(false),
            cost: Cell::new(cost),
            show_at: Cell::new(show_at),
            text: text.to_string(),
            action,
            costs,
        }
    }
    fn costs_double(&self) {
        self.cost.set(self.cost.get() * 2);
        self.show_at.set(self.show_at.get() * 2);
    }
    fn costs_static(&self) {}
    fn costs_onetime(&self) {
        self.cost.set(usize::MAX);
        self.show_at.set(usize::MAX);
    }
}
fn do_automove() {
    let dispatch = Dispatch::<UpgradeableBoard>::new();
    dispatch.reduce(|board| {
        board.mv();
        board
    });
}
fn do_harvest() {
    let dispatch = Dispatch::<UpgradeableBoard>::new();
    dispatch.reduce(|board| {
        board.harvest();
        board
    });
}
fn do_random_place() {
    let dispatch = Dispatch::<UpgradeableBoard>::new();
    dispatch.reduce(|board| {
        board.random_place(2);
        board
    });
}
use yewdux::storage::Area::Local;
macro_rules! save {
    ($a:ident) => {
        let dispatch = Dispatch::<$a>::new();
        storage::save(dispatch.get().as_ref(), Local).unwrap();
    };
}

fn do_save() {
    save!(Upgrades);
    save!(Points);
    save!(UpgradeableBoard);
    save!(AutoActions);
    save!(Stats);
}
#[derive(Serialize, Deserialize)]
pub struct AutoActions {
    automove: RefCell<AutoAction>,
    autoharvest: RefCell<AutoAction>,
    random_place: RefCell<AutoAction>,
    autosave: RefCell<AutoAction>,
}
impl AutoActions {
    fn upgrade_automove(&self) {
        self.automove.borrow_mut().upgrade(50);
    }
    fn enable_automove(&self) {
        self.automove.borrow_mut().enable();
    }
    fn upgrade_random_place(&self) {
        self.random_place.borrow_mut().upgrade(50);
    }
    fn enable_random_place(&self) {
        self.random_place.borrow_mut().enable();
    }
    fn upgrade_autoharvest(&self) {
        self.random_place.borrow_mut().upgrade(50);
    }
    fn enable_autoharvest(&self) {
        self.autoharvest.borrow_mut().enable();
    }
}

impl Default for AutoActions {
    fn default() -> Self {
        let a = AutoActions {
            automove: RefCell::new(AutoAction::new(None, Action::AutoMove, 1000, false)),
            autoharvest: RefCell::new(AutoAction::new(None, Action::AutoHarvest, 10000, false)),
            random_place: RefCell::new(AutoAction::new(None, Action::RandomPlace, 1000, false)),
            autosave: RefCell::new(AutoAction::new(None, Action::AutoSave, 5000, true)),
        };
        a.autosave.borrow_mut().set_callback();
        a
    }
}

impl Store for AutoActions {
    fn should_notify(&self, _old: &Self) -> bool {
        false // ?
    }
    fn new() -> Self {
        let me: Self = storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default();
        //activate timers
        me.automove.borrow_mut().set_callback();
        me.autosave.borrow_mut().set_callback();
        me.random_place.borrow_mut().set_callback();
        me.autoharvest.borrow_mut().set_callback();
        me
    }
}
#[derive(Serialize, Deserialize)]
enum Action {
    AutoMove,
    RandomPlace,
    AutoSave,
    AutoHarvest,
}
#[derive(Serialize, Deserialize)]
struct AutoAction {
    #[serde(skip)]
    interval: Option<Interval>,
    action: Action,
    time: u32,
    active: bool,
}

impl AutoAction {
    fn new(interval: Option<Interval>, f: Action, time: u32, active: bool) -> Self {
        Self {
            interval,
            action: f,
            time,
            active,
        }
    }
    fn upgrade(&mut self, timediff: u32) {
        // assert!(self.active);
        self.time -= timediff;
        self.set_callback();
    }
    fn enable(&mut self) {
        self.active = true;
        self.set_callback();
    }
    fn set_callback(&mut self) {
        if !self.active {
            return;
        }
        let f: &'static dyn Fn() = match self.action {
            Action::AutoMove => &do_automove,
            Action::AutoSave => &do_save,
            Action::RandomPlace => &do_random_place,
            Action::AutoHarvest => &do_harvest,
        };

        let dispatch = Dispatch::<UpgradeableBoard>::new();
        let cb = dispatch.reduce_callback(|action| {
            (f)();
            action
        });
        self.interval = Some(Interval::new(self.time, move || cb.emit(())));
    }
}
