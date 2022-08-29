use yew::prelude::*;
use yewdux::prelude::Dispatch;
use yewdux::prelude::use_store;


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
        let f = Callback::once( move |_| { 
            upgrade.run()
        });
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

/*

impl UpgradeButton {
    fn visible(&self) -> bool {
        self.props.upgrade.visible(self.props.points)
    }
    fn clickable(&self) -> bool {
        self.props.upgrade.clickable(self.props.points)
    }
}
pub struct UpgradeButton {
    props: Props,
    // parent_link: yew::html::Scope<SideBar>,
}
impl Component for UpgradeButton {
    type Message = ();
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            // parent_link,
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.points = ctx.props().points;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
        let upgrade = self.props.upgrade.clone();
        let f = Callback::once( move |_| { 
            upgrade.run()
        });
        let upgrade = self.props.upgrade.clone();

        let text = format!("{} (Cost: {})", upgrade.text, upgrade.cost.get());

        if self.visible() && self.clickable() {
            html! { <button class={style} onclick={f}>{text}</button> }
        } else if self.visible() {
            html! { <button class={style}> {text}</button> }
        } else {
            html! {}
        }
    }
}
*/
