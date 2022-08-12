use yew::prelude::*;

mod maze;
mod sidebar;
mod twentyfourtyeight;
use sidebar::SideBar;

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
        let classes = classes!(
            "text-red-900",
            "text-center",
            // "p-20",
            "justify-center",
            "flex",
            "items-center",
            "text-5xl",
            color(self.value.unwrap_or(0))
        );
        // classes.push("a");
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
    ExtendY(),
    ExtendX(),
}

use maze::Board;
use maze::Direction;
use maze::Point;
struct Model {
    board: Board<Field>,
}

impl Model {
    fn extend_x(&mut self) {
        let board = &mut self.board.board;
        let max = &mut self.board.max;
        for i in 0..max.1 {
            let point = Point(max.0, i);
            board.insert(point, Field::new(None));
        }
        max.0 += 1;
    }

    fn extend_y(&mut self) {
        let board = &mut self.board.board;
        let max = &mut self.board.max;
        for i in 0..max.0 {
            let point = Point(i, max.1);
            board.insert(point, Field::new(None));
        }
        max.1 += 1;
    }
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
            Msg::Key(key) => {
                let _ = match key.key_code() {
                    37 => self.board.play(Direction::Left),
                    38 => self.board.play(Direction::Up),
                    39 => self.board.play(Direction::Right),
                    40 => self.board.play(Direction::Down),
                    _ => false,
                };
            }
            Msg::ExtendX() => self.extend_x(),
            Msg::ExtendY() => self.extend_y(),
        };
        true
    }

    /* Grid classes for tailwindcss
     * grid-cols-4 grid-cols-5 grid-cols-6 grid-cols-7 grid-cols-8 grid-cols-9
     */

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

        let link = link.clone();
        let extend_x = link.callback(|_| Msg::ExtendX());
        let extend_y = link.callback(|_| Msg::ExtendY());
        // let extend_y = Callback::from(|_| self.extend_y());

        // let body = gloo_utils::body();
        // body.add_event_listener_with_callback("keydown", link.callback(|key| Msg::Key(key)));
        let cols = format!("grid-cols-{}", self.board.max.0);
        let grid_class = classes!("float-left", "grid", cols, "gap-2", "h-screen", "w-4/6");
        html! {
            <body class={classes!("float-root", "h-full")}
                onkeydown={link.callback(|key| Msg::Key(key))} tabindex=0 >

            <div class={grid_class}>
                {html}
            </div>
            <SideBar points={0} extend_x={extend_x} extend_y={extend_y}/>

            </body>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    log::info!("starting");
    // twentyfourtyeight::main();
}
