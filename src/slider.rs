use crate::*;

#[derive(Default, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct SliderPoints {
    points: usize,
}
impl Display for SliderPoints{
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
pub fn points() -> html {
    let (points, _) = use_store::<SliderPoints>();
    html! {
        <div>
            {format!("({} points)", points)} <br/>
        </div>
    }
}

use std::marker::PhantomData;

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Slider<T: IsUpgrade> {
    // action: SliderType,
    pub current: usize,
    max: usize,
    text: String,
    pub t: PhantomData<*const T>,
}
impl<T: IsUpgrade> Clone for Slider<T> {
    fn clone(&self) -> Self {
        Self {
            t: self.t,
            text: self.text.clone(),
            ..*self
        }
    }
}
impl<T: IsUpgrade> Slider<T> {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Slider {
            current: 0,
            max: 0,
            t: PhantomData,
            text: text.into(),
        }
    }
    pub fn _upgrade(&mut self) {
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
impl<T: IsUpgrade> Store for Slider<T> {
    fn new() -> Self {
        Default::default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        true
    }
}
impl<T: IsUpgrade> Default for Slider<T> {
    fn default() -> Self {
        Slider::<T>::new(format!("Unimplemented {}", std::any::type_name::<T>()))
    }
}

#[function_component(Slide)]
pub fn slide<T: IsUpgrade>() -> html {
    let (slider, dispatch) = use_store::<Slider<T>>();
    let (upgrade, _) = use_store::<Upgrade<T>>();
    if upgrade.level != slider.max {
        dispatch.reduce_mut(|s| s.max = upgrade.level);
    }
    // info!("Slider");
    if slider.max == 0 {
        return html! {<div> </div>};
    }
    // let max = slider.max.to_string();
    // let value = slider.current.to_string();
    let decrease = Callback::from(|_| {
        Dispatch::<Slider<T>>::new().reduce_mut(|s| s.decrease());
    });
    let increase = Callback::from(|_| {
        Dispatch::<Slider<T>>::new().reduce_mut(|s| s.increase());
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
        // <input type="range" min=0 {max} {value} class="slider"/>
        {slider.current}
        {inc_button}
        // <button {class} onclick={increase}>{" + "}</button>
    </p >
    }
}
