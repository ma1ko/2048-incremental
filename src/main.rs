
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
use yewdux::mrc::Mrc;
use yewdux::storage;
use std::fmt::Display;
use std::marker::PhantomData;
use std::collections::HashMap;


use crate::stats::*;
use crate::model::*;
use crate::upgrade::*;
use crate::number::*;
use crate::points::*;
use crate::upgrade_button::*;
use crate::condition::*;
use crate::autoaction::*;
use crate::slider::*;

use log::info;
use yew::Renderer;
use std::ops::Index;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<model::Model>::new().render();
    // yew::start_app::<model::Model>();
}
