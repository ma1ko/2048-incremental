use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::model::Msg;
use crate::model::UpgradeableBoard;

use crate::upgrade::*;

// #[derive(PartialEq, Eq, Clone, Properties)]
// pub struct Button {
//     pub visible: bool,
//     pub show_points: usize,
//     pub clickable_points: usize,
//     pub msg: &'static Msg,
//     pub text: &'static str,
// }

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub points: usize,
    pub upgrade: Upgrade,
}

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
        let board = Dispatch::<UpgradeableBoard>::new();
        let points = Dispatch::<Points>::new();
        let f = board.reduce_callback(move |board| {
            points.reduce(|points| points.sub(upgrade.cost));
            // board.upgrade(action);
            upgrade.f.emit(());
            board
        });

        let text = format!("{} (Cost: {})", upgrade.text, upgrade.cost);

        if self.visible() && self.clickable() {
            html! { <button class={style} onclick={f}>{text}</button> }
        } else if self.visible() {
            html! { <button class={style}> {text}</button> }
        } else {
            html! {}
        }
    }
}
