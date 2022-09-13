use yew::prelude::*;
use yewdux::prelude::*;

// use crate::upgrade_button::Button;
use crate::*;

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
    // let _ = use_store::<Stats>();
    // let _ = use_store::<UpgradeableBoard>();
    // let _ = use_store::<Points>();
    let statics = upgrades
        .statics()
        .map(|upgrade| {
            html! {
                <> <UpgradeButton {upgrade}/> </>
            }
        });
    let onetime = upgrades
        .onetimes()
        .map(|upgrade| {
            html! {
                <> <UpgradeButton {upgrade}/> </>
            }
        });

    html! {
        <div classes={classes!("float-right", "w-2/6")}>
            
            <ShowPoints/> <br/>
            <div class={classes!("float-left", "w-1/6", "grid-cols-1", "grid-rows-6", "h-1/2")} >
                <p> {"Actions (Move the Board with WASD):"} </p>
                {statics.collect::<Html>()}
                <br/>
                <Statistics/>
            </div>
            <div class={classes!("float-right", "w-1/6", "grid-cols-1", "grid-rows-6", "h-1/2")} >
                <p> {"Upgrades"} </p>
                {onetime.collect::<Html>()}
            </div>
        </div>
    }
}
