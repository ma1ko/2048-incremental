
mod maze;
mod sidebar;
mod twentyfourtyeight;
mod upgrade_button;
mod upgrade;
mod stats;
mod number;

mod model;

use serde::{Serialize, Deserialize};
use yew::prelude::*;
use std::cell::{Cell,RefCell};
use std::rc::Rc;
use yewdux::prelude::*;
use yewdux::storage;
use crate::stats::*;
use crate::model::*;
use crate::upgrade::*;
use crate::number::*;

use log::info;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<model::Model>();
    log::info!("starting");
    // twentyfourtyeight::main();
}
