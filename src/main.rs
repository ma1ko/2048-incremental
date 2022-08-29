
mod maze;
mod sidebar;
mod twentyfourtyeight;
mod upgrade_button;
mod upgrade;

mod model;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<model::Model>();
    log::info!("starting");
    // twentyfourtyeight::main();
}
