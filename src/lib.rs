#![allow(incomplete_features)]
#![feature(default_free_fn)]

use std::{
    default::default,
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::{
    gm::Size,
    tools::Boxed,
    ui::{input::touch::Event, Touch},
    Screen,
};

use crate::controls_view::ControlsView;

mod controls_view;
mod game_level;

static mut SCREEN: *mut Screen = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        SCREEN = Box::into_raw(Box::new(
            Screen::new(default())
                .add_view(ControlsView::boxed())
                .add_debug_view(),
        ));
    }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_float, height: c_float) {
    unsafe {
        SCREEN
            .as_mut()
            .unwrap_unchecked()
            .set_size(Size { width, height });
    }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe {
        SCREEN.as_mut().unwrap_unchecked().update();
    }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    #[allow(clippy::useless_conversion)]
    unsafe {
        SCREEN.as_mut().unwrap_unchecked().on_touch(Touch {
            id:       id.into(),
            position: (x * 2.0, y * 2.0).into(),
            event:    Event::from_int(event),
        })
    }
}
