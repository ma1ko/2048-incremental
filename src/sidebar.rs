use yew::prelude::*;
use yewdux::prelude::*;

// use crate::upgrade_button::Button;
use crate::*;

#[function_component(ShowPoints)]
pub fn points() -> Html {
    let (points, _) = use_store::<Points>();

    html! {
        <div>
            {format!("You have {} points", points)} <br/>
        </div>
    }
}
use crate::stats::*;

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    // return html! {};

    let automoves = html! { <>

        /*
            <DoAutoAction<Automove>/>
            <DoAutoAction<Autosave>/>
            <DoAutoAction<Autoplace>/>
            */
            </>
    };
    let upgrades = Dispatch::<Upgrades>::new().get();
    let statics = upgrades.statics().map(|upgrade| {
        let upgrade_type = upgrade.borrow().t;
        html! {<UpgradeButton {upgrade_type}/>}

    });
    let upgrades = upgrades.onetime().map(|upgrade| {
        let upgrade_type = upgrade.borrow().t;
        html! {<UpgradeButton {upgrade_type}/>}

    });
    /*
    let statics = html! { <>
                <UpgradeButton<Place>/>
                <UpgradeButton<Harvest>/>
                <UpgradeButton<Reset>/>
                <UpgradeButton<Shuffle>/>
                </>
    };
    let upgrades = html! { <>
                <UpgradeButton<ExtendX>/>
                <UpgradeButton<ExtendY>/>
                <UpgradeButton<Automove>/>
                <UpgradeButton<Autoplace>/>
                <UpgradeButton<Stats>/>
                <UpgradeButton<BonusTile>/>
                <UpgradeButton<BonusPoints>/>
                <UpgradeButton<ScientificNotation>/>
                <UpgradeButton<SliderPoint>/>
                // <UpgradeButton<Shuffle>/>
                <UpgradeButton<AutoShuffle>/>
                </>
    };
    */
    let sliders = html! { <>
                <ShowSliders/>
                // <Slide<ExtendX>/>
                // <Slide<ExtendY>/>
                // <Slide<Automove>/>
                // <Slide<Autoplace>/> 
                // <Slide<AutoShuffle>/> 
                </>
    };

    html! {
        <div classes={classes!("float-right", "w-2/6")}>
            <ShowAutoActions/>
            <ShowPoints/> <br/>
            <div class={classes!("float-left", "w-1/6", "grid-cols-1", "grid-rows-6", "h-1/2")} >
                <p> {"Actions (Move the Board with WASD):"} </p>
                // {statics.collect::<Html>()}
                {statics.collect::<Vec<_>>()}

                <br/>
                <Statistics/>
            </div>
            <div class={classes!("float-right", "w-1/6", "grid-cols-1", "grid-rows-6", "h-1/2")} >
                <p> {"Upgrades"} </p>
                {upgrades.collect::<Vec<_>>()}

                <p> {"Sliders"} <ShowSliderPoints/> </p>
                 {sliders}

            </div>
        </div>
    }
}
