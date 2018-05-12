
#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;

extern crate myothercrate;

#[cfg_attr(feature="flame_it", flame)]
pub fn caller() {
    println!("Calling hello world");
    myothercrate::hello_world_from_other();
}
