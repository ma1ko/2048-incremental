use yew::prelude::*;
use yewdux::prelude::use_store;
use yewdux::prelude::Dispatch;

use crate::upgrade::*;

#[function_component(UpgradeButton)]
pub fn upgrade_button(props: &Props) -> html {
    let (points, _) = use_store::<Points>();
    let style = classes!(
        "bg-white",
        "text-gray-800",
        "font-semibold",
        "py-2",
        "px-4",
        "border",
        "border-gray-400",
        "rounded",
        "shadow"
    );
    let upgrade = props.upgrade.clone();
    let f = Callback::once(move |_| upgrade.run());
    let upgrade = props.upgrade.clone();

    let text = format!("{} (Cost: {})", upgrade.text, upgrade.cost.get());

    if upgrade.visible(points.points) && upgrade.clickable(points.points) {
        html! { <button class={style} onclick={f}>{text}</button> }
    } else if upgrade.visible(points.points) {
        html! { <button class={style}> {text}</button> }
    } else {
        html! {}
    }
}

use std::rc::Rc;
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub upgrade: Rc<Upgrade>,
}
