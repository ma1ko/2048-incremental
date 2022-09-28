use std::borrow::BorrowMut;

use yew::prelude::*;

use crate::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub upgrade_type: UpgradeType,
}

#[function_component(UpgradeButton)]
pub fn upgrade_button(props: &Props) -> Html {
    // info!("I am {:?}", props.upgrade_type);
    let upgrade_type = props.upgrade_type;
    let upgrade = use_selector(move |upgrades: &Upgrades| upgrades[&upgrade_type].clone());
    let upgrade = upgrade.as_ref().borrow();
    let _ = use_store_(&upgrade.show);
    let _ = use_store_(&upgrade.cost);
    let onclick = Callback::from(move |_: MouseEvent| {
        Dispatch::<Upgrades>::new().reduce(|u| {
            u[&upgrade_type].borrow_mut().upgrade();
            u[&upgrade_type].borrow().run();
            u
        });
    });
    if !upgrade.visible() || upgrade.done {
        return html! { <div> </div> }; // cause yew bug
    }

    let mut style = classes!(
        "text-gray-800",
        "font-semibold",
        "py-2",
        "px-4",
        "border",
        "border-gray-400",
        "rounded",
        "shadow",
        "has-tooltip"
    );
    let text = format!("{}", upgrade.text);
    let title = format!("{}", upgrade.cost);
    let upgrade = upgrade.clone();
    let f = if upgrade.clickable() {
        style.push("bg-green-400");
        onclick
    } else {
        style.push("bg-white");
        Callback::noop()
    };

    html! {
        <div class="has-tooltip">
            <span class="tooltip rounded shadow-lg bg-sky-100 test-red-500 -mt-8 text-xl">{title}</span>
          <button class={style} onclick={f}>{text}</button>

        </div>

    }
}

