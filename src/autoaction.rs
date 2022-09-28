
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
//
#[derive(Serialize, Deserialize, Clone)]
pub struct AutoActions {
    actions: HashMap<UpgradeType, Mrc<AutoAction>>,
}
impl Store for AutoActions {
    fn new() -> Self {
        Self {
            actions: [
                (AutoMove, AutoAction::new(AutoMove, 1000, false)),
                (RandomPlace, AutoAction::new(RandomPlace, 1000, false)),
                (AutoSave, AutoAction::new(AutoSave, 1000, true)),
            ]
            .into(),
        }
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}
impl AutoActions {
    fn get(&self, t: &UpgradeType) -> &Mrc<AutoAction> {
        let action = self.actions.get(t).unwrap();
        action
    }
}
impl Index<&UpgradeType> for AutoActions {
    type Output = Mrc<AutoAction>;
    fn index(&self, index: &UpgradeType) -> &Self::Output {
        self.get(index)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AutoAction {
    #[serde(skip)]
    interval: Option<Interval>,
    // action: Action,
    time: usize,
    active: bool,
    t: UpgradeType,
}

// impl Default for AutoAction {
//     fn default() -> Self {
//         AutoAction::<T>::new(1000, false)
//     }
// }
// impl<T: IsUpgrade> Clone for AutoAction<T> {
//     fn clone(&self) -> Self {
//         Self {
//             interval: None,
//             time: self.time,
//             t: self.t,
//             active: self.active,
//         }
//     }
// }

impl AutoAction {
    pub fn new(t: UpgradeType, time: usize, active: bool) -> Mrc<Self> {
        let mut me = Self {
            interval: None,
            // action: f,
            time,
            active,
            t,
        };
        me.set_callback(0);
        Mrc::new(me)
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

        let t = self.t;
        let cb = Callback::from(move |_| {
            // Dispatch::<Upgrades>::new().get().run(t, );
            t.run(level);
        });
        self.interval = Some(Interval::new(self.time as u32, move || cb.emit(())));
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    action_type: UpgradeType,
}
#[function_component(ShowAutoActions)]
pub fn show_auto_actions() -> Html {
    let dispatch = Dispatch::<AutoActions>::new().get();

    let html = dispatch.actions.keys().cloned().map(|action_type| {
        if action_type == AutoSave {
            html! {}
        } else {
            html! {<DoAutoAction {action_type}/>}
        }
    });

    html.collect()
}

#[function_component(DoAutoAction)]
pub fn do_auto_action(props: &Props) -> Html {
    let action_type = props.action_type.clone();
    let action = use_selector(move |actions: &AutoActions| actions[&action_type].clone());
    // Slider Controlling us
    let slider = use_selector(move |sliders: &Sliders| sliders[&action_type].clone());
    let slider = slider.as_ref().borrow_mut();
    let mut action = action.as_ref().borrow_mut();

    if !slider.enabled() {
        return html! {};
    }
    if slider.current == 0 {
        // info!("Disable {:?}, ", action_type);
        action.disable();
    } else {
        // info!("Setting {:?}, to {}", action_type, slider.current);
        action.active = true;
        action.time = 1000 - slider.current;
        action.set_callback(slider.current);
    }

    html! {}
}

/*
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
pub fn countdown<T: IsUpgrade>() -> Html {
    let (action, _) = use_store::<AutoAction<T>>();
    let (countdown, dp) = use_store::<Countdown<T>>();
    // let time = dp.reduce_mut(|c| c.step());
    html! {}
}
*/
