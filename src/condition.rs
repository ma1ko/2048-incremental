use crate::*;


#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Eq)]
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
    // Until(Action),
    Between(Box<Condition>, Box<Condition>),
    NumberOnBoard(usize),
}
pub use Condition::*;
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

// conditional subscribe to updates (very hacky...)
#[hook]
pub fn use_store_(c: &Condition) {
    if c.equals(&HavePoints(0)) && (use_store::<Points>().0.get() == 0) {}
    if c.equals(&PointsOnBoard(0)) && (use_store::<UpgradeableBoard>().0.type_id() == 0.type_id()) {
    }
    if c.equals(&Harvested(0)) && (use_store::<Stats>().0.points == 0) {}
    if c.equals(&AlltimePoints(0)) && (use_store::<Stats>().0.points == 0) {}
    if c.equals(&AvgPoints(0)) && (use_store::<Avg>().type_id() == 0.type_id()) {}
    if c.equals(&UpgradeDone(Place)) && (use_store::<Upgrades>().type_id() == 0.type_id()) {}
    if c.equals(&NumberOnBoard(0)) && (use_store::<UpgradeableBoard>().type_id() == 0.type_id()) {}
}
impl Condition {
    pub fn equals(&self, c: &Condition) -> bool {
        match (self, c) {
            (HavePoints(_), HavePoints(_)) => true,
            (PointsOnBoard(_), PointsOnBoard(_)) => true,
            (Free(), Free()) => true,
            (Harvested(_), Harvested(_)) => true,
            (AlltimePoints(_), AlltimePoints(_)) => true,
            (AvgPoints(_), AvgPoints(_)) => true,
            (UpgradeDone(_), UpgradeDone(_)) => true,
            (NumberOnBoard(_), NumberOnBoard(_)) => true,
            _ => false,
        }
    }
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
            // Until(upgrade) => !get::<Upgrades>().is_clickable(*upgrade),
            Between(a, b) => a.check() && !b.check(),
            Free() => true,
            NumberOnBoard(i) => get::<UpgradeableBoard>().contains(*i),
        }
    }
    pub fn fulfilled(&self) {
        match self {
            HavePoints(p) => Dispatch::<Points>::new().reduce_mut(|points| points.sub(*p)),
            HaveShuffles(p) => Dispatch::<Shuffles>::new().reduce_mut(|s| s.sub(*p as f64)),
            NumberOnBoard(i) => {
                Dispatch::<UpgradeableBoard>::new().reduce(|board| { board.harvest_number(*i); board});
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
            UpgradeDone(_upgrade) => write!(f, "Requires other upgrade first"),
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
