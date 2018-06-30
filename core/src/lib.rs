#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod rom;
pub mod mem;
pub mod cpu;
pub mod gpu;
pub mod input;
pub mod sound;

mod io;
