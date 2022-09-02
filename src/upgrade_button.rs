use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::upgrade::*;

#[function_component(UpgradeButton)]
pub fn upgrade_button(props: &Props) -> html {
    let index = props.index;
    let (points, _) = use_store::<Points>();
    let (upgrades, dispatch) = use_store::<Upgrades>(); // for reset
    let mut style = classes!(
        "text-gray-800",
        "font-semibold",
        "py-2",
        "px-4",
        "border",
        "border-gray-400",
        "rounded",
        "shadow"
    );
    let upgrade = upgrades.upgrades.get(props.index).unwrap();
    let f = dispatch.reduce_callback(move |upgrades| {
        upgrades.upgrades[index].run();
        Dispatch::<Upgrades>::new().get() // get new state (only required for reset :(
    });

    let text = format!("{} (Cost: {})", upgrade.text, upgrade.cost.get());

    if upgrade.visible(points.points) && upgrade.clickable(points.points) {
        style.push("bg-green-400");
        html! { <button class={style} onclick={f}>{text}</button> }
    } else if upgrade.visible(points.points) {
        style.push("bg-white");
        html! { <button class={style}> {text}</button> }
    } else {
        html! {}
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub index: usize,
}
