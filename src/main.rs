// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![allow(
    clippy::match_bool,
    clippy::iter_nth_zero,
    clippy::module_name_repetitions
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod adapters;
mod consts;
mod error;
mod message;
mod ports;

use crate::{
    consts::msg,
    ports::ui::{AppBuilder, NamedWindowDimensions},
};
use adapters::ui::MandelBuilder;
use error::Error;
use ports::ui::{App, WindowSettings};
use std::env;

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let args = env::args_os()
        .map(|oss| oss.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    println!("args: {:?}", args);

    MandelBuilder::new()
        .set_window_state(WindowSettings::Resizable(
            NamedWindowDimensions::QuarterScreen.into(),
        ))
        .set_title(msg::WELCOME_TO_MANDEL)
        .build()?
        .run()
}
