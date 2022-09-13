use crate::model::UpgradeableBoard;
use crate::*;
use gloo::timers::callback::Interval;
use std::fmt::Display;

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
    ($a:ident, $b:ident, $c:expr) => {{
        let dispatch = Dispatch::<$a>::new().get();
        dispatch.$b($c)
    }};
}

#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UpgradeCosts {
    CostsDouble,
    CostsOnetime,
    CostsStatic,
}
#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UpgradeType {
    Shuffle,
    Place(usize),
    ExtendX(usize),
    ExtendY(usize),
    EnableAutomove,
    UpgradeAutomove(usize),
    EnableAutoHarvest,
    UpgradeAutoHarvest,
    EnableRandomPlace,
    UpgradeRandomPlace(usize),
    Reset,
    Harvest,
    ScientificNotation,
    EnableStatistics,
    NlogNCost,
    BonusTile(usize, usize),
}
impl Display for UpgradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}
impl<'a> UpgradeType {
    pub fn text(&self) -> String {
        use UpgradeType::*;
        match self {
            Shuffle => "Shuffle the Board".into(),
            Place(n) => format!("Place a {}", n),
            ExtendX(_) => "Extend Board horizontally".into(),
            ExtendY(_) => "Extend Board vertically".into(),
            EnableAutomove => "Automatically move the board".into(),
            EnableRandomPlace => "Place a 4 randomly".into(),
            UpgradeRandomPlace(_) => "Place 4 faster".into(),
            EnableAutoHarvest => "Harvest largest number regularly".into(),
            UpgradeAutoHarvest => "Harvest faster".into(),
            UpgradeAutomove(_) => "Move faster".into(),
            Reset => "HARD RESET".into(),
            Harvest => "Harvest largest Number".into(),
            ScientificNotation => "Enable log notation for large numbers".into(),
            EnableStatistics => "Show statistics tab".into(),
            NlogNCost => "Each harvest gives n * log(n) points instead of n".into(),
            BonusTile(x, y) => format!("Bonus {} when merging a {}", 1 << y, 1 << x),
        }
    }
    fn run(&self) {
        use UpgradeType::*;
        match self {
            Shuffle => run!(UpgradeableBoard, shuffle),
            Place(x) => run!(UpgradeableBoard, random_place, *x),
            ExtendX(_) => run!(UpgradeableBoard, extend_x),
            ExtendY(_) => run!(UpgradeableBoard, extend_y),
            EnableAutomove => run!(AutoActions, enable_automove),
            EnableRandomPlace => run!(AutoActions, enable_random_place),
            UpgradeAutomove(n) => run!(AutoActions, upgrade_automove,*n ),
            UpgradeRandomPlace(n) => run!(AutoActions, upgrade_random_place,*n),
            EnableAutoHarvest => run!(AutoActions, enable_autoharvest),
            UpgradeAutoHarvest => run!(AutoActions, upgrade_autoharvest),
            Reset => reset(),
            Harvest => run!(UpgradeableBoard, harvest),
            ScientificNotation => run!(UpgradeableBoard, scientific_notation),
            EnableStatistics => enable_stats(),
            NlogNCost => Dispatch::<Points>::new().reduce(|p| p.set_log()),
            BonusTile(x, y) => run!(UpgradeableBoard, set_combine_fn, CombineFn::Bonus(*x, *y)),
        }
    }
}
impl From<UpgradeType> for Condition {
    fn from(u: UpgradeType) -> Self {
        UpgradeDone(u)
    }
}
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Condition {
    AlltimePoints(usize),
    HavePoints(usize),
    AvgPoints(usize),
    Harvested(usize),
    BoardSize(usize, usize),
    PointsOnBoard(usize),
    Free(),
    Multi(Box<Condition>, Box<Condition>),
    UpgradeDone(UpgradeType),
    Until(UpgradeType),
    Between(Box<Condition>, Box<Condition>),
}
impl Condition {
    pub fn check(&self) -> bool {
        use Condition::*;
        match self {
            Multi(a, b) => a.check() && b.check(),
            AlltimePoints(p) => attr!(Stats, points) >= *p,
            AvgPoints(p) => get!(Avg, get_avg) >= *p as f64,
            PointsOnBoard(p) => get!(UpgradeableBoard, get_points) >= *p,
            HavePoints(p) => get!(Points, get) >= *p,
            Harvested(p) => attr!(Stats, largest_harvest) >= *p,
            BoardSize(_, _) => unimplemented!(),
            UpgradeDone(upgrade) => get!(Upgrades, is_done, *upgrade),
            Until(upgrade) => !get!(Upgrades, is_clickable, *upgrade),
            Between(a, b) => a.check() && !b.check(),
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
            Multi(a, b) => write!(f, "{} AND {}", a, b),
            UpgradeDone(upgrade) => write!(f, "Bought Upgrade \"{}\"", upgrade),
            Until(upgrade) => write!(f, ""),
            Between(a, b) => write!(f, "{}", a),
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
    // price, show condition, action [static]
    [
        // manual placing
        Upgrade::new(Free(), Until(Place(4)), Place(2)).static_(),
        Upgrade::new(Free(), AlltimePoints(1000), Place(4)).static_(),
        // Harvest
        Upgrade::new(Free(), Free(), Harvest).static_(),
        Upgrade::new(16, UpgradeAutomove(800), Shuffle).static_(),
        // RESET
        Upgrade::new(Free(), Free(), Reset).static_(),
        // Upgrades
        // ExtendX
        Upgrade::new(AvgPoints(4), Free(), ExtendX(1)),
        Upgrade::new(AvgPoints(8), AvgPoints(6), ExtendX(2)),
        // ExtendY
        Upgrade::new(PointsOnBoard(256), PointsOnBoard(100), ExtendY(1)),
        Upgrade::new(PointsOnBoard(1024), ExtendY(1), ExtendY(2)),
        // Automation
        Upgrade::new(64, 32, EnableAutomove),
        Upgrade::new(512, EnableAutomove, UpgradeAutomove(800)),
        Upgrade::new(10_000, UpgradeAutomove(800), UpgradeAutomove(500)),
        Upgrade::new(100_000, UpgradeAutomove(500), UpgradeAutomove(250)),
        // Upgrade::new(256, 256, "Enable Autoharvesting", EnableAutoHarvest, CostsOnetime),
        // Upgrade::new(1024, 512, "Faster Autoharvesting", UpgradeAutoHarvest, CostsDouble),
        Upgrade::new(16, 8, EnableRandomPlace),
        Upgrade::new(64, EnableRandomPlace, UpgradeRandomPlace(800)),
        Upgrade::new(256, UpgradeRandomPlace(800), UpgradeRandomPlace(500)),
        // Stats and display
        Upgrade::new(AlltimePoints(1000), AlltimePoints(500), ScientificNotation),
        Upgrade::new(12, AlltimePoints(12), EnableStatistics),
        Upgrade::new(Harvested(128), Harvested(32), ScientificNotation),
        Upgrade::new(Harvested(16), Harvested(16), BonusTile(6, 4)),
        Upgrade::new(Harvested(128), EnableAutomove, NlogNCost),
    ]
    .into()
}

fn enable_stats() {
    Dispatch::<Stats>::new().reduce_mut(|stats| stats.enable());
}
fn reset() {
    if !gloo::dialogs::confirm("Perform hard reset?") {
        return;
    }
    log::info!("Reseting game!");
    Dispatch::<Points>::new().set(Default::default());
    Dispatch::<UpgradeableBoard>::new().set(Default::default());
    Dispatch::<AutoActions>::new().set(Default::default());
    Dispatch::<Upgrades>::new().set(Default::default());
    Dispatch::<Stats>::new().set(Default::default());
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

#[derive(Serialize, Deserialize)]
pub struct Upgrades {
    pub upgrades: Vec<Rc<Upgrade>>,
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
            upgrades: get_upgrades().into_iter().map(Rc::new).collect(),
        }
    }
}

impl Upgrades {
    fn is_done(&self, t: UpgradeType) -> bool {
        let upgrade = self.upgrades.iter().find(|u| u.action == t).unwrap();
        upgrade.done.get()
    }
    fn is_clickable(&self, t: UpgradeType) -> bool {
        let upgrade = self.upgrades.iter().find(|u| u.action == t).unwrap();
        upgrade.clickable()
    }
    pub fn statics(&self) -> impl Iterator<Item = &Rc<Upgrade>> {
        self.upgrades
            .iter()
            .filter(|u| u.status == UpgradeStatus::Static)
    }
    pub fn onetimes(&self) -> impl Iterator<Item = &Rc<Upgrade>> {
        self.upgrades
            .iter()
            .filter(|u| u.status == UpgradeStatus::OneTime)
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum UpgradeStatus {
    OneTime,
    Static,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Upgrade {
    // pub visible: Cell<bool>,
    pub done: Cell<bool>,
    pub cost: Condition,
    pub show: Condition,
    pub action: UpgradeType,
    pub status: UpgradeStatus,
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
        match self.status {
            UpgradeStatus::Static => {}
            UpgradeStatus::OneTime => self.done.set(true),
        }
        // check if it should be remain visible
        // self.visible.set(false);
        // self.visible();

        // run whatever the upgrade is supposed to do
        self.action.run();
    }

    fn new<T, U>(cost: T, show: U, action: UpgradeType) -> Self
    where
        T: Into<Condition>,
        U: Into<Condition>,
    {
        Self {
            done: Cell::new(false),
            cost: cost.into(),
            show: show.into(),
            action,
            status: UpgradeStatus::OneTime,
        }
    }
    fn static_(mut self) -> Self {
        self.status = UpgradeStatus::Static;
        self
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
    fn upgrade_automove(&self, time: usize) {
        self.automove.borrow_mut().upgrade(time);
    }
    fn enable_automove(&self) {
        self.automove.borrow_mut().enable();
    }
    fn upgrade_random_place(&self, time: usize) {
        self.random_place.borrow_mut().upgrade(time);
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
       storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
        //activate timers
        // me.automove.borrow_mut().set_callback();
        // me.autosave.borrow_mut().set_callback();
        // me.random_place.borrow_mut().set_callback();
        // me.autoharvest.borrow_mut().set_callback();
        // me
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
    time: usize,
    active: bool,
}

impl AutoAction {
    fn new(interval: Option<Interval>, f: Action, time: usize, active: bool) -> Self {
        let mut me = Self {
            interval,
            action: f,
            time,
            active,
        };
        me.set_callback();
        me
    }
    fn upgrade(&mut self, time: usize) {
        self.time = time ;
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
                Action::RandomPlace => run!(UpgradeableBoard, random_place, 4),
                Action::AutoHarvest => run!(UpgradeableBoard, harvest),
            };
        });
        self.interval = Some(Interval::new(self.time as u32, move || cb.emit(())));
    }
}
