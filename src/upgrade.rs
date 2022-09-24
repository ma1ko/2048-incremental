use crate::model::UpgradeableBoard;
use crate::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::marker::PhantomData;

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

#[derive(Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum UpgradeCosts {
    CostsMultiply,
    CostsOnetime,
    CostsStatic,
}

// #[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug, Hash, Eq, PartialOrd, Ord)]
// pub enum Action {
//     Slider(SliderType),
//     EnableShuffle,
//     AutoShuffle,
//     Shuffle,
//     Place,
//     ExtendX,
//     ExtendY,
//     EnableAutomove,
//     UpgradeAutomove,
//     EnableAutoHarvest,
//     UpgradeAutoHarvest,
//     EnableRandomPlace,
//     UpgradeRandomPlace,
//     Reset,
//     Harvest,
//     ScientificNotation,
//     EnableStatistics,
//     NlogNCost,
//     BonusTile,
//     Cheat,
// }
// impl Display for UpgradeType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.text())
//     }
// }
// impl<T: IsUpgrade> Display for Upgrade<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let level = self.level.get();
//         let text = match self.action {
//             Slider(s) => format!("Enable {}", s).into(),
//             EnableShuffle => "Enable shuffling the board".into(),
//             AutoShuffle => format!("Automatically shuffle the board every {:?}ms", level),
//             Shuffle => "Shuffle the Board".into(),
//             Place => format!("Place a {:?}", level),
//             ExtendX => "Extend Board horizontally".into(),
//             ExtendY => "Extend Board vertically".into(),
//             EnableAutomove => "Automatically move the board".into(),
//             EnableRandomPlace => "Place a 4 randomly".into(),
//             UpgradeRandomPlace => "Place 4 faster".into(),
//             EnableAutoHarvest => "Harvest largest number regularly".into(),
//             UpgradeAutoHarvest => "Harvest faster".into(),
//             UpgradeAutomove => "Move faster".into(),
//             Reset => "HARD RESET".into(),
//             Harvest => "Harvest largest Number".into(),
//             ScientificNotation => "Enable log notation for large numbers".into(),
//             EnableStatistics => "Show statistics tab".into(),
//             NlogNCost => "Each harvest gives n * log(n) points instead of n".into(),
//             BonusTile => format!("Bonus {} when merging a {}", 1 << level, 1 << level / 4),
//             Cheat => format!("CHEAT"),
//         };
//         write!(f, "{}", text)
//     }
// }
// impl<'a> Action{
// pub fn text(&self) -> String {
//     use UpgradeType::*;
//     match self {
//         EnableShuffle => "Enable shuffling the board".into(),
//         AutoShuffle(i) => format!("Automatically shuffle the board every {}ms", i),
//         Shuffle => "Shuffle the Board".into(),
//         Place(n) => format!("Place a {}", n),
//         ExtendX(_) => "Extend Board horizontally".into(),
//         ExtendY(_) => "Extend Board vertically".into(),
//         EnableAutomove => "Automatically move the board".into(),
//         EnableRandomPlace => "Place a 4 randomly".into(),
//         UpgradeRandomPlace(_) => "Place 4 faster".into(),
//         EnableAutoHarvest => "Harvest largest number regularly".into(),
//         UpgradeAutoHarvest(_) => "Harvest faster".into(),
//         UpgradeAutomove(_) => "Move faster".into(),
//         Reset => "HARD RESET".into(),
//         Harvest => "Harvest largest Number".into(),
//         ScientificNotation => "Enable log notation for large numbers".into(),
//         EnableStatistics => "Show statistics tab".into(),
//         NlogNCost => "Each harvest gives n * log(n) points instead of n".into(),
//         BonusTile(x, y) => format!("Bonus {} when merging a {}", 1 << y, 1 << x),
//         Cheat(i) => format!("CHEAT")
//     }
// }
// }
// use Action::*;

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

use crate::condition::Condition::*;

