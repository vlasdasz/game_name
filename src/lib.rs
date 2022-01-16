#![allow(incomplete_features)]
#![feature(result_option_inspect)]

use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::{
    gm::Size,
    tools::Boxed,
    ui::{input::touch::Event, Touch},
    Screen,
};
use tokio::runtime::Runtime;

use crate::controls_view::ControlsView;

mod controls_view;
mod game_level;

static mut SCREEN: *mut Screen = ptr::null_mut();
static mut RUNTIME: Option<Runtime> = None;

#[no_mangle]
pub unsafe extern "C" fn create_screen() {
    RUNTIME = Some(tokio::runtime::Runtime::new().unwrap());

    RUNTIME.as_ref().unwrap().block_on(async {
        let mut screen = Box::new(Screen::new(Default::default()));
        screen.ui.set_view(ControlsView::boxed());
        screen.ui.add_debug_view();

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
