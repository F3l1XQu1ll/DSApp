#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod kampf;
#[macro_use]
mod macros;
mod apmods;
mod character_talents;
mod cost;
mod data;
mod debugger;
mod display;
mod erfahrungsgrade;
mod math;
mod spells;
mod spells_index;
mod spezies;
mod sprachen_schriften;
mod talent;
mod talents_view;
// mod vorteile;
mod widgets;
pub use app::DSApp;
mod roll;