/*
pub fn get_upgrades() -> Vec<Upgrade> {
    use Action::*;
    // price, show condition, action [static]
    [
        // manual placing
        Upgrade::new(Free(), Free(), Cheat).static_(),
        // Upgrade::new(Free(), Until(Place), Place).static_(),
        // Upgrade::new(
        //     Free(),
        //     Between(AlltimePoints(1000).into(), Place(8).into()),
        //     Place(4),
        // )
        // .static_(),
        // Upgrade::new(Free(), AlltimePoints(10000), Place(8)).static_(),
        // Harvest
        Upgrade::new(Free(), Free(), Harvest).static_(),
        // Shuffle
        Upgrade::new(HaveShuffles(1), EnableShuffle, Shuffle).static_(),
        // RESET
        Upgrade::new(Free(), Free(), Reset).static_(),
        // Upgrades
        // ExtendX
        Upgrade::new(0, AvgPoints(0), Slider(SliderType::SliderX)),
        // Upgrade::new(AvgPoints(4), Free(), ExtendX(1)),
        // Upgrade::new(AvgPoints(8), ExtendX(1), ExtendX(2)),
        // Upgrade::new(AvgPoints(15), ExtendX(2), ExtendX(3)),
        // ExtendY
        Upgrade::new(PointsOnBoard(256), PointsOnBoard(128), ExtendY),
        Upgrade::new(PointsOnBoard(1024), ExtendY, ExtendY),
        Upgrade::new(PointsOnBoard(4096), ExtendY, ExtendY),
        // Automation
        Upgrade::new(64, 32, EnableAutomove),
        Upgrade::new(512, EnableAutomove, UpgradeAutomove).multiply(2),
        // Upgrade::new(10_000, UpgradeAutomove(750), UpgradeAutomove(500)),
        // Upgrade::new(100_000, UpgradeAutomove(500), UpgradeAutomove(250)),
        // Upgrade::new(256, 256, "Enable Autoharvesting", EnableAutoHarvest, CostsOnetime),
        // Upgrade::new(1024, 512, "Faster Autoharvesting", UpgradeAutoHarvest, CostsDouble),
        // Upgrade::new(16, 8, EnableRandomPlace),
        Upgrade::new(0, 0, UpgradeRandomPlace),
        // Upgrade::new(256, UpgradeRandomPlace, UpgradeRandomPlace),
        // Shuffling
        Upgrade::new(1_000, UpgradeAutomove, EnableShuffle),
        Upgrade::new(HaveShuffles(100), BoardSize(6, 6), AutoShuffle),
        // Stats and display
        Upgrade::new(AlltimePoints(1000), AlltimePoints(500), ScientificNotation),
        Upgrade::new(12, AlltimePoints(12), EnableStatistics),
        Upgrade::new(Harvested(128), Harvested(32), ScientificNotation),
        Upgrade::new(Harvested(16), Harvested(16), BonusTile),
        Upgrade::new(Harvested(128), EnableAutomove, NlogNCost),
    ]
    .into()
}
*/

fn enable_stats() {
    Dispatch::<Stats>::new().reduce_mut(|stats| stats.enable());
}
use gloo_storage::Storage;
pub fn reset() {
    if !gloo::dialogs::confirm("Perform hard reset?") {
        return;
    }
    log::info!("Reseting game!");
    // Dispatch::<Points>::new().set(Default::default());
    // Dispatch::<UpgradeableBoard>::new().set(Default::default());
    // Dispatch::<AutoActions>::new().set(Default::default());
    // Dispatch::<Upgrades>::new().set(Default::default());
    // Dispatch::<Stats>::new().set(Default::default());
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
// use serde_with::serde_as;
// #[serde_as]
// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Upgrades {
    // pub upgrades: Vec<Rc<Upgrade>>,
    // #[serde_as(as = "Vec<(_,_)>")]
    // pub upgrades: HashMap<Action, Rc<Upgrade>>,
    // pub upgrades: Vec<&'static dyn IsUpgrade>,
}
// impl Store for Upgrades {
//     fn should_notify(&self, _old: &Self) -> bool {
//         true
//     }
//     fn new() -> Self {
//         storage::load(storage::Area::Local)
//             .expect("Unable to load state")
//             .unwrap_or_default()
//     }
// }

impl Default for Upgrades {
    fn default() -> Self {
        Upgrades {
            // upgrades: vec![] //&Place {}, &ExtendX {}, &ExtendY {}],
        }
    }
}

impl Upgrades {
    // pub fn is_done(&self, t: Action ) -> bool {
    //     let upgrade = self.upgrades.get(&t);
    //     if let Some(upgrade) = upgrade {
    //         upgrade.done.get()
    //     } else {
    //         false // either upgrade doesn't exist or it will be created some time
    //     }
    // }
    // pub fn is_clickable(&self, t: Action ) -> bool {
    //     let upgrade = self.upgrades.get(&t);
    //     if let Some(upgrade) = upgrade {
    //         upgrade.clickable()
    //     } else {
    //         false // either upgrade doesn't exist or it will be created some time
    //     }
    // }
    // pub fn statics(&self) -> impl Iterator<Item = &Rc<Upgrade>> {
    //     self.upgrades
    //         .iter()
    //         .filter(|u| u.status == UpgradeStatus::Static)
    // }
    // pub fn upgrades(&self) -> impl Iterator<Item = &Rc<Upgrade>> {
    //     self.upgrades
    //         .iter()
    //         .filter(|u| u.status != UpgradeStatus::Static)
    // }
    // pub fn sliders(&self) -> impl Iterator<Item = &Rc<Upgrade>> {
    //     self.upgrades
    //         .values()
    //         .filter(|u| u.status == UpgradeStatus::Slider)
    // }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum UpgradeStatus {
    OneTime,
    Static,
    Multiply(usize),
}

