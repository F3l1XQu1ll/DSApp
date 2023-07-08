#![warn(clippy::all, rust_2018_idioms)]

mod app;
#[macro_use]
mod macros;
mod character_talents;
mod cost;
mod data;
mod debugger;
mod display;
mod erfahrungsgrade;
mod liturgy;
mod math;
mod profession;
mod spells;
mod spells_index;
mod spezies;
mod talent;
mod talents_view;
mod vorteile;
mod widgets;
pub use app::DSApp;
