use gloo::events::EventListener;

use crate::sidebar::SideBar;

// use twentyfourtyeight::Field;
fn color(value: usize) -> &'static str {
    match value {
        0 => "bg-blue-50",
        1 => "bg-blue-100",
        2 => "bg-blue-400",
        3 => "bg-blue-700",
        4 => "bg-red-100",
        5 => "bg-red-400",
        6 => "bg-red-700",
        7 => "bg-green-100",
        8 => "bg-green-400",
        9 => "bg-green-700",
        10 => "bg-black-100",
        11 => "bg-black-400",
        12 => "bg-black-700",
        x => unimplemented!("Color for {}", x),
    }
}

#[function_component(YewField)]
fn field(props: &Props) -> html {
    let (board, _) = use_store::<UpgradeableBoard>();
    let scientific_notation = board.scientific_notation.get();
    let board = board.board.borrow();
    let value = board.board.get(&props.index).unwrap().value;
    let classes = classes!(
        "text-red-900",
        "text-center",
        // "p-20",
        "justify-center",
        "flex",
        "items-center",
        "text-5xl",
        color(value.unwrap_or(1))
    );
    let value = if let Some(value) = value {
        if scientific_notation && value <= 1024 {
            format!("2e{}", value)
        } else {
            (1 << value).to_string()
        }
    } else {
        " ".to_string()
    };

    // classes.push("a");
    html! {
        <div class={classes}>{value}</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    index: Point,
}

use crate::upgrade::*;
#[derive(PartialEq, Eq)]
pub enum Msg {
    Key(KeyboardEvent),
    Move(),
    Board(Rc<UpgradeableBoard>),
}

use crate::maze::*;
use crate::twentyfourtyeight::Field;
use crate::*;

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
        let max = Point::new(4, 4);
        let mut board: Board<Field> = Board::new(max);
        for i in 0..max.x {
            for j in 0..max.y {
                board.insert(Point::new(i, j), Field::new(None));
            }
        }
        board.insert(Point::new(0, 0), Field::new(Some(1)));
        Self {
            board: RefCell::new(board),
            scientific_notation: Cell::new(false),
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
#[derive(Deserialize, Serialize)]
pub struct UpgradeableBoard {
    board: RefCell<Board<Field>>,
    scientific_notation: Cell<bool>,
}

impl UpgradeableBoard {
    pub fn scientific_notation(&self) {
        self.scientific_notation.set(true);
    }
    pub fn extend_x(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.y {
            let point = Point::new(board.max.x, i);
            board.insert(point, Field::new(None));
        }
        board.max.x += 1;
    }

    pub fn extend_y(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.x {
            let point = Point::new(i, board.max.y);
            board.insert(point, Field::new(None));
        }
        board.max.y += 1;
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
            let dispatch = Dispatch::<Points>::new();
            dispatch.reduce(|points| points.add(1 << value));

            let dispatch = Dispatch::<Stats>::new();
            dispatch.reduce_mut(|points| points.harvest(value));
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
        let mut board = self.board.borrow_mut();
        let field = Field::new(Some(number));
        board.random_empty_replace(field);
    }
}

#[function_component(Main)]
fn ui() -> html {
    html! {}
}

pub struct Model {
    board: Dispatch<UpgradeableBoard>,
    _link: yew::html::Scope<Self>,
    _listener: EventListener,
}

fn handle_keypress() {}

impl Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // initiates saving state
        let save = Dispatch::<crate::upgrade::AutoActions>::new();
        save.reduce(|action| action);

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
                    _ => Direction::Nowhere,
                };
                self.board.reduce(|board| {
                    board.play(direction);
                    board
                });
            }

            Msg::Move() => {
                self.board.reduce(|board| {
                    board.mv();
                    board
                });
            }
            Msg::Board(_board) => { /* Board has changed, redraw it */ }
        };
        true
    }

    /* Grid classes for tailwindcss
     * grid-cols-4 grid-cols-5 grid-cols-6 grid-cols-7 grid-cols-8 grid-cols-9
     * grid-rows-4 grid-rows-5 grid-rows-6 grid-rows-7 grid-rows-8 grid-rows-9
     */

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let board = self.board.get();
        let board = board.board.borrow();
        let html: Html = board
            .points()
            .map(|index| {
                html! {<YewField {index}/>}
            })
            .collect();

        let cols = format!("grid-cols-{}", board.max.x);
        let rows = format!("grid-rows-{}", board.max.y);
        let grid_class = classes!(
            "bg-black",
            "float-left",
            "grid",
            "gap-2",
            "h-screen",
            "w-4/6",
            cols,
            rows
        );
        html! {
            <body class={classes!("float-root", "h-full")}>

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
