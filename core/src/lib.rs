#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serialize")]
extern crate bincode;

pub mod rom;
pub mod mem;
pub mod cpu;
pub mod gpu;
pub mod input;
pub mod sound;

mod io;
