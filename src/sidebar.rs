use yew::prelude::*;
use yewdux::prelude::*;

use crate::model::Model;
// use crate::upgrade_button::Button;
use crate::upgrade::*;
use std::rc::Rc;

pub enum Msg {
    UpdatePoints(Rc<Points>),
    UpdateUpgrades(Rc<Upgrades>),
}
pub struct SideBar {
    points: Dispatch<Points>,
    upgrades: Dispatch<Upgrades>,
}

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

// #[function_component(Bar)]
// pub fn bar() -> html {}

impl Component for SideBar {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let _parent = ctx.link().get_parent().unwrap();
        let callback = ctx.link().callback(Msg::UpdatePoints);
        let points = Dispatch::<Points>::subscribe(callback);
        let callback = ctx.link().callback(Msg::UpdateUpgrades);
        let upgrades = Dispatch::<Upgrades>::subscribe(callback);
        Self { points, upgrades }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePoints(_points) => {
                // log::info!("Got {:?}", points);
            }
            Msg::UpdateUpgrades(_upgrades) => {
                // log::info!("Got new upgrades");
            }
        }
        false 
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let upgrades = self.upgrades.get();
        let points = self.points.get();
        let buttons: Html = upgrades
            .upgrades
            .iter()
            .map(|upgrade| {
                html! {
                    <> <UpgradeButton {upgrade}/> <br/> </>
                }
            })
            .collect();

        html! {
            <div class={classes!("float-right", "w-2/6")} >
            <ShowPoints/>
            {buttons}
            </div>
        }
    }
}
