use crate::model::UpgradeableBoard;
use crate::*;
use gloo::timers::callback::Interval;
use std::fmt::Display;
use std::ops::Deref;

macro_rules! run {
    ($a:ident, $b:ident) => {{
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
macro_rules! attr {
    ($a:ident, $b:ident) => {{
        let dispatch = Dispatch::<$a>::new().get();
        dispatch.$b
    }};
}
macro_rules! get {
    ($a:ident, $b:ident) => {{
        let dispatch = Dispatch::<$a>::new().get();
        dispatch.$b()
    }};
}

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
impl UpgradeType {
    pub fn text(&self) -> &'static str {
        use UpgradeType::*;
        match self {
            ExtendX => "Extend Board in X direction",
            ExtendY => "Extend Board in Y direction",
            EnableAutomove => "Automatically move the board",
            EnableRandomPlace => "Place a 4 randomly",
            UpgradeRandomPlace => "Place 4 faster",
            EnableAutoHarvest => "Harvest largest number regularly",
            UpgradeAutoHarvest => "Harvest faster",
            UpgradeAutomove => "Move faster",
            Reset => "HARD RESET",
            Harvest => "Harvest largest Number",
            ScientificNotation => "Enable log notation for large numbers",
            EnableStatistics => "Show statistics tab",
        }
    }
    fn run(&self) {
        use UpgradeType::*;
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
            EnableStatistics => enable_stats(),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    AlltimePoints(usize),
    HavePoints(usize),
    AvgPoints(usize),
    Harvested(usize),
    BoardSize(usize, usize),
    PointsOnBoard(usize),
    Free(),
}
impl Condition {
    pub fn check(&self) -> bool {
        use Condition::*;
        match self {
            AlltimePoints(p) => attr!(Stats, points) >= *p,
            AvgPoints(p) => attr!(Stats, avg) >= *p as f64,
            PointsOnBoard(p) => get!(UpgradeableBoard, get_points) >= *p,
            HavePoints(p) => attr!(Points, points) >= *p,
            Harvested(p) => attr!(Stats, largest_harvest) >= *p,
            BoardSize(_, _) => unimplemented!(),
            Free() => true,
        }
    }
    pub fn fulfilled(&self) {
        match self {
            HavePoints(p) => Dispatch::<Points>::new().reduce(|points| points.sub(*p)),
            // AlltimePoints(_) => {}
            // Harvested(_) => {}
            // BoardSize(_, _) => {}
            // Free() => {}
            _ => {}
        }
    }
}
impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlltimePoints(p) => write!(f, "Earn a total of {} points", p),
            HavePoints(p) => write!(f, "Requires {} points", p),
            Harvested(p) => write!(f, "Harvest a block of at least {} points", p),
            BoardSize(x, y) => write!(f, "Board must by at least {}x{}", x, y),
            AvgPoints(p) => write!(f, "Earn at least {} points per second", p),
            PointsOnBoard(p) => write!(f, "Have {} points on the board", p),
            Free() => write!(f, "Free"),
        }
    }
}
impl From<usize> for Condition {
    fn from(points: usize) -> Self {
        Condition::HavePoints(points)
    }
}
use Condition::*;

pub fn get_upgrades() -> Vec<Upgrade> {
    use UpgradeType::*;
    vec![
        Upgrade::new(AvgPoints(4), AvgPoints(2), ExtendX, CostsDouble),
        Upgrade::new(PointsOnBoard(256), PointsOnBoard(100), ExtendY, CostsDouble),
        Upgrade::new(64, 32, EnableAutomove, CostsOnetime),
        Upgrade::new(512, 256, UpgradeAutomove, CostsDouble),
        Upgrade::new(Free(), Free(), Harvest, CostsStatic),
        // Upgrade::new(256, 256, "Enable Autoharvesting", EnableAutoHarvest, CostsOnetime),
        // Upgrade::new(1024, 512, "Faster Autoharvesting", UpgradeAutoHarvest, CostsDouble),
        Upgrade::new(1000, 750, ScientificNotation, CostsOnetime),
        Upgrade::new(16, 8, EnableRandomPlace, CostsOnetime),
        Upgrade::new(64, 32, UpgradeRandomPlace, CostsDouble),
        Upgrade::new(12, AlltimePoints(12), EnableStatistics, CostsOnetime),
        Upgrade::new(Free(), Free(), Reset, CostsStatic),
        Upgrade::new(
            Harvested(128),
            Harvested(32),
            ScientificNotation,
            CostsOnetime,
        ),
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
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
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
    // pub visible: Cell<bool>,
    pub done: Cell<bool>,
    pub cost: Condition,
    pub show: Condition,
    pub action: UpgradeType,
    pub costs: UpgradeCosts,
}
// impl Eq for Upgrade {}
impl Upgrade {
    pub fn visible(&self) -> bool {
        // show upgrade at threshold, don't hide it again
        if self.done.get() {
            false
        } else {
            self.show.check()
        }
    }
    pub fn clickable(&self) -> bool {
        self.cost.check()
    }
    pub fn run(&self) {
        // reduce points
        self.cost.fulfilled();
        match self.costs {
            CostsStatic => {}
            CostsDouble => self.costs_double(),
            CostsOnetime => self.done.set(true),
        }
        // check if it should be remain visible
        // self.visible.set(false);
        // self.visible();

        // run whatever the upgrade is supposed to do
        self.action.run();
    }

    fn new<T: Into<Condition>, U: Into<Condition>>(
        cost: T,
        show: U,
        action: UpgradeType,
        costs: UpgradeCosts,
    ) -> Self {
        Self {
            // visible: Cell::new(false),
            done: Cell::new(false),
            cost: cost.into(),
            show: show.into(),
            action,
            costs,
        }
    }
    fn costs_double(&self) {
        // self.cost.set(self.cost.get() * 2);
        // self.show_at.set(self.show_at.get() * 2);
    }
    fn costs_static(&self) {}
    fn costs_onetime(&self) {
        // self.cost.set(usize::MAX);
        // self.show_at.set(usize::MAX);
    }
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
#[derive(Clone, Serialize, Deserialize)]
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

        let action = self.action.clone();
        let cb = Callback::from(move |_| {
            match action {
                Action::AutoMove => run!(UpgradeableBoard, mv),
                Action::AutoSave => do_save(),
                Action::RandomPlace => run!(UpgradeableBoard, random_place, 2),
                Action::AutoHarvest => run!(UpgradeableBoard, harvest),
            };
        });
        self.interval = Some(Interval::new(self.time, move || cb.emit(())));
    }
}
