#![macro_use]

use crate::*;

// macro_rules! upgrade{
//     ($name:ident, $text:expr, $cost:expr, $show:expr,$type:expr ) => {
//         pub struct $name{}
//         impl IsUpgrade for $name{
//             fn text() -> &'static str {
//                 $text
//             }
//             fn cost() -> Condition {
//                 $cost.into()
//             }
//             fn show() -> Condition {
//                 $show.into()
//             }
//         }
//    };
// }
fn register<S: Store + DeserializeOwned>(store: S) {
// fn register<S: Store >(store: S) {
    let load = storage::load(storage::Area::Local)
        .expect("Unable to load state")
        .unwrap_or(store);
    Dispatch::<S>::new().set(load);
}
pub fn register_upgrades() {
    info!("Registering");
    // static
    register(Upgrade::<Harvest>::new(Free(), Free(), "Harvest").static_());
    register(Upgrade::<Place>::new(Free(), Free(), "Place a 2").static_());
    register(Upgrade::<Reset>::new(Free(), Free(), "HARD RESET").static_());
    register(Upgrade::<Shuffle>::new(
        AlltimePoints(100),
        AlltimePoints(20),
        "Shuffle board",
    )
    .static_());
    // upgrades
    register(
        Upgrade::<ExtendX>::new(AvgPoints(4), AvgPoints(2), "Extend Horizontally")
            .multiply(4)
            .max_level(6),
    );
    register(
        Upgrade::<ExtendY>::new(PointsOnBoard(256), PointsOnBoard(128), "Extend Vertically")
            .multiply(4)
            .max_level(6),
    );
    register(Upgrade::<Automove>::new(16, Free(), "Auto move the board").multiply(4));
    register(Upgrade::<Autoplace>::new(16, Free(), "Auto place a number").multiply(4));
    register(Upgrade::<Stats>::new(64, 32, "Enable Statistcs"));
    register(Upgrade::<BonusTile>::new(
        AvgPoints(8),
        AvgPoints(4),
        "Bonus 16 tile",
    ));
    register(Upgrade::<BonusPoints>::new(
        Harvested(1024),
        Harvested(246),
        "Get extra points for Harvesting",
    ));
    register(Upgrade::<ScientificNotation>::new(
        AlltimePoints(1000),
        AlltimePoints(500),
        "Show large numbers in scientific notation",
    ));

    register(Upgrade::<AutoShuffle>::new(
        AlltimePoints(100),
        AlltimePoints(20),
        "Automatic Shuffling",
    ));
    register(
        Upgrade::<SliderPoint>::new(NumberOnBoard(8), NumberOnBoard(4), "Get a Slider point")
            .multiply(2),
    );
    // sliders
    register(Slider::<ExtendX>::new("Extend horizontally"));
    register(Slider::<ExtendY>::new("Extend vertically"));
    register(Slider::<Automove>::new("Board move speed"));
    register(Slider::<Autoplace>::new("Value of auto-placed number"));
    register(Slider::<AutoShuffle>::new("Automatic shuffling"));

    //Automovers
    register(AutoAction::<Automove>::new(1000, false));
    register(AutoAction::<Autoplace>::new(1000, false));
    register(AutoAction::<Autosave>::new(10000, true));
    register(AutoAction::<AutoShuffle>::new(30_000, false));
}

use condition::Condition::*;
use serde::de::DeserializeOwned;
pub trait IsUpgrade: 'static {
    fn run(level: usize) {}
}


pub struct Place {}
impl IsUpgrade for Place {
    fn run(level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.random_place(2);
            b
        });
    }
}
pub struct Mark<T> {
    t: PhantomData<*const T>
}
impl<T> Clone for Mark<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl<T> Default for Mark<T> {
    fn default() -> Self {
        Self {t: PhantomData}
    }
}

pub struct ExtendY {}
impl IsUpgrade for ExtendY {}
pub struct ExtendX {}
impl IsUpgrade for ExtendX {}
pub struct Harvest {}
impl IsUpgrade for Harvest {
    fn run(level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.harvest();
            b
        });
    }
}
pub struct BonusTile {}
impl IsUpgrade for BonusTile {
    fn run(_: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|board| {
            board.set_combine_fn(CombineFn::Bonus(6, 4));
            board
        });
    }
}
pub struct BonusPoints {}
impl IsUpgrade for BonusPoints {
    fn run(_: usize) {
        Dispatch::<Points>::new().reduce_mut(|points| points.set_log());
    }
}
pub struct ScientificNotation {}
impl IsUpgrade for ScientificNotation {
    fn run(_: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|board| {
            board.scientific_notation();
            board
        });
    }
}
pub struct SliderPoint {}
impl IsUpgrade for SliderPoint {
    fn run(_: usize) {
        Dispatch::<SliderPoints>::new().reduce_mut(|points| points.add(1));
    }
}

pub struct Reset {}
impl IsUpgrade for Reset {}

// AutoActions
pub struct Automove {}
impl IsUpgrade for Automove {
    fn run(_level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.mv();
            b
        });
    }
}

pub struct Autosave {}
impl IsUpgrade for Autosave {
    fn run(_level: usize) {
        do_save()
    }
}
pub struct Autoplace {}
impl IsUpgrade for Autoplace {
    fn run(level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.random_place(1 * 2usize.pow(level as u32));
            b
        });
    }
}
// pub struct Shuffle {}
// impl IsUpgrade for Shuffle {}

pub struct Shuffle {}
impl IsUpgrade for Shuffle {
    fn run(_level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.shuffle();
            b
        });
    }
}
pub struct AutoShuffle {}
impl IsUpgrade for AutoShuffle {
    fn run(_level: usize) {
        Dispatch::<UpgradeableBoard>::new().reduce(|b| {
            b.shuffle();
            b
        });
    }
}
