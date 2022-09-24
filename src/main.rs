
mod upgrades;
mod condition;
mod points;
mod maze;
mod sidebar;
mod twentyfourtyeight;
mod upgrade_button;
mod upgrade;
mod stats;
mod number;
mod autoaction;
mod slider;

mod model;

use serde::{Serialize, Deserialize};
use yew::prelude::*;
use std::cell::{Cell,RefCell};
use std::rc::Rc;
use yewdux::prelude::*;
use yewdux::storage;
use std::fmt::Display;
use std::marker::PhantomData;
use crate::stats::*;
use crate::model::*;
use crate::upgrade::*;
use crate::number::*;
use crate::points::*;
use crate::upgrade_button::*;
use crate::condition::*;
use crate::autoaction::*;
use crate::slider::*;
use upgrades::*;

use log::info;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<model::Model>();
    log::info!("starting");
    // twentyfourtyeight::main();
}
