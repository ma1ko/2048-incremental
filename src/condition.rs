use crate::*;

// impl From<Action> for Condition {
//     fn from(u: Action) -> Self {
//         UpgradeDone(u)
//     }
// }
// impl From<UpgradeType> for Box<Condition> {
//     fn from(u: UpgradeType) -> Self {
//         Box::new(UpgradeDone(u))
//     }
// }
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug,Eq)]
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
    // UpgradeDone(Action),
    // Until(Action),
    Between(Box<Condition>, Box<Condition>),
    NumberOnBoard(usize),
}
use Condition::*;
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

fn get<S: Store>() -> Rc<S> {
    Dispatch::<S>::new().get()
}
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
            // UpgradeDone(upgrade) => get::<Upgrades>().is_done(*upgrade),
            // Until(upgrade) => !get::<Upgrades>().is_clickable(*upgrade),
            Between(a, b) => a.check() && !b.check(),
            Free() => true,
            NumberOnBoard(i) => get::<UpgradeableBoard>().contains(*i)
        }
    }
    pub fn watch(&self) -> Box<dyn Any> {
        use Condition::*;
        match self {
            NumberOnBoard(_) => Box::new(use_store::<UpgradeableBoard>().1),
            AvgPoints(_) => Box::new(use_store::<Avg>().1),
            HavePoints(_) => Box::new(use_store::<Points>().1),
            HaveShuffles(_) => Box::new(use_store::<Shuffles>().1),
            PointsOnBoard(_) => Box::new(use_store::<UpgradeableBoard>().1),
            AlltimePoints(_) => Box::new(use_store::<Stats>().1),
            // UpgradeDone(_) => Box::new(use_store::<Upgrades>().1),
            // UpgradeDone(_) => Box::new(use_store::<Nothing>().1),
            // Until(_) => Box::new(use_store::<Upgrades>().1),
            // Until(_) => Box::new(use_store::<Nothing>().1),
            BoardSize(_, _) => Box::new(use_store::<UpgradeableBoard>()),
            Free() => Box::new(Dispatch::<Nothing>::new()),
            Harvested(_) => Box::new(use_store::<Stats>().1),
            Between(a, b) => Box::new((a.watch(), b.watch())),
        }
    }
    pub fn fulfilled(&self) {
        match self {
            HavePoints(p) => Dispatch::<Points>::new().reduce_mut(|points| points.sub(*p)),
            HaveShuffles(p) => Dispatch::<Shuffles>::new().reduce_mut(|s| s.sub(*p as f64)),
            NumberOnBoard(i) => {
                Dispatch::<UpgradeableBoard>::new().get().harvest_number(*i);
                // Dispatch::<SliderPoints>::new().reduce_mut(|p| p.add(1))

            }
            // AlltimePoints(_) => {}
            // Harvested(_) => {}
            // BoardSize(_, _) => {}
            // Free() => {}
            _ => {}
        }
    }
    pub fn multiply(&self, i: usize) -> Self {
        match self {
            HavePoints(p) => HavePoints(p * i),
            PointsOnBoard(p) => PointsOnBoard(p * i),
            AvgPoints(p) => AvgPoints(p * i),
            AlltimePoints(p) => AlltimePoints(p * i),
            Harvested(p) => Harvested(p * i),
            Free() => Free(),
            NumberOnBoard(i) => NumberOnBoard(i * 2),
            // UpgradeDone(i) => UpgradeDone(*i),

            _ => panic!("Didn't double {:?}", self),
        }
    }
}
impl std::fmt::Display for Condition {
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
            // UpgradeDone(upgrade) => write!(f, "Requires other upgrade first"),
            // Until(_) => write!(f, ""),
            Between(a, _) => write!(f, "{}", a),
            NumberOnBoard(i) => write!(f, "Harvest a {}", i),
        }
    }
}
impl From<usize> for Condition {
    fn from(points: usize) -> Self {
        Condition::HavePoints(points)
    }
}