// #[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct Upgrade<T: IsUpgrade> {
    max_level: usize,
    pub level: usize,
    pub visible: Cell<bool>,
    pub done: bool,
    pub cost: Condition,
    pub show: Condition,
    // pub action: Action,
    pub status: UpgradeStatus,
    pub t: PhantomData<*const T>,
    pub text: String,
}

impl<T: IsUpgrade> Store for Upgrade<T> {
    fn new() -> Self {
        // storage::load(storage::Area::Local)
        // .expect("Unable to load state")
        // .unwrap_or_default()
        Default::default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

impl<T: IsUpgrade> Display for Upgrade<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}
impl<T: IsUpgrade> Default for Upgrade<T> {
    fn default() -> Self {
        Upgrade::<T>::new(
            Free(),
            Free(),
            format!("Unimplemented {}", std::any::type_name::<T>()),
        )
    }
}
impl<T: IsUpgrade> Clone for Upgrade<T> {
    fn clone(&self) -> Self {
        Self {
            visible: self.visible.clone(),
            t: self.t,
            show: self.show.clone(),
            cost: self.cost.clone(),
            text: self.text.clone(),
            ..*self
        }
    }
}
impl<T: IsUpgrade> Upgrade<T> {
    pub fn visible(&self) -> bool {
        // don't hide an upgrade after it has been shown once
        if self.visible.get() {
            return true;
        }
        if self.done {
            false
        } else {
            let vis = self.show.check();
            if vis { self.visible.set(true)}
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
            // UpgradeStatus::Slider => {
            //     self.done.set(true);
            //     if self.level == self.max_level {
            //         self.level.set(self.level.get() + 1)
            //     }
            //     self.max_level.set(self.max_level.get());
            // }
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
        }
        if self.level == self.max_level {
            self.done = true;

        }
        // self.run();

        // Dispatch::<Upgrades>::new().reduce(|upgrades| upgrades);
    }
    pub fn run(&mut self) {
        self.upgrade();
        T::run(self.level);
        // run whatever the upgrade is supposed to do
        // self.action.run();
        // let level = self.level;
        // match self.action {
        //     Slider(s) => Dispatch::<Sliders>::new().get().sliders[&s].increase(),
        //     Cheat => Dispatch::<Points>::new().reduce_mut(|p| p.add(1_000_000)),
        //     EnableShuffle => Dispatch::<Shuffles>::new().reduce_mut(|s| s.enable()),
        //     AutoShuffle => run!(AutoActions, upgrade_autoshuffle, level),
        //     Shuffle => board(&UpgradeableBoard::shuffle),
        //     Place => run!(UpgradeableBoard, random_place, level),
        //     ExtendX => run!(UpgradeableBoard, extend_x),
        //     ExtendY => run!(UpgradeableBoard, extend_y),
        //     EnableAutomove => run!(AutoActions, enable_automove),
        //     EnableRandomPlace => run!(AutoActions, enable_random_place),
        //     UpgradeAutomove => run!(AutoActions, upgrade_automove, level),
        //     UpgradeRandomPlace => run!(AutoActions, upgrade_random_place, level),
        //     EnableAutoHarvest => run!(AutoActions, enable_autoharvest),
        //     UpgradeAutoHarvest => run!(AutoActions, upgrade_autoharvest, level),
        //     Reset => reset(),
        //     Harvest => run!(UpgradeableBoard, harvest),
        //     ScientificNotation => run!(UpgradeableBoard, scientific_notation),
        //     EnableStatistics => enable_stats(),
        //     NlogNCost => Dispatch::<Points>::new().reduce_mut(|p| p.set_log()),
        //     BonusTile => run!(
        //         UpgradeableBoard,
        //         set_combine_fn,
        //         CombineFn::Bonus(level, level / 4)
        //     ),
        // }
    }
    // fn improve(&mut self, x: usize) {
    //     let level = &mut self.level.get();
    //     match self.action {
    //         Place => *level = *level * 2,
    //         ExtendX => *level += 1,
    //         ExtendY => *level += 1,
    //         UpgradeRandomPlace => *level = ((*level as f64) * 0.9) as usize,
    //         UpgradeAutomove => *level = ((*level as f64) * 0.9) as usize,

    //         _ => unimplemented!(),
    //     };
    // }

    pub fn new<C, S, D>(cost: C, show: S, text: D) -> Self
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
            t: Default::default(),
            text: text.into(),
        }
    }
    pub fn static_(mut self) -> Self {
        self.status = UpgradeStatus::Static;
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
