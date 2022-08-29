use gloo::events::EventListener;
use yew::prelude::*;
use yewdux::prelude::Dispatch;
use yewdux::prelude::Store;

use crate::sidebar::SideBar;

use gloo::timers::callback::Interval;
// use twentyfourtyeight::Field;
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
use crate::upgrade::*;
#[derive(PartialEq, Eq)]
pub enum Msg {
    Key(KeyboardEvent),
    // Upgrade(&'static str),
    // ExtendY(),
    // ExtendX(),
    // Harvest(),
    // AutoClicker(),
    Move(),
    Board(Rc<UpgradeableBoard>),
}

use crate::maze::*;
use crate::twentyfourtyeight::Field;
use crate::upgrade::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Store for UpgradeableBoard {
    fn new() -> Self {
    yewdux::storage::load(yewdux::storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
          }
    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}


impl Default for UpgradeableBoard {
    fn default() -> Self {
          let max = Point(4, 4);
        let mut board: Board<Field> = Board::new(max);
        for i in 0..max.0 {
            for j in 0..max.1 {
                board.insert(Point(i, j), Field::new(None));
            }
        }
        board.insert(Point(0, 0), Field::new(Some(2)));
        Self {
            board: RefCell::new(board),
            // clicker: RefCell::new(None),
        }

    }

}


impl PartialEq for UpgradeableBoard {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}
impl Eq for UpgradeableBoard {}
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct UpgradeableBoard {
    board: RefCell<Board<Field>>,
}

impl UpgradeableBoard {
    pub fn extend_x(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.1 {
            let point = Point(board.max.0, i);
            board.insert(point, Field::new(None));
        }
        board.max.0 += 1;
    }

    pub fn extend_y(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.0 {
            let point = Point(i, board.max.1);
            board.insert(point, Field::new(None));
        }
        board.max.1 += 1;
    }
    pub fn harvest(&self) {
        let board = &mut self.board.borrow_mut();
        let max = board
            .board
            .iter_mut()
            .max_by(|(_, f1), (_, f2)| f1.value.cmp(&f2.value));
        if let Some((_, f)) = max {
            // let f: Field = self.board.board.remove(&p).unwrap();
            let value = f.value.unwrap_or(0);
            log::info!("Found {}", value);
            // self.points += f.value.unwrap_or(0);
            let dispatch = Dispatch::<Points>::new();
            dispatch.reduce(|points| points.add(value) );

            f.value = None;
        }
    }
    pub fn mv(&self) {
        self.board.borrow_mut().play_random();
    }
    fn play(&self, direction: Direction) {
        self.board.borrow_mut().play(direction);
    }
    pub fn random_place(&self, number: usize) {
        log::info!("Random placing");
        let mut board = self.board.borrow_mut();
        let field = Field::new(Some(number));
        board.random_empty_replace(field);



    }

}
pub struct Model {
    board: Dispatch<UpgradeableBoard>,
    _link: yew::html::Scope<Self>,
    _listener: EventListener,
}

impl Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // initiates saving state
        let save = Dispatch::<crate::upgrade::AutoActions>::new();
        save.reduce(|action| {
            action

        });

        let window: web_sys::EventTarget = web_sys::window().unwrap().into();
        let cb = ctx.link().callback(|key| Msg::Key(key));
        let x = EventListener::new(&window, "keydown", move |event| {
            use wasm_bindgen::JsCast;
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            cb.emit(event.clone());
        });
        let board = Dispatch::<UpgradeableBoard>::subscribe(ctx.link().callback(Msg::Board));

        Self {
            board,
            _link: ctx.link().clone(),
            _listener: x,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Key(key) => {
                let direction = match key.key_code() {
                    37 | 65 => Direction::Left,
                    38 | 87 => Direction::Up,
                    39 | 68 => Direction::Right,
                    40 | 83 => Direction::Down,
                    _ => {
                        /*log::info!("Code {}", x);*/
                        Direction::Nowhere
                    }
                };
                // self.board.get().play(direction);

                self.board.reduce(|board| {
                    board.play(direction);
                    board
                });
            }
            //Msg::Upgrade(name) => {
            //    // self.upgrade(name);
            //    //TODO?
            //}

            // Msg::ExtendX() => self.extend_x(),
            // Msg::ExtendY() => self.extend_y(),
            // Msg::Harvest() => self.harvest(),
            // Msg::AutoClicker() => self.autoclicker(),
            Msg::Move() => {
                self.board.reduce(|board| {
                    board.mv();
                    board
                });
            }
            Msg::Board(_board) => { /* Board has changed, redraw it */} 
        };
        true
    }

    /* Grid classes for tailwindcss
     * grid-cols-4 grid-cols-5 grid-cols-6 grid-cols-7 grid-cols-8 grid-cols-9
     */

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let board = self.board.get();
        let html: Html = board
            .board
            .borrow()
            .rows()
            .map(|row| {
                row.map(|field| {
                    html! {<YewField value={field.value}/>}
                })
            })
            .flatten()
            .collect();


        // let body = gloo_utils::body();
        // body.add_event_listener_with_callback("keydown", link.callback(|key| Msg::Key(key)));
        let cols = format!("grid-cols-{}", board.board.borrow().max.0);
        let grid_class = classes!("float-left", "grid", cols, "gap-2", "h-screen", "w-4/6");
        html! {
            <body class={classes!("float-root", "h-full")}>
                // onkeydown={link.callback(|key| Msg::Key(key))} tabindex=0 >

            <div class={grid_class}>
                {html}
            </div>
            <SideBar/>

            </body>
        }
    }
}

// fn main() {
//     wasm_logger::init(wasm_logger::Config::default());
//     yew::start_app::<Model>();
//     log::info!("starting");
//     // twentyfourtyeight::main();
// }
