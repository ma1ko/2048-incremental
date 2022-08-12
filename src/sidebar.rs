
use yew::prelude::*;


#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub points: usize,
    pub extend_x: Callback<()>,
    pub extend_y: Callback<()>,

}
pub struct SideBar {
    props: Props

}


impl Component for SideBar {
    type Message = ();
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {props: ctx.props().clone()}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let button = classes!("bg-white","text-gray-800", "font-semibold", "py-2" ,"px-4", "border", "border-gray-400", "rounded", "shadow");
        let x = self.props.extend_x.clone();
        let y = self.props.extend_y.clone();

        html! {
            <div class={classes!("float-right", "w-2/6")} > 
            <button class={button.clone()} onclick={move |_| x.emit(())}>{"Extend x"}</button>
            <button class={button} onclick={move |_| y.emit(())}>{"Extend y"}</button>
            </div>
        }
    }


    
}
