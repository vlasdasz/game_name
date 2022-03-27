#![allow(incomplete_features)]
#![allow(clippy::missing_safety_doc)]
#![feature(result_option_inspect)]

extern crate core;

use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::app::App;

use crate::controls_view::ControlsView;

mod controls_view;
mod game_level;

static mut APP: *mut App<ControlsView> = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen(width: c_int, height: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().create_screen(width, height) }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_int, height: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().set_screen_size(width, height) }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe { APP.as_mut().unwrap_unchecked().update_screen() }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().on_touch(id, x, y, event) }
}

#[no_mangle]
pub extern "C" fn set_monitor(
    ppi: c_int,
    scale: c_float,
    refresh_rate: c_int,
    resolution_x: c_int,
    resolution_y: c_int,
    width: c_float,
    height: c_float,
    diagonal: c_float,
) {
    unsafe {
        APP = Box::into_raw(Box::new(App::<ControlsView>::default()));
        APP.as_mut().unwrap_unchecked().set_monitor(
            ppi,
            scale,
            refresh_rate,
            resolution_x,
            resolution_y,
            width,
            height,
            diagonal,
        );
    }
}