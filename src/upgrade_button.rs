use yew::prelude::*;

use crate::*;

// macro_rules! watch {
//     ($a:ident) => {{
//         match $a {
//             Condition::HavePoints(usize) => use_store::<Points>(),
//             Condition::AvgPoints(usize) => use_store::<Stats>(),
//             _ => unimplemented!()
//         }
//     }};
// }

#[function_component(UpgradeButton)]
pub fn upgrade_button(props: &Props) -> html {
    let upgrade = &props.upgrade;
    // info!("Rerender"); 
    let _ = use_store::<Upgrades>();
    let _ = use_store::<Stats>();
    let _ = upgrade.show.watch();
    let _ = upgrade.cost.watch();

    if !upgrade.visible() || upgrade.done.get() {
        return html! { <div> </div> }; // cause yew bug
    }

    // let (points, _) = use_store::<Points>();
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
    let text = format!("{}", upgrade.action.text());
    let title = format!("{}", upgrade.cost);
    let upgrade = upgrade.clone();
    let f = if upgrade.clickable() {
        style.push("bg-green-400");
        Callback::from(move |_| {
            upgrade.run();
            // upgrades.upgrades[index].run();
            // Dispatch::<Upgrades>::new().get() // get new state (only required for reset :(
        })
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

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub upgrade: Rc<Upgrade>,
}
