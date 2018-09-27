#![feature(custom_attribute)]

#[macro_use]
extern crate stdweb;
extern crate chess_core;

use chess_core::{Figure};

#[js_export]
pub fn webasm(values: &[Figure], index: usize) -> Figure {
    values[index]
}