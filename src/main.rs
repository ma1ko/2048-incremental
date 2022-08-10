use yew::prelude::*;

mod maze;
mod twentyfourtyeight;

use std::collections::HashMap;
use twentyfourtyeight::Field;

fn color(value: usize) -> &'static str {
    match value {
        0 => "bg-blue-50",
        2 => "bg-blue-100",
        4 => "bg-blue-400",
        8 => "bg-blue-700",
        16 => "bg-red-100",
        32 => "bg-red-400",
        64 => "bg-red-700",
        128 => "bg-green-100",
        256 => "bg-green-400",
        512 => "bg-green-700",
        _ => "",
    }
}

struct YewField {
    value: Option<usize>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    value: Option<usize>,
}
impl Component for YewField {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: ctx.props().value,
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let value = if let Some(value) = self.value {
            value.to_string()
        } else {
            "".to_string()
        };
        let mut classes = classes!(
            "text-red-900",
            "text-center",
            "p-20",
            "text-5xl",
            color(self.value.unwrap_or(0))
        );
        classes.push("a");
        html! {
            <div class={classes}>{value}</div>
        }
    }
    // fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    //     true
    // }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().value != self.value {
            self.value = ctx.props().value;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Msg {
    Key(KeyboardEvent),
    Click(),
}

use maze::Board;
use maze::Direction;
use maze::Point;
struct Model {
    board: Board<Field>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let max = Point(4, 4);
        let mut board: Board<Field> = Board::new(max);
        for i in 0..max.0 {
            for j in 0..max.1 {
                board.insert(Point(i, j), Field::new(None));
            }
        }
        board.insert(Point(0, 0), Field::new(Some(2)));

        Self { board }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Key(key) => match key.key_code() {
                37 => self.board.play(Direction::Left),
                38 => self.board.play(Direction::Up),
                39 => self.board.play(Direction::Right),
                40 => self.board.play(Direction::Down),
                _ => false,
            },
            Msg::Click() => {false}
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let html: Html = self
            .board
            .rows()
            .map(|row| {
                row.map(|field| {
                    html! {<YewField value={field.value}/>}
                })
            })
            .flatten()
            .collect();
 

        html! {
            <div class={classes!("grid", "grid-cols-4", "gap-2", "h-screen")}
                onkeydown={link.callback(|key| Msg::Key(key))} tabindex=0 >
                {html}
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    log::info!("starting");
}
