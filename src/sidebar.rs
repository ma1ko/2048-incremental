use yew::prelude::*;
use yewdux::prelude::*;

// use crate::upgrade_button::Button;
use crate::upgrade::*;



use crate::upgrade_button::*;

#[function_component(ShowPoints)]
pub fn points() -> html {
    let (points, _) = use_store::<Points>();

    html! {
        <div>
            {format!("You have {} points", points)} <br/>
        </div>
    }
}
use crate::stats::*;

#[function_component(SideBar)]
pub fn bar() -> html {
    let (upgrades, _) = use_store::<Upgrades>();
    let (_, _) = use_store::<Points>();
    let buttons: Html = upgrades
        .upgrades
        .iter()
        .enumerate()
        .map(|(index, _upgrade)| {
            html! {
                <> <UpgradeButton {index}/> <br/> </>
            }
        })
        .collect();

    html! {
        <div class={classes!("float-right", "w-2/6")} >
        <ShowPoints/>
        {buttons}
        <Statistics/>
        </div>
    }
}
