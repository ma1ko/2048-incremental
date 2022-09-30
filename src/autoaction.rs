use crate::*;

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
    // return;
    info!("Saving");

    save!(Upgrades);
    save!(Points);
    save!(UpgradeableBoard);
    save!(AutoActions);
    save!(Stats);
    save!(Sliders);
    save!(SliderPoints);
    // save!(Shuffles);
    // save!(Bonus);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AutoActions {
    actions: HashMap<UpgradeType, Mrc<AutoAction>>,
}
impl Store for AutoActions {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}
impl Default for AutoActions {
    fn default() -> Self {
        Self {
            actions: [
                (AutoMove, AutoAction::new(AutoMove, 1000, false)),
                (RandomPlace, AutoAction::new(RandomPlace, 1000, false)),
                (AutoShuffle, AutoAction::new(AutoShuffle, 10000, false)),
                (AutoSave, AutoAction::new(AutoSave, 10000, false)),
            ]
            .into(),
        }
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
    current: usize,
    max: usize,
    active: bool,
    initial: usize,
    t: UpgradeType,
}

impl AutoAction {
    pub fn new(t: UpgradeType, time: usize, active: bool) -> Mrc<Self> {
        let me = Self {
            max: time,
            current: time,
            active,
            t,
            initial: time,
        };
        Mrc::new(me)
    }
    fn upgrade(&mut self, time: usize, level: usize) {
        self.max = time;
    }
    fn enable(&mut self, level: usize) {
        self.active = true;
    }
    fn disable(&mut self) {
        self.active = false;
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
        html! {<>
            <DoAutoAction {action_type}/>
            <ShowCountdown {action_type}/>
        </>}
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
        action.disable();
    } else {
        action.active = true;
        action.max = (action.initial as f64 / (1.2f64).powf(slider.current as f64)) as usize;
    }

    html! {}
}

#[function_component(ShowCountdown)]
pub fn countdown(props: &Props) -> Html {
    let t = props.action_type;
    let _ = use_store::<Timer>();
    let dispatch = Dispatch::<AutoActions>::new().get();
    let mut action = dispatch[&t].borrow_mut();
    if !action.active {
        return html! {};
    }
    if action.current <= 100 {
        action.current = action.max;
        // create a callback to avoid borrowing issues

            Dispatch::<Sliders>::new().get().run(&t);
    } else {
        action.current -= 100;
    }
    html! {
        <p> {format!("{:?}:", action.t)} {action.current} {"/"} {action.max} </p>
    }
    // }
}
