use std::marker::PhantomData;

use crate::*;

use gloo::timers::callback::Interval;
use yewdux::storage::Area::Local;
macro_rules! save {
    ($a:ident) => {
        let dispatch = Dispatch::<$a>::new();
        storage::save(dispatch.get().as_ref(), Local).unwrap();
    };
}
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

pub fn do_save() {
    return;
    info!("Saving");

    // save!(Upgrades);
    // save!(Points);
    // save!(UpgradeableBoard);
    // save!(AutoActions);
    // save!(Stats);
    // save!(Shuffles);
    // save!(Bonus);
}
// #[derive(Serialize, Deserialize, PartialEq)]
// pub struct AutoActions {
//     automove: RefCell<AutoAction>,
//     autoharvest: RefCell<AutoAction>,
//     random_place: RefCell<AutoAction>,
//     autosave: RefCell<AutoAction>,
//     autoshuffle: RefCell<AutoAction>,
// }
// impl AutoActions {
//     pub fn upgrade_automove(&self, time: usize) {
//         self.automove.borrow_mut().upgrade(time);
//     }
//     pub fn enable_automove(&self) {
//         self.automove.borrow_mut().enable();
//     }
//     pub fn upgrade_autoshuffle(&self, time: usize) {
//         self.autoshuffle.borrow_mut().upgrade(time);
//     }
//     pub fn enable_autoshuffle(&self) {
//         self.autoshuffle.borrow_mut().enable();
//     }
//     pub fn upgrade_random_place(&self, time: usize) {
//         self.random_place.borrow_mut().upgrade(time);
//     }
//     pub fn enable_random_place(&self) {
//         self.random_place.borrow_mut().enable();
//     }
//     pub fn upgrade_autoharvest(&self, time: usize) {
//         self.random_place.borrow_mut().upgrade(time);
//     }
//     pub fn enable_autoharvest(&self) {
//         self.autoharvest.borrow_mut().enable();
//     }
// }

// impl Default for AutoActions {
//     fn default() -> Self {
//         AutoActions {
//             automove: RefCell::new(AutoAction::new(None, Action::AutoMove, 1000, false)),
//             autoharvest: RefCell::new(AutoAction::new(None, Action::AutoHarvest, 10000, false)),
//             random_place: RefCell::new(AutoAction::new(None, Action::RandomPlace, 1000, false)),
//             autosave: RefCell::new(AutoAction::new(None, Action::AutoSave, 5000, true)),
//             autoshuffle: RefCell::new(AutoAction::new(None, Action::AutoShuffle, 10000, false)),
//         }
//     }
// }

// impl Store for AutoActions {
//     fn should_notify(&self, old: &Self) -> bool {
//         self != old
//     }
//     fn new() -> Self {
//         let me: Self = storage::load(storage::Area::Local)
//             .expect("Unable to load state")
//             .unwrap_or_default();
//         me.autoshuffle.borrow_mut().set_callback();
//         me.autosave.borrow_mut().set_callback();
//         me.autoharvest.borrow_mut().set_callback();
//         me.automove.borrow_mut().set_callback();
//         me.random_place.borrow_mut().set_callback();
//         me
//     }
// }
// #[derive(Clone, Serialize, Deserialize, PartialEq)]
// enum Action {
//     AutoMove,
//     RandomPlace,
//     AutoSave,
//     AutoHarvest,
//     AutoShuffle,
// }
#[derive(Serialize, Deserialize)]
pub struct AutoAction<T: IsUpgrade> {
    #[serde(skip)]
    interval: Option<Interval>,
    // action: Action,
    time: usize,
    active: bool,
    t: PhantomData<*const T>,
}

impl<T: IsUpgrade> Store for AutoAction<T> {
    fn new() -> Self {
        Default::default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}
impl<T: IsUpgrade> Default for AutoAction<T> {
    fn default() -> Self {
        AutoAction::<T>::new(1000, false)
    }
}
impl<T: IsUpgrade> Clone for AutoAction<T> {
    fn clone(&self) -> Self {
        Self {
            interval: None,
            time: self.time,
            t: self.t,
            active: self.active,
        }
    }
}

impl<T: IsUpgrade> AutoAction<T> {
    pub fn new(time: usize, active: bool) -> Self {
        let mut me = Self {
            interval: None,
            // action: f,
            time,
            active,
            t: PhantomData,
        };
        me.set_callback(0);
        me
    }
    fn upgrade(&mut self, time: usize, level: usize) {
        self.time = time;
        self.set_callback(level);
    }
    fn enable(&mut self, level: usize) {
        self.active = true;
        self.set_callback(level);
    }
    fn disable(&mut self) {
        self.active = false;
        self.interval = None;
    }
    fn set_callback(&mut self, level: usize) {
        if !self.active {
            return;
        }

        // let action = self.action.clone();
        let cb = Callback::from(move |_| {
            T::run(level);
        });
        self.interval = Some(Interval::new(self.time as u32, move || cb.emit(())));
    }
}
#[function_component(DoAutoAction)]
pub fn do_auto_action<T: IsUpgrade>() -> html {
    let (slider, _) = use_store::<Slider<T>>();
    let dispatch = Dispatch::<AutoAction<T>>::new();

    if !slider.enabled() {
        return html! {};
    }
    if slider.current == 0 {
        dispatch.reduce_mut(|auto| {
            auto.disable();
        });
    } else {
        dispatch.reduce_mut(|auto| {
            auto.active = true;
            auto.time = 1000 - slider.current;
            auto.set_callback(slider.current);
        });
    }

    html! {}
}

#[derive(Clone)]
struct Countdown<T: IsUpgrade> {
    current: usize,
    max: usize,
    t: PhantomData<T>,
}
impl<T: IsUpgrade> Countdown<T> {
    fn reset(&mut self) {
        self.current = self.max;
    }
    fn step(&mut self) -> usize {
        if self.current == 0 {
            return 0;
        }
        self.current -= 10;
        self.current
    }
}


impl<T: IsUpgrade> Store for Countdown<T> {
    fn new() -> Self {
        Countdown {
            current: 0,
            max:0,
            t: PhantomData

        }
        // Default::default()


    }
    fn should_notify(&self, old: &Self) -> bool {
        true
    }
}

#[function_component(ShowCountdown)]
pub fn countdown<T: IsUpgrade>() -> html {
    let (action, _) = use_store::<AutoAction<T>>();
    let (countdown, dp) = use_store::<Countdown<T>>();
    // let time = dp.reduce_mut(|c| c.step());
    html! {}
}
