use crate::model::UpgradeableBoard;
use crate::*;
use gloo::timers::callback::Interval;
use std::fmt::Display;

macro_rules! run {
    ($a:ident, $b:ident) => {{
        log::info!("Running ",);
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

#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UpgradeCosts {
    CostsDouble,
    CostsOnetime,
    CostsStatic,
}
#[derive(Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Debug)]
pub enum UpgradeType {
    EnableShuffle,
    AutoShuffle(usize),
    Shuffle,
    Place(usize),
    ExtendX(usize),
    ExtendY(usize),
    EnableAutomove,
    UpgradeAutomove(usize),
    EnableAutoHarvest,
    UpgradeAutoHarvest(usize),
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
            EnableShuffle => "Enable shuffling the board".into(),
            AutoShuffle(i) => format!("Automatically shuffle the board every {}ms", i),
            Shuffle => "Shuffle the Board".into(),
            Place(n) => format!("Place a {}", n),
            ExtendX(_) => "Extend Board horizontally".into(),
            ExtendY(_) => "Extend Board vertically".into(),
            EnableAutomove => "Automatically move the board".into(),
            EnableRandomPlace => "Place a 4 randomly".into(),
            UpgradeRandomPlace(_) => "Place 4 faster".into(),
            EnableAutoHarvest => "Harvest largest number regularly".into(),
            UpgradeAutoHarvest(_) => "Harvest faster".into(),
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
            EnableShuffle => Dispatch::<Shuffles>::new().reduce_mut(|s| s.enable()),
            AutoShuffle(x) => run!(AutoActions, upgrade_autoshuffle, *x),
            Shuffle => board(&UpgradeableBoard::shuffle),
            Place(x) => run!(UpgradeableBoard, random_place, *x),
            ExtendX(_) => run!(UpgradeableBoard, extend_x),
            ExtendY(_) => run!(UpgradeableBoard, extend_y),
            EnableAutomove => run!(AutoActions, enable_automove),
            EnableRandomPlace => run!(AutoActions, enable_random_place),
            UpgradeAutomove(n) => run!(AutoActions, upgrade_automove, *n),
            UpgradeRandomPlace(n) => run!(AutoActions, upgrade_random_place, *n),
            EnableAutoHarvest => run!(AutoActions, enable_autoharvest),
            UpgradeAutoHarvest(n) => run!(AutoActions, upgrade_autoharvest, *n),
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
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Condition {
    AlltimePoints(usize),
    HavePoints(usize),
    HaveShuffles(usize),
    AvgPoints(usize),
    Harvested(usize),
    BoardSize(usize, usize),
    PointsOnBoard(usize),
    Free(),
    // Multi(Box<Condition>, Box<Condition>),
    UpgradeDone(UpgradeType),
    Until(UpgradeType),
    // Between(Box<Condition>, Box<Condition>),
}
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

fn get<S: Store>() -> Rc<S> {
    Dispatch::<S>::new().get()
}
struct Nothing;
impl Store for Nothing {
    fn should_notify(&self, _old: &Self) -> bool {
        false
    }
    fn new() -> Self {
        Nothing
    }
}
use std::any::Any;
impl Condition {
    pub fn check(&self) -> bool {
        use Condition::*;
        match self {
            // Multi(a, b) => a.check() && b.check(),
            AlltimePoints(p) => get::<Stats>().points >= *p,
            AvgPoints(p) => get::<Avg>().get_avg() >= *p as f64,
            PointsOnBoard(p) => get::<UpgradeableBoard>().get_points() >= *p,
            HavePoints(p) => get::<Points>().get() >= *p,
            HaveShuffles(p) => get::<Shuffles>().get() >= *p as f64,
            Harvested(p) => get::<Stats>().largest_harvest >= *p,
            BoardSize(x, y) => {
                let p = get::<UpgradeableBoard>().boardsize();
                p.x >= *x && p.y >= *y
            }
            UpgradeDone(upgrade) => get::<Upgrades>().is_done(*upgrade),
            Until(upgrade) => !get::<Upgrades>().is_clickable(*upgrade),
            // Between(a, b) => a.check() && !b.check(),
            Free() => true,
        }
    }
    pub fn watch(&self) -> Box<dyn Any> {
        use Condition::*;
        match self {
            AvgPoints(_) => Box::new(use_store::<Avg>().1),
            HavePoints(_) => Box::new(use_store::<Points>().1),
            HaveShuffles(_) => Box::new(use_store::<Shuffles>().1),
            PointsOnBoard(_) => Box::new(use_store::<UpgradeableBoard>().1),
            AlltimePoints(_) => Box::new(use_store::<Stats>().1),
            UpgradeDone(_) => Box::new(use_store::<Upgrades>().1),
            Until(_) => Box::new(use_store::<Upgrades>().1),
            BoardSize(_, _) => Box::new(use_store::<UpgradeableBoard>()),
            Free() => Box::new(Dispatch::<Nothing>::new()),
            Harvested(_) => Box::new(use_store::<Stats>().1),
        }
    }
    pub fn fulfilled(&self) {
        match self {
            HavePoints(p) => Dispatch::<Points>::new().reduce(|points| points.sub(*p)),
            HaveShuffles(p) => Dispatch::<Shuffles>::new().reduce_mut(|s| s.sub(*p as f64)),
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
            HaveShuffles(p) => write!(f, "Requires {} shuffles", p),
            Harvested(p) => write!(f, "Harvest a block of at least {} points", p),
            BoardSize(x, y) => write!(f, "Board must by at least {}x{}", x, y),
            AvgPoints(p) => write!(f, "Earn at least {} points per second", p),
            PointsOnBoard(p) => write!(f, "Have {} points on the board", p),
            Free() => write!(f, "Free"),
            // Multi(a, b) => write!(f, "{} AND {}", a, b),
            UpgradeDone(upgrade) => write!(f, "Bought Upgrade \"{}\"", upgrade),
            Until(_) => write!(f, ""),
            // Between(a, b) => write!(f, "{}", a),
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
        // Shuffle
        Upgrade::new(HaveShuffles(1), EnableShuffle, Shuffle).static_(),
        // RESET
        Upgrade::new(Free(), Free(), Reset).static_(),
        // Upgrades
        // ExtendX
        Upgrade::new(AvgPoints(4), Free(), ExtendX(1)),
        Upgrade::new(AvgPoints(8), ExtendX(1), ExtendX(2)),
        Upgrade::new(AvgPoints(15), ExtendX(2), ExtendX(3)),
        // ExtendY
        Upgrade::new(PointsOnBoard(256), PointsOnBoard(128), ExtendY(1)),
        Upgrade::new(PointsOnBoard(1024), ExtendY(1), ExtendY(2)),
        Upgrade::new(PointsOnBoard(4096), ExtendY(2), ExtendY(3)),
        // Automation
        Upgrade::new(64, 32, EnableAutomove),
        Upgrade::new(512, EnableAutomove, UpgradeAutomove(750)),
        Upgrade::new(10_000, UpgradeAutomove(750), UpgradeAutomove(500)),
        Upgrade::new(100_000, UpgradeAutomove(500), UpgradeAutomove(250)),
        // Upgrade::new(256, 256, "Enable Autoharvesting", EnableAutoHarvest, CostsOnetime),
        // Upgrade::new(1024, 512, "Faster Autoharvesting", UpgradeAutoHarvest, CostsDouble),
        Upgrade::new(16, 8, EnableRandomPlace),
        Upgrade::new(64, EnableRandomPlace, UpgradeRandomPlace(800)),
        Upgrade::new(256, UpgradeRandomPlace(800), UpgradeRandomPlace(500)),
        // Shuffling
        Upgrade::new(1_000, UpgradeAutomove(500), EnableShuffle),
        Upgrade::new(HaveShuffles(100), BoardSize(6,6), AutoShuffle(30000)),



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
    do_save();
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
        self.cost.check() && self.visible()
    }
    pub fn run(&self) {
        // reduce points
        self.cost.fulfilled();
        match self.status {
            UpgradeStatus::Static => {}
            UpgradeStatus::OneTime => {
                self.done.set(true);
                Dispatch::<Upgrades>::new().reduce(|x| x);
            }
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
    info!("Saving");
    save!(Upgrades);
    save!(Points);
    save!(UpgradeableBoard);
    save!(AutoActions);
    save!(Stats);
    save!(Shuffles);
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct AutoActions {
    automove: RefCell<AutoAction>,
    autoharvest: RefCell<AutoAction>,
    random_place: RefCell<AutoAction>,
    autosave: RefCell<AutoAction>,
    autoshuffle: RefCell<AutoAction>,
}
impl AutoActions {
    fn upgrade_automove(&self, time: usize) {
        self.automove.borrow_mut().upgrade(time);
    }
    fn enable_automove(&self) {
        self.automove.borrow_mut().enable();
    }
    fn upgrade_autoshuffle(&self, time: usize) {
        self.autoshuffle.borrow_mut().upgrade(time);
    }
    fn enable_autoshuffle(&self) {
        self.autoshuffle.borrow_mut().enable();
    }
    fn upgrade_random_place(&self, time: usize) {
        self.random_place.borrow_mut().upgrade(time);
    }
    fn enable_random_place(&self) {
        self.random_place.borrow_mut().enable();
    }
    fn upgrade_autoharvest(&self, time: usize) {
        self.random_place.borrow_mut().upgrade(time);
    }
    fn enable_autoharvest(&self) {
        self.autoharvest.borrow_mut().enable();
    }
}

impl Default for AutoActions {
    fn default() -> Self {
        AutoActions {
            automove: RefCell::new(AutoAction::new(None, Action::AutoMove, 1000, false)),
            autoharvest: RefCell::new(AutoAction::new(None, Action::AutoHarvest, 10000, false)),
            random_place: RefCell::new(AutoAction::new(None, Action::RandomPlace, 1000, false)),
            autosave: RefCell::new(AutoAction::new(None, Action::AutoSave, 5000, true)),
            autoshuffle: RefCell::new(AutoAction::new(None, Action::AutoShuffle, 10000, false)),
        }
    }
}

impl Store for AutoActions {
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
    fn new() -> Self {
        let me: Self = storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default();
        me.autoshuffle.borrow_mut().set_callback();
        me.autosave.borrow_mut().set_callback();
        me.autoharvest.borrow_mut().set_callback();
        me.automove.borrow_mut().set_callback();
        me.random_place.borrow_mut().set_callback();
        me
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
enum Action {
    AutoMove,
    RandomPlace,
    AutoSave,
    AutoHarvest,
    AutoShuffle,
}
#[derive(Serialize, Deserialize)]
struct AutoAction {
    #[serde(skip)]
    interval: Option<Interval>,
    action: Action,
    time: usize,
    active: bool,
}
impl PartialEq for AutoAction {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
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
        self.time = time;
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
                Action::AutoShuffle => run!(UpgradeableBoard, shuffle),
            };
        });
        self.interval = Some(Interval::new(self.time as u32, move || cb.emit(())));
    }
}
