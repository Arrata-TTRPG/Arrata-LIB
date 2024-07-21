#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
//! Utilities for Rust-based Arrata applications.

pub mod character;
pub use character::*;
pub mod dice;
pub use dice::*;
pub mod obstacle;
pub use obstacle::Obstacle;
pub mod quirk;
pub use quirk::*;
