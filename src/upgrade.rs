use crate::model::UpgradeableBoard;
use gloo::timers::callback::Interval;
use std::fmt::Display;
use std::ops::Deref;
use yewdux::prelude::*;

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
    EnableRandomPlace,
    UpgradeRandomPlace,
    Reset,
    Harvest,
}
use UpgradeType::*;

// impl Into<&'static dyn Fn()> for UpgradeType {
//     fn into(self) -> &'static dyn Fn() {
//         match self {
//             ExtendX => &extend_x,
//             _ => panic!(),
//         }
//     }
// }
impl From<UpgradeType> for Box<dyn Fn()> {
    fn from(u: UpgradeType) -> Self {
        let f: &'static dyn Fn() = match u {
            ExtendX => &extend_x,
            ExtendY => &extend_y,
            EnableAutomove => &enable_automove,
            EnableRandomPlace => &enable_random_place,
            UpgradeAutomove => &upgrade_automove,
            UpgradeRandomPlace => &upgrade_random_place,
            Reset => &reset,
            Harvest => &harvest,
        };
        Box::new(f)
    }
}

pub fn get_upgrades() -> Vec<Rc<Upgrade>> {
    let upgrades = Vec::from([
        Upgrade::new(20, 10, "Extend in X direction", ExtendX, CostsDouble),
        Upgrade::new(256, 100, "Extend in Y direction", ExtendY, CostsDouble),
        Upgrade::new(64, 32, "Enable Automove", EnableAutomove, CostsOnetime),
        Upgrade::new(512, 256, "Upgrade Automove", UpgradeAutomove, CostsDouble),
        Upgrade::new(0, 0, "Harvest", Harvest, CostsStatic),
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
        Upgrade::new(0, 0, "HARD RESET", Reset, CostsStatic),
    ]);
    upgrades.into_iter().map(|x| Rc::new(x)).collect()
}

fn extend_x() {
    let board = Dispatch::<UpgradeableBoard>::new();
    board.reduce(|board| {
        board.extend_x();
        board
    })
}
fn extend_y() {
    let board = Dispatch::<UpgradeableBoard>::new();
    board.reduce(|board| {
        board.extend_y();
        board
    })
}

fn enable_automove() {
    let actions = Dispatch::<AutoActions>::new();
    actions.reduce(|actions| {
        actions.automove.borrow_mut().enable();
        actions
    })
}
fn upgrade_automove() {
    let actions = Dispatch::<AutoActions>::new();
    actions.reduce(|actions| {
        actions.automove();
        actions
    })
}
fn harvest() {
    let board = Dispatch::<UpgradeableBoard>::new();
    board.reduce(|board| {
        board.harvest();
        board
    })
}
fn enable_random_place() {
    let actions = Dispatch::<AutoActions>::new();
    actions.reduce(|actions| {
        actions.random_place.borrow_mut().enable();
        actions
    })
}
fn upgrade_random_place() {
    let actions = Dispatch::<AutoActions>::new();
    actions.reduce(|actions| {
        actions.random_place();
        actions
    })
}
fn reset() {
    log::info!("Reseting game!");
    // Callback::once(|_| {
    Dispatch::<Points>::new().set(Default::default());
    Dispatch::<UpgradeableBoard>::new().set(Default::default());
    Dispatch::<AutoActions>::new().set(Default::default());
    Dispatch::<Upgrades>::new().set(Default::default());
    // })
}

use serde::Deserialize;
use serde::Serialize;
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
        log::info!("New Points: {}", self.points + points);
        Points {
            points: self.points + points,
        }
    }
}
use yewdux::storage;
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

use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Upgrades {
    pub upgrades: Vec<Rc<Upgrade>>,
}
impl Store for Upgrades {
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
    fn new() -> Self {
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

impl PartialEq for Upgrade {
    fn eq(&self, other: &Self) -> bool {
        self.text.eq(&other.text)
    }
}
use std::cell::Cell;
#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    pub visible: Cell<bool>,
    pub cost: Cell<usize>,
    pub show_at: Cell<usize>,
    pub text: String,
    pub action: UpgradeType,
    pub costs: UpgradeCosts,
}
impl Eq for Upgrade {}
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
        let action: Box<dyn Fn()> = Box::from(self.action);
        action();
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
fn do_random_place() {
    let dispatch = Dispatch::<UpgradeableBoard>::new();
    dispatch.reduce(|board| {
        board.random_place(4);
        board
    });
}
use yewdux::storage::Area::Local;
fn do_save() {
    let dispatch = Dispatch::<Points>::new();
    storage::save(dispatch.get().as_ref(), Local).unwrap();

    let dispatch = Dispatch::<UpgradeableBoard>::new();
    storage::save(dispatch.get().as_ref(), Local).unwrap();
    let dispatch = Dispatch::<AutoActions>::new();
    storage::save(dispatch.get().as_ref(), Local).unwrap();

    let dispatch = Dispatch::<Upgrades>::new();
    storage::save(dispatch.get().as_ref(), Local).unwrap();
}

use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct AutoActions {
    automove: RefCell<AutoAction>,
    random_place: RefCell<AutoAction>,
    save: RefCell<AutoAction>,
}
impl AutoActions {
    fn automove(&self) {
        self.automove.borrow_mut().upgrade(50);
    }
    fn random_place(&self) {
        self.random_place.borrow_mut().upgrade(50);
    }
}

impl Default for AutoActions {
    fn default() -> Self {
        let a = AutoActions {
            automove: RefCell::new(AutoAction::new(None, Action::AutoMove, 1000, false)),
            random_place: RefCell::new(AutoAction::new(None, Action::RandomPlace, 1000, false)),
            save: RefCell::new(AutoAction::new(None, Action::Save, 5000, true)),
        };
        a.save.borrow_mut().set_callback();
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
        me.save.borrow_mut().set_callback();
        me.random_place.borrow_mut().set_callback();
        me
    }
}
#[derive(Serialize, Deserialize)]
enum Action {
    AutoMove,
    RandomPlace,
    Save,
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
        assert!(self.active);
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
            Action::Save => &do_save,
            Action::RandomPlace => &do_random_place,
        };

        let dispatch = Dispatch::<UpgradeableBoard>::new();
        let cb = dispatch.reduce_callback(|action| {
            (f)();
            action
        });
        self.interval = Some(Interval::new(self.time, move || cb.emit(())));
    }
}
