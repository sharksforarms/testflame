#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;

#[cfg_attr(feature="flame_it", flame)]
pub fn hello_world_from_other() {
    println!("Hello world");
}

