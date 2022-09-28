use std::ops::Index;

use crate::*;

#[derive(Default, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct SliderPoints {
    points: usize,
}
impl Display for SliderPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points)
    }
}
impl SliderPoints {
    pub fn sub(&mut self, points: usize) {
        self.points -= points;
    }
    pub fn add(&mut self, points: usize) {
        self.points += points;
    }
    pub fn get(&self) -> usize {
        self.points
    }
}
impl Store for SliderPoints {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}
#[function_component(ShowSliderPoints)]
pub fn points() -> Html {
    let (points, _) = use_store::<SliderPoints>();
    html! {
        <div>
            {format!("({} points)", points)} <br/>
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sliders {
    sliders: HashMap<UpgradeType, Mrc<Slider>>,
}
impl Store for Sliders {
    fn new() -> Self {
        Self {
            sliders: [
                (ExtendX, Slider::new(ExtendX, "Extend X")),
                (ExtendY, Slider::new(ExtendY, "Extend Y")),
                (AutoMove, Slider::new(AutoMove, "Automove")),
                (RandomPlace, Slider::new(RandomPlace, "Auto place number")),
            ]
            .into(),
        }
    }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

impl Sliders {
    fn get(&self, t: &UpgradeType) -> &Mrc<Slider> {
        let slider = self.sliders.get(t).expect(&format!("Slider {:?} doesn't exist", t));
        slider
    }
}
impl Index<&UpgradeType> for Sliders {
    type Output = Mrc<Slider>;
    fn index(&self, index: &UpgradeType) -> &Self::Output {
        self.get(index)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Slider {
    // action: SliderType,
    pub current: usize,
    max: usize,
    text: String,
    t: UpgradeType,
}

impl Slider {
    pub fn new<S: Into<String>>(t: UpgradeType, text: S) -> Mrc<Self> {
        Mrc::new(Slider {
            current: 0,
            max: 0,
            text: text.into(),
            t,
        })
    }
    pub fn upgrade(&mut self) {
        // info!("Increasing");
        self.max += 1;
    }
    pub fn enabled(&self) -> bool {
        self.max > 0
    }
    pub fn increase(&mut self) {
        if self.current == self.max {
            return;
        } else {
            self.current += 1;
        }
    }
    pub fn decrease(&mut self) {
        if self.current == 0 {
            return;
        } else {
            self.current -= 1;
        }
    }
}
#[derive(Properties, PartialEq)]
pub struct Props {
    slider_type: UpgradeType,
}

#[function_component(ShowSliders)]
pub fn slide() -> Html {
    let dispatch = Dispatch::<Sliders>::new().get();

    let html = dispatch.sliders.keys().cloned().map(|slider_type| {
        html! {<ShowSlider {slider_type}/>}
    });

    html.collect()
}
#[function_component(ShowSlider)]
pub fn show_slider(props: &Props) -> Html {
    let slider_type = props.slider_type;
    let slider = use_selector(move |sliders: &Sliders| sliders[&slider_type].clone());
    let mut slider = slider.as_ref().borrow_mut();
    let upgrade = use_selector(move |u: &Upgrades| u[&slider_type].clone());
    let upgrade = upgrade.as_ref().borrow();

    if upgrade.level != slider.max {
        slider.max = upgrade.level;
    }
    if slider.max == 0 {
        return html! {<div> </div>};
    }
    let decrease = Callback::from(move |_| {
        Dispatch::<Sliders>::new().reduce(|s| {
            s[&slider_type].borrow_mut().decrease();
            s
        });
    });
    let increase = Callback::from(move |_| {
        Dispatch::<Sliders>::new().reduce(|s| {
            s[&slider_type].borrow_mut().increase();
            s
        });
    });
    let mut class = classes!(
        "text-gray-800",
        "font-semibold",
        "py-1",
        "px-4",
        "border",
        "border-gray-400",
        "rounded",
        "shadow",
        "has-tooltip"
    );
    let dec_button = html! {
        <button class={class.clone()} onclick={decrease}>{"-"} </button>
    };
    if slider.current < slider.max {
        class.push("bg-green-400");
    }
    let inc_button = html! {
        <button class={class.clone()} onclick={increase}>{"+"} </button>
    };

    html! {
    <p>
        <p> {format!("{}", {slider.text.clone()})} </p>
        {dec_button}
        {slider.current}
        {inc_button}
    </p >
    }
}
