#![allow(incomplete_features)]
#![allow(clippy::missing_safety_doc)]
#![feature(result_option_inspect)]

use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::{
    gl_wrapper::monitor::Monitor,
    gm::Size,
    rtools::Boxed,
    ui::{input::touch::Event, Touch},
    Screen,
};
use tokio::runtime::Runtime;

use crate::controls_view::ControlsView;

mod controls_view;
mod game_level;

static mut SCREEN: *mut Screen = ptr::null_mut();
static mut MONITOR: *mut Monitor = ptr::null_mut();

static mut RUNTIME: Option<Runtime> = None;

#[no_mangle]
pub unsafe extern "C" fn create_screen(width: c_int, height: c_int) {
    RUNTIME = Some(tokio::runtime::Runtime::new().unwrap());

    RUNTIME.as_ref().unwrap().block_on(async {
        let mut screen = Box::new(Screen::new((width, height).into()));

        screen.ui.set_view(ControlsView::boxed());
        screen.ui.add_debug_view();

        screen.add_monitor(MONITOR.as_ref().unwrap().clone());

        SCREEN = Box::into_raw(screen);
    });
}

#[no_mangle]
pub unsafe extern "C" fn set_screen_size(width: c_float, height: c_float) {
    RUNTIME.as_ref().unwrap().block_on(async {
        SCREEN
            .as_mut()
            .unwrap_unchecked()
            .set_size(Size { width, height });
    });
}

#[no_mangle]
pub unsafe extern "C" fn update_screen() {
    RUNTIME.as_ref().unwrap().block_on(async {
        SCREEN.as_mut().unwrap_unchecked().update();
    });
}

#[no_mangle]
pub unsafe extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    #[allow(clippy::useless_conversion)]
    RUNTIME.as_ref().unwrap().block_on(async {
        SCREEN.as_mut().unwrap_unchecked().ui.on_touch(Touch {
            id:       id.into(),
            position: (x * 2.0, y * 2.0).into(),
            event:    Event::from_int(event),
        })
    });
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
    let monitor = Monitor::new(
        "Phone screen".into(),
        ppi as _,
        scale,
        refresh_rate as _,
        (resolution_x, resolution_y).into(),
        (width, height).into(),
        diagonal as _,
    );

    dbg!(&monitor);

    unsafe {
        MONITOR = Box::into_raw(Box::new(monitor));
    }
}
