use gloo::events::EventListener;
use rand::prelude::SliceRandom;

use crate::sidebar::SideBar;

#[function_component(DisplayBoard)]
fn display_board() -> Html {
    let (board, _) = use_store::<UpgradeableBoard>();
    // let (x, _) = use_store::<Slider<ExtendX>>();
    let (_, sliders) = use_store::<Sliders>();
    let sliders = sliders.get();
    let x = sliders[&ExtendX].borrow();
    let y = sliders[&ExtendY].borrow();

    // let (y, _) = use_store::<Slider<ExtendY>>();
    board.set_size(x.current + 4, y.current + 4);

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
            <div class={grid_class}>
                {html}
            </div>
    }
}

#[function_component(YewField)]
fn field(props: &Props) -> Html {
    let (board, _) = use_store::<UpgradeableBoard>();
    let board = board.board.borrow();
    let number = board.board.get(&props.index).unwrap();
    let classes = classes!(
        "text-red-900",
        "text-center",
        // "p-20",
        "justify-center",
        "flex",
        "items-center",
        "text-5xl",
        number.color()
    );
    html! {
        <div class={classes}>{number}</div>
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
    Board(Rc<UpgradeableBoard>),
}

use crate::maze::*;
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
        let mut board: Board<Number> = Board::new(max);
        for i in 0..max.x {
            for j in 0..max.y {
                board.insert(Point::new(i, j), Number::none());
            }
        }
        board.insert(Point::new(0, 0), Number::new(2));
        Self {
            board: RefCell::new(board),
            scientific_notation: Cell::new(false),
            points: Cell::new(2),
            combine_fn: Cell::new(CombineFn::Standard),
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
    board: RefCell<Board<Number>>,
    pub scientific_notation: Cell<bool>,
    pub points: Cell<usize>,
    combine_fn: Cell<CombineFn>,
}

impl UpgradeableBoard {
    pub fn shuffle(&self) {
        let mut board = self.board.borrow_mut();
        let mut values = board
            .iter_mut()
            .map(|value| value.value.take())
            .collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        values.shuffle(&mut rng);
        values
            .into_iter()
            .zip(board.iter_mut())
            .for_each(|(value, board)| {
                board.set(value);
            });
    }
    fn calc_points(&self) {
        let points = self.board.borrow().iter().map(|field| field.value()).sum();
        self.points.set(points);
    }
    pub fn boardsize(&self) -> Point {
        self.board.borrow().max
    }
    pub fn get_points(&self) -> usize {
        self.points.get()
    }
    pub fn set_combine_fn(&self, f: CombineFn) {
        self.combine_fn.set(f);
    }
    pub fn scientific_notation(&self) {
        self.scientific_notation.set(true);
    }
    pub fn set_size(&self, bx: usize, by: usize) {
        let Point { mut x, mut y } = self.boardsize();
        while x > bx {
            x -= 1;
            self.reduce_x();
        }
        while y > by {
            y -= 1;
            self.reduce_y();
        }
        while x < bx {
            x += 1;
            self.extend_x();
        }
        while y < by {
            y += 1;
            self.extend_y();
        }
    }
    pub fn extend_x(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.y {
            let point = Point::new(board.max.x, i);
            board.insert(point, Number::none())
        }
        board.max.x += 1;
    }
    pub fn reduce_x(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.y {
            let point = Point::new(board.max.x - 1, i);
            board.board.remove(&point);
        }
        board.max.x -= 1;
    }

    pub fn extend_y(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.x {
            let point = Point::new(i, board.max.y);
            board.insert(point, Number::none())
        }
        board.max.y += 1;
    }
    pub fn reduce_y(&self) {
        let board = &mut self.board.borrow_mut();
        for i in 0..board.max.x {
            let point = Point::new(i, board.max.y - 1);
            board.board.remove(&point);
        }
        board.max.y -= 1;
    }
    pub fn harvest_number(&self, num: usize) -> bool {
        let number = Number::new(num);
        let found = {
            let board = &mut self.board.borrow_mut();

            board.iter_mut().any(|field| {
                if *field == number {
                    field.set(Some(0));
                    true
                } else {
                    false
                }
            })
        };
        if found {
            self.harvest_stats(num);
            true
        } else {
            false
        }
    }
    fn harvest_stats(&self, value: usize) {
        // Add points
        let dispatch = Dispatch::<Points>::new();
        dispatch.reduce_mut(|points| points.add(value));

        // Update stats
        let dispatch = Dispatch::<Stats>::new();
        dispatch.reduce_mut(|stats| stats.harvest(value));
        // Need to exclude that from stats too
        let dispatch = Dispatch::<Avg>::new();
        dispatch.reduce_mut(|avg| avg.harvested(value));
        // self.points.set(self.points.get() - value);
        self.calc_points();
    }
    pub fn harvest(&self) {
        let value = {
            let board = &mut self.board.borrow_mut();
            let max = board
                .board
                .values_mut()
                // .iter_mut()
                .max_by(|f1, f2| f1.value().cmp(&f2.value()));
            if let Some(f) = max {
                let value = f.value();
                f.set(None);
                value
            } else {
                return;
            }
        };
        self.harvest_stats(value);
    }
    pub fn mv(&self) {
        self
            .board
            .borrow_mut()
            .play_random(self.combine_fn.get().into());
        // self.points.set(self.points.get() + points);
        self.random_place(4);
    }
    fn play(&self, direction: Direction) {
        self
            .board
            .borrow_mut()
            .play(direction, self.combine_fn.get().into());
        let points = self.random_place(4);
        Dispatch::<Avg>::new().reduce_mut(|avg| {
            avg.manually_added(points);
        });
    }
    pub fn random_place(&self, number: usize) -> usize {
        let mut board = self.board.borrow_mut();
        let field = number.into();
        let value = board.random_empty_replace(field);
        self.points.set(self.points.get() + value);
        value

    }
    pub fn contains(&self, number: usize) -> bool {
        let board = self.board.borrow();
        board.iter().any(|field| *field == Number::new(number))
    }
}

#[function_component(Main)]
fn ui() -> Html {
    html! {}
}

pub struct Model {
    board: Dispatch<UpgradeableBoard>,
    _link: yew::html::Scope<Self>,
    _listener: EventListener,
}

// fn handle_keypress() {}

impl Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // register_upgrades();
        // initiates saving state
        // let save = Dispatch::<AutoActions>::new();
        // save.reduce(|action| action);

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
            Msg::Board(_board) => { /* Board has changed, redraw it */ }
        };
        true
    }

    /* Grid classes for tailwindcss
     * grid-cols-4 grid-cols-5 grid-cols-6 grid-cols-7 grid-cols-8 grid-cols-9 grid-cols-10
     * grid-rows-4 grid-rows-5 grid-rows-6 grid-rows-7 grid-rows-8 grid-rows-9 grid-rows-10
     */

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        // let board = self.board.get();
        // let board = board.board.borrow();

        html! {
            <body class={classes!("float-root", "h-full")}>

            <DisplayBoard/>

            <SideBar/>

            </body>
        }
    }
}

