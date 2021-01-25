#![no_std]
#![feature(stmt_expr_attributes)]

#[macro_use]
#[cfg(feature = "heap_alloc")]
extern crate alloc;

#[macro_use]
#[cfg(feature = "logging")]
extern crate log;

#[macro_use]
#[cfg(feature = "serialisation")]
extern crate serde;

pub mod cpu;
pub mod gpu;
pub mod input;
pub mod mem;
pub mod rom;
pub mod sound;

mod io;
