
#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;
extern crate testflame;

use std::fs::File;

#[cfg_attr(feature="flame_it", flame)]
fn start() {
    testflame::caller();
}
fn main() {
    start();
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
